use diesel::SqliteConnection;
use diesel_demo::models::PostDraft;
use libpt::log::{self, debug, error, trace};

use diesel_demo as lib;

fn main() -> anyhow::Result<()> {
    let _logger = log::Logger::builder()
        .max_level(log::Level::TRACE)
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
                new                 -     create a new post"
            )
        } else if buf.starts_with("EXIT") {
            break;
        } else if buf.starts_with("LIST") {
            let posts = lib::load_posts(conn)?;
            lib::print_posts(&posts);
        } else if buf.starts_with("NEW") {
            let post = PostDraft::interactive_create()?;
            let _ = post.post(conn).inspect_err(|e| {
                error!("Could not submit the post: {e:?}");
            });
        } else {
            println!("Bad input: try 'help'");
        }
    }

    Ok(())
}
