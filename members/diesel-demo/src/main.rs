use diesel::SqliteConnection;
use diesel_demo::models::{Post, PostDraft};
use libpt::log::{self, debug, error, trace, warn};

use diesel_demo as lib;

fn main() -> anyhow::Result<()> {
    let _logger = log::Logger::builder()
        .max_level(log::Level::DEBUG)
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
        if buf.starts_with("HELP") {
            println!(
                "\
                help                -     show this menu\n\
                exit                -     exit the application\n\
                list                -     list all posts\n\
                publish [id]         -     delete the post with the id [id]\n\
                delete [id]         -     delete the post with the id [id]\n\
                new                 -     create a new post"
            )
        } else if buf.starts_with("EXIT") {
            break;
        } else if buf.starts_with("PUBLISH") {
            let id: i32 = match get_id(&buf) {
                Some(i) => i,
                None => continue,
            };
            if let Err(e) = Post::publish(conn, id){
                if let Some(e) = e.downcast_ref::<diesel::result::Error>() {
                    if matches!(e, diesel::result::Error::NotFound) {
                        warn!("No post with id {id} exists");
                    }
                }
            };
        } else if buf.starts_with("DELETE") {
            let id: i32 = match get_id(&buf) {
                Some(i) => i,
                None => continue,
            };
            if let Err(e) = Post::delete(conn, id){
                if let Some(e) = e.downcast_ref::<diesel::result::Error>() {
                    if matches!(e, diesel::result::Error::NotFound) {
                        warn!("No post with id {id} exists");
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
                error!("Could not submit the post: {e:?}");
            });
        } else {
            usage()
        }
    }

    Ok(())
}

fn usage() {
    println!("Bad input: try 'help'");
}

fn get_id(buf: &str) -> Option<i32> {
    match buf.split(' ').nth(1) {
        Some(s) => match s.parse() {
            Ok(i) => Some(i),
            Err(e) => {
                error!("could not parse the id: {e:?}");
                None
            }
        },
        None => {
            usage();
            None
        }
    }
}
