use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::{env, fs, io};

use anyhow::Ok;
/// This demo application uses a sqlite file to store some data. It does *not* use ORM (that would
/// be done with the `diesel` crate.)!
///
/// A very useful ressource is the
/// [rust-cookbook](https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html).
use rusqlite::{Connection, Rows};

const DBNAME: &str = "cats.db";
const TABLE_CAT_COLOR: &str = "cat_colors";
const TABLE_CAT: &str = "cats";

const USAGE_DELETE: &str = "Usage:
> D cat 15 (to delete cat with id 15)
> D cat 15 16 (to delete cat with id 15 and 16)
> D color 5 (to delete color with id 5)";

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
             color_id INTEGER NOT NULL REFERENCES CAT_COLORS(ID)
         )"
        ),
        (),
    )?;

    Ok(())
}

// color names are unique, so we use the color name instead of the id because it's easier
fn check_if_color_exists(conn: &Connection, color: &str) -> anyhow::Result<bool> {
    let mut stmt = conn.prepare(&format!(
        "SELECT EXISTS( SELECT 1 FROM {TABLE_CAT_COLOR} WHERE name = (LOWER(?1)))"
    ))?;

    // we need the return or the temporary result is dropped somehow
    #[allow(clippy::needless_return)]
    return Ok(stmt
        .query([color.to_string()])?
        .next()?
        .unwrap()
        .get::<usize, usize>(0)?
        == 1);
}

// note that cat names are not UNIQUE, so we need to use the id instead
fn check_if_cat_exists(conn: &Connection, id: usize) -> anyhow::Result<bool> {
    let mut stmt = conn.prepare(&format!(
        "SELECT EXISTS( SELECT 1 FROM {TABLE_CAT} WHERE id = (?1))"
    ))?;

    // we need the return or the temporary result is dropped somehow
    #[allow(clippy::needless_return)]
    return Ok(stmt.query([id])?.next()?.unwrap().get::<usize, usize>(0)? == 1);
}

/// Add a new color to the cat_colors table
///
/// Will fail if the sql fails
///
/// If the color already exists, will return Ok(id)
///
/// The table stores the name in lowercase
///
/// Returns the id of the color
fn new_color(conn: &Connection, color: &str) -> anyhow::Result<usize> {
    if check_if_color_exists(conn, color)? {
        return Ok(get_color_id(conn, color)?);
    }

    conn.execute(
        &format!("INSERT INTO {TABLE_CAT_COLOR} (name) VALUES (LOWER(?1))"),
        [color.to_string()],
    )?;

    Ok(get_color_id(conn, color)?)
}

fn get_color_id(conn: &Connection, color: &str) -> anyhow::Result<usize> {
    Ok(conn.query_row(
        &format!("SELECT id FROM {TABLE_CAT_COLOR} WHERE name = (LOWER(?1))"),
        [color.to_string()],
        |row| row.get::<usize, usize>(0),
    )?)
}

fn get_color_by_id(conn: &Connection, id: usize) -> anyhow::Result<Option<String>> {
    let maybe = conn.query_row(
        &format!("SELECT name FROM {TABLE_CAT_COLOR} WHERE id = (?1)"),
        [id.to_string()],
        |row| row.get::<usize, String>(0),
    );
    if let Result::Ok(color) = maybe {
        Ok(Some(color))
    } else {
        let err: rusqlite::Error = maybe.unwrap_err();
        if matches!(err, rusqlite::Error::QueryReturnedNoRows) {
            Ok(None)
        } else {
            Err(err.into())
        }
    }
}

fn new_cat(conn: &Connection, name: &str, color_id: usize) -> anyhow::Result<usize> {
    conn.execute(
        &format!("INSERT INTO {TABLE_CAT} (name, color_id) VALUES (LOWER(?1), ?2)"),
        [name.to_string(), color_id.to_string()],
    )?;

    Ok(conn.last_insert_rowid() as usize)
}

fn get_cat_color(conn: &Connection, id: usize) -> anyhow::Result<String> {
    Ok(conn.query_row(
        &format!("SELECT cc.name FROM {TABLE_CAT_COLOR} cc, {TABLE_CAT} c WHERE c.id = (?1) AND c.color_id = cc.id"),
        [id.to_string()],
        |row| row.get::<usize, String>(0),
    )?)
}

fn interactive_add_cat(conn: &Connection) -> anyhow::Result<usize> {
    let stdin = io::stdin();
    print!("the name of your cat?\n> ");
    io::stdout().flush()?;
    let mut cat_name: String = String::new();
    let _ = stdin.read_line(&mut cat_name)?;
    cat_name = cat_name.trim().to_string();

    print!("the color of your cat?\n> ");
    io::stdout().flush()?;
    let mut cat_color: String = String::new();
    let _ = stdin.read_line(&mut cat_color)?;
    cat_color = cat_color.trim().to_string();
    new_color(conn, &cat_color)?;

    let cat_id = new_cat(conn, &cat_name, get_color_id(conn, &cat_color)?)?;
    assert!(check_if_cat_exists(conn, cat_id)?);
    println!("your cat has id {cat_id}");
    println!("your cat has color '{}'", get_cat_color(conn, cat_id)?);

    Ok(cat_id)
}

