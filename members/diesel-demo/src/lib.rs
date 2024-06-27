pub mod models;
pub mod schema;

use std::io::Write;
use std::{env, io};

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use dotenvy::dotenv;

use libpt::log::{error, info, warn};

use self::models::*;
use self::schema::posts::dsl::*;
use diesel::prelude::*;

pub fn establish_connection() -> anyhow::Result<SqliteConnection> {
    dotenv()?;

    let database_url = env::var("DATABASE_URL")?;
    Ok(SqliteConnection::establish(&database_url)
        .inspect_err(|e| error!("Error connecting to {}:\n{e:#?}", database_url))?)
}

pub fn load_posts(conn: &mut SqliteConnection) -> anyhow::Result<Vec<models::Post>> {
    Ok(posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(conn)?)
}

pub fn print_posts(posts_to_print: &Vec<Post>) {
    if !posts_to_print.is_empty() {
        info!("Displaying {} posts", posts_to_print.len());
        for post in posts_to_print {
            println!("{}", post.title);
            println!("-----------\n");
            println!("{}", post.body);
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
