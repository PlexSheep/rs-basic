pub mod schema;

use std::env;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use dotenvy::dotenv;

use libpt::log::error; // load envars from a `.env` file

pub fn establish_connection() -> anyhow::Result<SqliteConnection> {
    dotenv()?;

    let database_url = env::var("DATABASE_URL")?;
    Ok(SqliteConnection::establish(&database_url)
        .inspect_err(|e| error!("Error connecting to {}:\n{e:#?}", database_url))?)
}
