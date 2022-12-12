#[macro_use] extern crate diesel;

mod ap;
mod models;
mod sc;
mod schema;
mod twitter;

use clap::Parser;
use diesel_async::pooled_connection::{mobc::Pool, AsyncDieselConnectionManager};
use diesel_async::AsyncPgConnection;
use twitter::Twitter;
use twitter_v2::authorization::BearerToken;

type PgPool = Pool<AsyncPgConnection>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let args = ap::Cli::parse();

    // Setup database connection
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = init_db(db_url);

    // Load auth token & create API interface
    let auth = std::env::var("TWTARC_AUTH")
        .expect("TWTARC_AUTH must be set");
    let twt = Twitter::new(BearerToken::new(auth));

    // Run appropriate subcommand
    match args.command {
        ap::Commands::Synch => sc::sync(twt, pool),
        ap::Commands::User { command } => match command {
            ap::UserCommands::List => sc::user_list(pool).await,
            ap::UserCommands::Add { url } => sc::user_add(twt, pool, url).await,
        },
        ap::Commands::Insert { url } => sc::insert(twt, pool, url),
    }
}

fn init_db(db_url: String) -> PgPool {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    Pool::new(config)
}
