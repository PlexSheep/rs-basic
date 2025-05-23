pub mod cli;
pub mod models;
pub mod schema;

use self::schema::posts::dsl::*;

use std::env;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use dotenvy::dotenv;

use libpt::log::error;

pub fn establish_connection() -> anyhow::Result<SqliteConnection> {
    dotenv()?;

    let database_url = env::var("DATABASE_URL")?;
    Ok(SqliteConnection::establish(&database_url)
        .inspect_err(|e| error!("Error connecting to {}:\n{e:#?}", database_url))?)
}

pub fn load_all_posts(conn: &mut SqliteConnection) -> anyhow::Result<Vec<models::Post>> {
    Ok(posts.select(models::Post::as_select()).load(conn)?)
}

pub fn load_relevant_posts(conn: &mut SqliteConnection) -> anyhow::Result<Vec<models::Post>> {
    Ok(posts
        .filter(schema::posts::published.eq(true))
        .limit(5)
        .select(models::Post::as_select())
        .load(conn)?)
}
