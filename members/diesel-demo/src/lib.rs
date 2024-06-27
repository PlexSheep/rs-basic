pub mod models;
pub mod schema;

use self::schema::posts::dsl::*;

use std::io::Write;
use std::{env, io};

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use dotenvy::dotenv;

use libpt::log::{error, info, warn};

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

// NOTE: formatting breaks when you use japanese fullwidth (or probably other longer chars too)
// characters. Works well for the regular alphabet
pub fn print_posts(posts_to_print: &Vec<models::Post>) {
    if !posts_to_print.is_empty() {
        info!("{} posts are in the database", posts_to_print.len());
        println!(
            "{: <12}| {: <30} | {: <40}[...] | {: <12} | {: <5}",
            "id", "title", "body (truncated)", "body len", "is published?"
        );
        println!("{:=^140}", "");
        for post in posts_to_print {
            let mut short_title = post.body.clone();
            short_title.truncate(30);
            let mut short_body = post.body.clone();
            short_body.truncate(40);
            println!(
                "{: <12}| {: <30} | {: <40}[...] | {: <12} | {: <5}",
                post.id,
                short_title,
                short_body,
                post.body.len(),
                post.published
            );
        }
    } else {
        warn!("Tried to display posts, but there are no posts stored in the database");
    }
}

// NOTE: this can't handle unicode stuff like 春 and I don't really care
pub fn read_buf_interactive(buf: &mut String) -> anyhow::Result<()> {
    buf.clear();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    print!("> ");
    stdout.flush()?;
    stdin.read_line(buf)?;
    *buf = buf.trim().to_string();

    Ok(())
}
