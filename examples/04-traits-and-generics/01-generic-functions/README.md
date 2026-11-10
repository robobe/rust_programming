# Generic functions

Prerequisites: stable Rust with rustfmt, Module 2 borrowing and slices, and
Module 3 Option. No external crates are required. Run from the repository root.
Rust runs directly; the course `.venv` is used for documentation tooling.

```bash
rustc examples/04-traits-and-generics/01-generic-functions/main.rs -o /tmp/rust-generic-functions
/tmp/rust-generic-functions
rustc --test examples/04-traits-and-generics/01-generic-functions/main.rs -o /tmp/rust-generic-functions-tests
/tmp/rust-generic-functions-tests
rustfmt --check examples/04-traits-and-generics/01-generic-functions/main.rs
```

Expected program output:

```text
identity: 7, Rust, 9
first word: Some("Read")
still owned: Read
```

Two tests check owned and Copy inputs, explicit type arguments, borrowed String
items, singleton input, and empty input. The identity function takes ownership;
the first-item function borrows. No Clone or Copy bounds are needed. These
functions do not support arbitrary arithmetic on an unconstrained T, nor can
returned references outlive their input. Stack practice remains in Topic 9.
