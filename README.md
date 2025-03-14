# Rust basics

This project contains various smaller rust projects, often made by myself to
gain more understanding with a topic or dependency. It contains the absolute
basics of the language, the more advanced topics, but also demos on various
dependencies.

Completeness is not a goal of this project.

## Working with this large repo

Please prefer making use of the scripts.

```bash
$ ./cargo_crate.sh CRATE CARGO_COMMANDS...
$ ./cargo.sh CARGO_COMMANDS...
$ ./for_each_crate.sh ANY COMMANDS (in the crate)
$ ./for_each_crate_cargo.sh CARGO_COMMANDS...
```

## Highlighted demo crates

**Basics**
* [echargs](./crates/echargs/)
* [shortc](./crates/shortc/)
* [hello-world](./crates/hello-world/)
* [revsqrt](./crates/revsqrt/)

**Intermediate**
* [mpsc](./crates/mpsc/)
* [mpsc-full](./crates/mpsc-full/)
* [panic-calm](./crates/panic-calm/)
* [socker](./crates/socker/)

**Dependencies**
* [serde-json-demo](./crates/serde-json-demo/) (for `serde` and `serde_json`)
* [claptest](./crates/claptest/) (for `clap`)
* [ptlog](./crates/ptlog/) (for `libpt`)
* [sqlite-demo](./crates/sqlite-demo/) (for `rusqlite`)
* [onlytoken](./crates/onlytoken/) (for `rand` and `argon2`)

**Advanced Dependencies**
* [diesel-demo](./crates/diesel-demo/) (for `diesel` and CLI dependencies)
* [tokio-send-sync](./crates/tokio-send-sync/) (for `tokio`)
* [tokryon](./crates/tokryon/) (for `tokio` and `rayon`)
* [cucumber-demo](./crates/cucumber-demo/) (for `cucumber`)
* [criterion-demo](./crates/criterion-demo/) (for `criterion`)
* [revsqrt](./crates/revsqrt/) (the bench and tests, for `criterion` and `cucumber`)
* [rest](./crates/rest/) (for `serde`, `tokio` and `warp`)
* [rest-queued](./crates/rest-queued/) (for `serde`, `tokio` and `warp`)

## Warnings

* Some of the crates, especially those related to GUIs, may not work in WSL
environments.

## Additional dependencies

If you need to compile the whole workspace:

```bash
apt install libgtk-3-dev  -y
```

## Rust unsafe

Unsafe rust offers many possibilities otherwise locked from rust, which might
cause undefined behavior (or are dubbed unsafe for other reasons). Let's be
honest, they are often hacks. But they can have fun uses and are sometimes
interesting to explore, if only to see how the underlying system works.

Unsafe rust also has important uses when using programs developed in other
languages (like C or C++) or when manipulation of bits, bytes, and memory is
in needed (sorting algorithms).

See [rs-unsafe](rs-unsafe) for more.
