pub mod developer;
pub mod functions;
pub mod hosts;
pub mod project;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;

use crate::config;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = config::env::database_url();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
