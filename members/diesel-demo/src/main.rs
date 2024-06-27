use diesel::SqliteConnection;
use diesel_demo::models::{Post, PostDraft};
use libpt::log::{self, debug, error, trace, warn};

const HELP_TEXT: &str = "\
                help|?              -     show this menu\n\
                exit                -     exit the application\n\
                list                -     list all posts\n\
                publish [id]        -     publish the post with the id [id]\n\
                unpublish [id]      -     make the post with the id [id] a draft\n\
                delete [id]         -     delete the post with the id [id]\n\
                read|show [id]      -     display the post with the id [id]\n\
                new                 -     create a new post";
const USAGE_TEXT: &str = "Bad input: try 'help'";

use colored::*;

use diesel_demo as lib;

fn main() -> anyhow::Result<()> {
    let _logger = log::Logger::builder()
        .max_level(log::Level::INFO)
        .display_level(false)
        .show_time(false)
        .build();
    debug!("logger initialized");

    let mut conn = lib::establish_connection()?;
    debug!("db connection established");

    trace!("entering the repl");
    repl(&mut conn)?;
    trace!("leaving the repl");

    Ok(())
}

fn repl(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    let mut buf = String::new();

    loop {
        lib::read_buf_interactive(&mut buf)?;
        buf = buf.to_uppercase();
        if buf.starts_with("HELP") || buf.starts_with("?") {
            println!("{}", HELP_TEXT.bright_blue())
        } else if buf.starts_with("EXIT") || buf.is_empty() {
            break;
        } else if buf.starts_with("UNPUBLISH") {
            let id: i32 = match get_id(&buf) {
                Some(i) => i,
                None => continue,
            };
            if let Err(e) = Post::publish(conn, id, false) {
                if let Some(e) = e.downcast_ref::<diesel::result::Error>() {
                    if matches!(e, diesel::result::Error::NotFound) {
                        warn!("No post with id {id} exists");
                        continue;
                    }
                }
            };
        } else if buf.starts_with("PUBLISH") {
            let id: i32 = match get_id(&buf) {
                Some(i) => i,
                None => continue,
            };
            if let Err(e) = Post::publish(conn, id, true) {
                if let Some(e) = e.downcast_ref::<diesel::result::Error>() {
                    if matches!(e, diesel::result::Error::NotFound) {
                        warn!("No post with id {id} exists");
                        continue;
                    }
                }
            };
        } else if buf.starts_with("READ") || buf.starts_with("SHOW") {
            let id: i32 = match get_id(&buf) {
                Some(i) => i,
                None => continue,
            };
            let r = Post::get(conn, id);
            let post: Post = if let Err(e) = r {
                if let Some(e) = e.downcast_ref::<diesel::result::Error>() {
                    if matches!(e, diesel::result::Error::NotFound) {
                        warn!("No post with id {id} exists");
                    }
                }
                continue;
            } else {
                r.unwrap()
            };
            println!("{post}");
        } else if buf.starts_with("DELETE") {
            let id: i32 = match get_id(&buf) {
                Some(i) => i,
                None => continue,
            };
            if let Err(e) = Post::delete(conn, id) {
                if let Some(e) = e.downcast_ref::<diesel::result::Error>() {
                    if matches!(e, diesel::result::Error::NotFound) {
                        warn!("No post with id {id} exists");
                        continue;
                    }
                }
            };
        } else if buf.starts_with("LIST") {
            let posts = lib::load_all_posts(conn)?;
            trace!("loaded posts for display: {posts:#?}");
            lib::print_posts(&posts);
        } else if buf.starts_with("NEW") {
            let post = PostDraft::interactive_create()?;
            let _ = post.post(conn).inspect_err(|e| {
                error!("Could not submit the post: {}", e.to_string());
            });
        } else {
            usage()
        }
    }

    Ok(())
}

fn usage() {
    println!("{}", USAGE_TEXT.red().bold());
}

fn get_id(buf: &str) -> Option<i32> {
    match buf.split(' ').nth(1) {
        Some(s) => match s.parse() {
            Ok(i) => Some(i),
            Err(e) => {
                error!("could not parse the id: {}", e.to_string());
                None
            }
        },
        None => {
            usage();
            None
        }
    }
}
