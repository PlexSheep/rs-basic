use std::fmt::Display;

use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::Table;
use console::{style, Color};
use dialoguer::{theme::ColorfulTheme, Completion, Input};
use dialoguer::{BasicHistory, History};
use libpt::log::{info, warn};

use crate::models::{self, Post};

const HELP_TEXT: &str = "\
                help|?              -     show this menu\n\
                exit                -     exit the application\n\
                list|ls             -     list all posts\n\
                publish [id]        -     publish the post with the id [id]\n\
                unpublish [id]      -     make the post with the id [id] a draft\n\
                delete [id]         -     delete the post with the id [id]\n\
                read|show [id]      -     display the post with the id [id]\n\
                new                 -     create a new post";
const USAGE_TEXT: &str = "Bad input: try 'help'";

pub struct MyCompletion {
    options: Vec<String>,
}
impl Default for MyCompletion {
    fn default() -> Self {
        MyCompletion {
            options: vec![
                "help".to_string(),
                "?".to_string(),
                "list".to_string(),
                "publish".to_string(),
                "unpublish".to_string(),
                "delete".to_string(),
                "read".to_string(),
                "show".to_string(),
                "new".to_string(),
                "ls".to_string(),
            ],
        }
    }
}

impl Completion for MyCompletion {
    /// Simple completion implementation based on substring
    fn get(&self, input: &str) -> Option<String> {
        let matches = self
            .options
            .iter()
            .filter(|option| option.starts_with(input))
            .collect::<Vec<_>>();

        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}
pub fn table_posts(posts_to_print: &Vec<models::Post>) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth)
        .set_header(vec!["id", "title", "published?", "body"]);
    for post in posts_to_print {
        let mut stitle = post.title.clone();
        stitle.truncate(40);
        let mut sbody = post.body.clone();
        sbody.truncate(40);

        table.add_row(vec![
            post.id.to_string(),
            stitle,
            post.published.to_string(),
            sbody,
        ]);
    }
    println!("{}", style(table).dim());
}

impl Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL_CONDENSED)
            .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth)
            .set_header(vec![format!("{:<6} {}", self.id, self.title)])
            .add_row(vec![self.body.clone()]);
        writeln!(f, "{table}")
    }
}

pub fn read_buf_interactive(
    buf: &mut String,
    completion: &impl Completion,
    history: &mut BasicHistory,
) -> anyhow::Result<()> {
    buf.clear();

    *buf = Input::with_theme(&ColorfulTheme::default())
        .completion_with(completion)
        .history_with(history)
        .interact_text()?;

    Ok(())
}

pub fn usage() {
    borderprint(USAGE_TEXT, Some(Color::Red));
}

pub fn help() {
    borderprint(HELP_TEXT, Some(Color::Cyan));
}

pub(crate) fn borderprint(content: impl ToString, color: Option<Color>) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth)
        .add_row(vec![content.to_string()]);
    match color {
        Some(c) => println!("{}", style(table).fg(c)),
        None => println!("{table}")
    }
}
