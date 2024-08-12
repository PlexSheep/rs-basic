# Rust basics

This project contains various smaller rust projects, often made by myself to
gain more understanding with a topic or dependency. It contains the absolute
basics of the language, the more advanced topics, but also demos on various
dependencies.

Completeness is not a goal of this project.

## Highlighted demo crates

**Basics**
* [echargs](./members/echargs/)
* [shortc](./members/shortc/)
* [hello-world](./members/hello-world/)
* [revsqrt](./members/revsqrt/)

**Intermediate**
* [mpsc](./members/mpsc/)
* [mpsc-full](./members/mpsc-full/)
* [panic-calm](./members/panic-calm/)
* [socker](./members/socker/)

**Dependencies**
* [serde-json-demo](./members/serde-json-demo/) (for `serde` and `serde_json`)
* [claptest](./members/claptest/) (for `clap`)
* [ptlog](./members/ptlog/) (for `libpt`)
* [sqlite-demo](./members/sqlite-demo/) (for `rusqlite`)
* [onlytoken](./members/onlytoken/) (for `rand` and `argon2`)

**Advanced Dependencies**
* [diesel-demo](./members/diesel-demo/) (for `diesel` and CLI dependencies)
* [tokio-send-sync](./members/tokio-send-sync/) (for `tokio`)
* [tokryon](./members/tokryon/) (for `tokio` and `rayon`)
* [cucumber-demo](./members/cucumber-demo/) (for `cucumber`)
* [criterion-demo](./members/criterion-demo/) (for `criterion`)
* [revsqrt](./members/revsqrt/) (the bench and tests, for `criterion` and `cucumber`)
* [rest](./members/rest/) (for `serde`, `tokio` and `warp`)
* [rest-queued](./members/rest-queued/) (for `serde`, `tokio` and `warp`)

## Warnings

* Some of the crates, especially those related to GUIs, may not work in WSL 
environments.

## Rust unsafe

Unsafe rust offers many possibilities otherwise locked from rust, which might
cause undefined behavior (or are dubbed unsafe for other reasons). Let's be
honest, they are often hacks. But they can have fun uses and are sometimes
interesting to explore, if only to see how the underlying system works.

Unsafe rust also has important uses when using programs developed in other
languages (like C or C++) or when manipulation of bits, bytes, and memory is
in needed (sorting algorithms).

See [rs-unsafe](rs-unsafe) for more.
