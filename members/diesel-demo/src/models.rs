use std::io::{self, Read, Write};

use diesel::prelude::*;

use crate::schema::posts;

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))] // optional but improves generated compiler
                                                     // errors
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct PostDraft {
    pub title: String,
    pub body: String,
}

impl PostDraft {
    pub fn new<T: ToString>(title: T, body: T) -> Self {
        Self {
            title: title.to_string(),
            body: body.to_string(),
        }
    }
    pub fn post(self, conn: &mut SqliteConnection) -> anyhow::Result<Post> {
        Ok(diesel::insert_into(posts::table)
            .values(&self)
            .returning(Post::as_returning())
            .get_result(conn)?)
    }

    pub fn interactive_create() -> anyhow::Result<Self> {
        let mut title = String::new();
        let mut body = String::new();
        let mut stdin = io::stdin();

        print!("Title: ");
        io::stdout().flush()?;
        stdin.read_line(&mut title)?;
        title = title.trim().to_string();

        println!("(End with {} when finished) Body:\n", EOF);
        stdin.read_to_string(&mut body)?;
        body = body.trim().to_string();

        Ok(Self::new(&title, &body))
    }
}
