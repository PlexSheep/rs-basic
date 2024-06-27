# Rust basics

This project contains various smaller rust projects, often made by myself to
gain more understanding with a topic or dependency. It contains the absolute
basics of the language, the more advanced topics, but also demos on various
dependencies.

Completeness is not a goal of this project.

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
* [serde-json-demo](./members/serde-json-demo/)
* [claptest](./members/claptest/)
* [ptlog](./members/ptlog/)
* [slog-demo](./members/slog-demo/)
* [sqlite-demo](./members/sqlite-demo/)

**Advanced Dependencies**
* [diesel-demo](./members/diesel-demo/)
* [tokio-send-sync](./members/tokio-send-sync/)
* [tokryon](./members/tokryon/)
* [cucumber-demo](./members/cucumber-demo/)
* [criterion-demo](./members/criterion-demo/)
* [revsqrt](./members/revsqrt/) (the bench and tests)
* [rest](./members/rest/)
* [rest-queued](./members/rest-queued/)

## Rust unsafe

Unsafe rust offers many possibilities otherwise locked from rust, which might
cause undefined behavior (or are dubbed unsafe for other reasons). Let's be
honest, they are often hacks. But they can have fun uses and are sometimes
interesting to explore, if only to see how the underlying system works.

Unsafe rust also has important uses when using programs developed in other
languages (like C or C++) or when manipulation of bits, bytes, and memory is
in needed (sorting algorithms).

See [rs-unsafe](rs-unsafe) for more.
