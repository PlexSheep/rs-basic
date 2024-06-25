use std::path::PathBuf;
use std::{env, fs};

use anyhow::Ok;
/// This demo application uses a sqlite file to store some data. It does *not* use ORM (that would
/// be done with the `diesel` crate.)!
///
/// A very useful ressource is the
/// [rust-cookbook](https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html).
use rusqlite::Connection;

const DBNAME: &str = "cats.db";
const TABLE_CAT_COLOR: &str = "cat_colors";
const TABLE_CAT: &str = "cats";

fn connect() -> anyhow::Result<Connection> {
    let mut wd = env::current_dir()?;
    let mut dbpath: Option<PathBuf> = None;

    // does the current directory have a data/cats.db ? (the file need not exist, we can create it
    // if data exists)
    {
        let mut wddata = wd.clone();
        wddata.push("data");
        if wddata.exists() {
            println!("found {DBNAME} in {:?}", &wddata);
            wddata.push(DBNAME);
            dbpath = Some(wddata);
        }
    }

    // otherwise, does the current or any higher directories contain a cats.db ?
    'search_dir: while dbpath.is_none() {
        for entry in fs::read_dir(&wd)? {
            let entry = entry?;
            if entry.file_name() == DBNAME {
                println!("found {DBNAME} in {:?}", wd);
                dbpath = Some(entry.path());
                break 'search_dir;
            }
        }
        if !wd.pop() {
            // we are at the root!
            break 'search_dir;
        }
    }

    // if all fails, use $PWD/data/cats.db
    if dbpath.is_none() {
        println!(
            "No {DBNAME} found, using {:?}/data/{DBNAME}",
            env::current_dir()?
        );
        fs::create_dir("data")?;
        let mut path = env::current_dir()?;
        path.push("data");
        path.push(DBNAME);
        dbpath = Some(path);
    }

    let conn = Connection::open(dbpath.unwrap())?;

    Ok(conn)
}

fn setup(conn: &Connection) -> anyhow::Result<()> {
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {TABLE_CAT_COLOR} (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL UNIQUE COLLATE NOCASE
         )"
        ),
        (),
    )?;
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {TABLE_CAT} (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL COLLATE NOCASE,
             COLOR_ID INTEGER NOT NULL REFERENCES CAT_COLORS(ID)
         )"
        ),
        (),
    )?;

    Ok(())
}

/// Add a new color to the cat_colors table
///
/// Will fail if the sql fails
///
/// If the color already exists, will return Ok(())
///
/// The table stores the name in lowercase
fn new_color(conn: &Connection, color: &str) -> anyhow::Result<()> {
    // check if the color already exists, so we can just try adding a color and not worry if it
    // already exists

    let mut stmt = conn.prepare(&format!(
        "SELECT EXISTS( SELECT 1 FROM {TABLE_CAT_COLOR} WHERE name = (?1))"
    ))?;
    if let Some(row) = stmt.query([color.to_string()])?.next()? {
        if row.get::<usize, usize>(0)? == 1 {
            // the color already exists
            return Ok(());
        }
    };

    // do the inserting
    conn.execute(
        "INSERT INTO cat_colors (name) VALUES (?1)",
        [color.to_string()],
    )?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let conn = connect()?;

    setup(&conn)?;
    new_color(&conn, "black")?;
    new_color(&conn, "red")?;
    new_color(&conn, "white")?;

    Ok(())
}