fn interactive_find_cat(conn: &Connection) -> anyhow::Result<()> {
    let stdin = io::stdin();
    print!("the name of your cat?\n> ");
    io::stdout().flush()?;
    let mut cat_name: String = String::new();
    let _ = stdin.read_line(&mut cat_name)?;
    cat_name = cat_name.trim().to_string();

    print!("the color of your cat?\n> ");
    io::stdout().flush()?;
    let mut cat_color: String = String::new();
    let _ = stdin.read_line(&mut cat_color)?;
    cat_color = cat_color.trim().to_string();

    let mut stmt = conn.prepare(&format!(
        "SELECT * FROM {TABLE_CAT} c, {TABLE_CAT_COLOR} cc
        WHERE c.color_id = cc.id
        AND (c.name LIKE (?1) OR cc.name LIKE (?2))"
    ))?;

    let mut fitting_cats = stmt.query([cat_name, cat_color])?;
    println!("These cats might fit your description:\n");
    print_cats(conn, &mut fitting_cats)?;

    Ok(())
}

fn print_colors(_conn: &Connection, colors: &mut Rows) -> anyhow::Result<()> {
    println!("{: <14}| {: <19}", "id", "name");
    println!("{:=^80}", "");
    while let Some(color) = colors.next()? {
        println!(
            "{:<14}| {: <19}",
            color.get::<_, usize>(0)?,
            color.get::<_, String>(1)?
        )
    }
    Ok(())
}

fn print_cats(conn: &Connection, cats: &mut Rows) -> anyhow::Result<()> {
    println!(
        "{: <14}| {: <19}| {: <19} -> {: <19}",
        "id", "name", "color id", "color name"
    );
    println!("{:=^80}", "");
    while let Some(cat) = cats.next()? {
        println!(
            "{: <14}| {: <19}| {: <19} -> {: <19}",
            cat.get::<_, usize>(0)?,
            cat.get::<_, String>(1)?,
            cat.get::<_, usize>(2)?,
            get_color_by_id(conn, cat.get::<_, usize>(2)?)?.expect("no color for this id"),
        )
    }
    Ok(())
}

fn print_all_data(conn: &Connection) -> anyhow::Result<()> {
    println!("Cat colors:");
    let mut stmt = conn.prepare(&format!("SELECT * FROM {TABLE_CAT_COLOR}"))?;
    let mut colors = stmt.query([])?;
    print_colors(conn, &mut colors)?;

    println!("\n\nCats:");
    let mut stmt = conn.prepare(&format!("SELECT * FROM {TABLE_CAT}"))?;
    let mut cats = stmt.query([])?;
    print_cats(conn, &mut cats)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let conn = connect()?;

    setup(&conn)?;

    let stdin = io::stdin();
    let mut buf: String = String::new();

    loop {
        print!("(A)dd a cat, (F)ind a cat, (P)rint out all data, (D)elete data, or (E)xit?\n> ");
        io::stdout().flush()?;
        let _ = stdin.lock().read_line(&mut buf);
        buf = buf.trim().to_string();
        buf = buf.to_uppercase().to_string();
        if buf.starts_with('A') {
            interactive_add_cat(&conn)?;
        } else if buf.starts_with('F') {
            interactive_find_cat(&conn)?;
        } else if buf.starts_with('P') {
            print_all_data(&conn)?;
        } else if buf.starts_with('D') {
            let words: Vec<&str> = buf.split(' ').collect();
            dbg!(&words);
            if words.len() < 2 {
                println!("{USAGE_DELETE}");
            } else {
                let mode = words[1];
                let nums: Vec<usize> = words[2..]
                    .iter()
                    .map(|word| {
                        word.parse::<usize>()
                            .expect("could not parse db integer to rust integer")
                    })
                    .collect();
                dbg!(&nums);
                match mode {
                    "CAT" => {
                        let mut stmt =
                            conn.prepare(&format!("DELETE FROM {TABLE_CAT} WHERE id = (?1)"))?;
                        for n in nums {
                            stmt.execute([n])?;
                        }
                    }
                    "COLOR" => {
                        // TODO: we need to delete cats that have this color too, or abort if such cats exist, or give
                        // the user the option to choose!
                        todo!();
                        let mut stmt = conn
                            .prepare(&format!("DELETE FROM {TABLE_CAT_COLOR} WHERE id = (?1)"))?;
                        let _ = nums.iter().map(|n| stmt.execute([n]));
                    }
                    _ => {
                        println!("{USAGE_DELETE}");
                    }
                }
            }
        } else if buf.starts_with('E') {
            println!("Goodbye");
            break;
        }
        println!("\n(Enter to continue)");
        let _ = stdin.lock().read_line(&mut buf); // wait for enter as confirmation
        buf.clear();
        print!("{}[2J", 27 as char); // clear terminal
        io::stdout().flush()?;
    }

    Ok(())
}
