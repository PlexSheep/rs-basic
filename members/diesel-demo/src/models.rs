use std::io::{self, Read, Write};

use dialoguer::Confirm;
use diesel::prelude::*;
use libpt::log::{info, trace};

use crate::schema::posts;

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))] // optional but improves generated compiler errors
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Post {
    pub fn get(conn: &mut SqliteConnection, id: i32) -> anyhow::Result<Post> {
        use crate::schema::posts::dsl::posts;

        Ok(posts.find(id).select(Post::as_select()).first(conn)?)
    }

    pub fn publish(conn: &mut SqliteConnection, id: i32, publish: bool) -> anyhow::Result<()> {
        use crate::schema::posts::dsl::{posts, published};

        let post = diesel::update(posts.find(id))
            .set(published.eq(publish))
            .returning(Post::as_returning())
            .get_result(conn)?;
        info!("updated post {}: publish = {}", post.id, post.published);
        Ok(())
    }
    pub fn delete(conn: &mut SqliteConnection, id: i32) -> anyhow::Result<()> {
        let confirmation = Confirm::new()
            .with_prompt(format!("You are about to delete post {id}, continue?"))
            .interact()?;
        if !confirmation {
            return Ok(());
        }

        use crate::schema::posts::dsl::posts;

        let post = diesel::delete(posts.find(id))
            .returning(Post::as_returning())
            .get_result(conn)?;
        info!("deleted post {}", post.id);
        Ok(())
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))] // optional but improves generated compiler errors
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
        trace!("PostDraft to post: {self:#?}");
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

        println!("(End with {} when finished) Body:", EOF);
        stdin.read_to_string(&mut body)?;
        body = body.trim().to_string();

        Ok(Self::new(&title, &body))
    }
}
