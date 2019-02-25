#[macro_use] extern crate serenity;
#[macro_use] extern crate mysql;
extern crate dotenv;
extern crate time;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use std::env;

use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct DiscordUser {
    id_discord: i32,
    date_registered: <time::Tm as 'struct-unit'>::TmFmt,
    group_type: Option<String>,
}

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Read .env file
    dotenv::dotenv().expect("Failed to read .env file");

    // See docs on the `OptsBuilder`'s methods for the list of options available via URL.
    let pool = my::Pool::new("mysql://user1@localhost:3306/test").unwrap();

    //
    // Start MYSQL EXAMPLE
    //
    // Let's create payment table.
    // Unwrap just to make sure no error happened.
    pool.prep_exec(r"CREATE TEMPORARY TABLE discord_users (
                         id_discord int not null,
                         date_registered text,
                         group_type text
                     )", ()).unwrap();
    
    let discord_users_vec = vec![
        DiscordUser { id_discord: 100099, date_registered: mysql::time::now_utc().rfc822(), group_type: Some("Owner".into()) },
        DiscordUser { id_discord: 100119, date_registered: mysql::time::now_utc().rfc822(), group_type: None },
    ];

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"INSERT INTO discord_users
                                       (id_discord, date_registered, group_type)
                                   VALUES
                                       (:id_discord, :date_registered, :group_type)").into_iter() {
        for d in discord_users_vec.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
            stmt.execute(params!{
                "id_discord" => d.id_discord,
                "date_registered" => d.date_registered,
                "group_type" => &d.group_type,
            }).unwrap();
        }
    }

    // Let's select payments from database
    let selected_users: Vec<DiscordUser> =
    pool.prep_exec("SELECT id_discord, date_registered, group_type from discord_users", ())
    .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
        // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
        // will map each `MyResult` to contained `row` (no proper error handling)
        // and second call to `map` will map each `row` to `Payment`
        result.map(|x| x.unwrap()).map(|row| {
            // ⚠️ Note that from_row will panic if you don't follow your schema
            let (id_discord, date_registered, group_type) = my::from_row(row);
            DiscordUser {
                id_discord: id_discord,
                date_registered: date_registered,
                group_type: group_type,
            }
        }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
    }).unwrap(); // Unwrap `Vec<Payment>`

    // Now make sure that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows without `ORDER BY`
    // so assume we are lukky.
    assert_eq!(discord_users_vec, selected_users);
    println!("Yay!");

    //
    // END MYSQl EXAMPLE
    //

    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("$")) // set the bot's prefix to "$"
        .cmd("ping", ping)
        .cmd("hello", hello));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});

command!(hello(_context, message) {
    let _ = message.reply("Hello!");
});