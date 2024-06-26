use std::io;
/// This demo application uses a sqlite file to store some data. It does *not* use ORM (that would
/// be done with the `diesel` crate.)!
///
/// A very useful ressource is the
/// [rust-cookbook](https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html).
use std::io::{BufRead, Write};

use rusqlite::{Connection, Rows};

mod db;
use db::*;

const USAGE_DELETE: &str = "Usage:
> D cat 15 (to delete cat with id 15)
> D cat 15 16 (to delete cat with id 15 and 16)
> D color 5 (to delete color with id 5)";

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

fn interactive_delete(conn: &Connection, buf: &mut String) -> anyhow::Result<()> {
    let words: Vec<&str> = buf.split(' ').collect();
    if words.len() < 2 {
        println!("{USAGE_DELETE}");
    } else {
        let stdin = io::stdin();
        let mode = words[1];
        let mut nums: Vec<usize> = Vec::new();
        for word in words[2..].iter() {
            nums.push(match word.parse() {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Could not parse '{word}' to id: {e}");
                    continue;
                }
            })
        }
        match mode {
            "CAT" => {
                let mut stmt = conn.prepare(&format!("DELETE FROM {TABLE_CAT} WHERE id = (?1)"))?;
                for n in nums {
                    stmt.execute([n])?;
                }
            }
            "COLOR" => {
                // Cats have colors, so if we delete a color, we need to delete cats with
                // that color too.
                let mut stmt_how_many_cats_with_color = conn.prepare(&format!(
                    "SELECT COUNT(1) FROM {TABLE_CAT} c, {TABLE_CAT_COLOR}
                            cc WHERE c.color_id = (?1) AND cc.id = (?1)"
                ))?;
                // FIXME: this must still be wrong?
                let mut stmt_cats_with_color = conn.prepare(&format!(
                    "SELECT c.* FROM {TABLE_CAT} c, {TABLE_CAT_COLOR} cc
                        WHERE c.color_id = (?1) AND cc.id = (?1)"
                ))?;
                // works: `SELECT cats.* FROM cats, cat_colors WHERE cats.color_id = 2 AND cat_colors.id = 2;`

                for color_id in &nums {
                    let cats_amount: usize = stmt_how_many_cats_with_color
                        .query_row([color_id], |row| row.get::<_, usize>(0))?;

                    if cats_amount > 0 {
                        let mut cats = stmt_cats_with_color.query([color_id])?; // Get the cats
                                                                                // that would be deleted
                        println!(
                            "\nYou are about to also delete these cats,\n\
                            as they have the color id {color_id}. Type 'YES' to confirm."
                        );
                        print_cats(conn, &mut cats)?;
                        drop(cats); // we need to renew this, because Rows is not Clone and we have
                                    // consumed the Rows in print_cats. The Rows index can not be reset,
                                    // probably because of a sqlite API limitation.

                        let mut cats = stmt_cats_with_color.query([color_id])?;
                        let mut cat_ids: Vec<usize> = Vec::new();

                        while let Some(cat) = cats.next()? {
                            cat_ids.push(cat.get::<_, usize>(0)?);
                        }
                        buf.clear();
                        let _ = stdin.lock().read_line(buf); // wait for enter as confirmation
                        *buf = buf.trim().to_string();
                        *buf = buf.to_uppercase().to_string();
                        dbg!(&buf);
                        if buf.as_str() != "YES" {
                            continue;
                        }
                        let mut stmt =
                            conn.prepare(&format!("DELETE FROM {TABLE_CAT} WHERE id = (?1)"))?;
                        for cat_id in cat_ids {
                            stmt.execute([cat_id])?;
                        }
                    }

                    let mut stmt =
                        conn.prepare(&format!("DELETE FROM {TABLE_CAT_COLOR} WHERE id = (?1)"))?;
                    for n in &nums {
                        stmt.execute([n])?;
                    }
                }
                println!("deleted ids {nums:?}");
            }
            _ => {
                println!("{USAGE_DELETE}");
            }
        }
    }
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

/// Print [Rows] of cats.
///
/// This needs all columns of the [TABLE_CAT], otherwise it will error.
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
        buf.clear();
        // print!("{}[2J", 27 as char); // clear terminal
        io::stdout().flush()?;
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
            interactive_delete(&conn, &mut buf)?;
        } else if buf.starts_with('E') {
            println!("Goodbye");
            break;
        }
        println!("\n");
    }

    Ok(())
}
