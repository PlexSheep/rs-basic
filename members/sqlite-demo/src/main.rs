use anyhow::Ok;
/// This demo application uses a sqlite file to store some data. It does *not* use ORM (that would
/// be done with the `diesel` crate.)!
///
/// A very useful ressource is the
/// [rust-cookbook](https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html).
use rusqlite::{Connection, Result};

fn setup(conn: &Connection) -> anyhow::Result<()> {
    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        (),
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        (),
    )?;

    Ok(())
}

fn new_color(conn: &Connection, color: &str) -> anyhow::Result<()> {
    conn.execute(
        "INSERT INTO cat_colors (name) values (?1)",
        [color.to_string()],
    )?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let conn = Connection::open("cats.db")?;

    setup(&conn)?;
    new_color(&conn, "black")?;
    new_color(&conn, "red")?;

    Ok(())
}
