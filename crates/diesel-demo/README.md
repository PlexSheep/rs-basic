# Diesel seems more complex

This project uses sqlite as I can't be bothered to host a postgres instance
just for this.

## Setup

* Install diesel and the diesel CLI tool.
* `DATABASE_URL=./data/dieseldemo.db diesel setup`
* We create migrations up/down and fill them with sql to do something and undo something
* Alternatively, we can use a rust macro to define schemes that become sql
  through magic. I like this way better for this project because I learn more
  diesel and also because it avoids having to deal with incompatibilities of the
  database types (maybe).
    * Run `diesel migration generate --diff-schema create_posts`

## Ressources

* [Official Guide](https://diesel.rs/guides/getting-started)

## Notes on the CLI

I initially made a bare bones REPL with just regular prints. Eventually,
this turned into a demo for CLI libraries too.
