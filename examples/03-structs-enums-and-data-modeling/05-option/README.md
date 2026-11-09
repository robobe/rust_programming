# Option: representing missing values

Prerequisites: stable Rust with rustfmt and the preceding Module 3 topics.
No external crates are needed. Run from the repository root. Rust commands run
directly; the course `.venv` is for documentation tooling.

```bash
rustc examples/03-structs-enums-and-data-modeling/05-option/main.rs -o /tmp/rust-05-option
/tmp/rust-05-option
rustfmt --check examples/03-structs-enums-and-data-modeling/05-option/main.rs
```

Expected program output:

```text
position 0: task 42 - Read Rust
position 99: no task
optional title: Practice
```

Assertions verify the illustrated behavior. 

## Limitations

Read Module 3 Topic 4 first. Positions are zero-based indexes, not identifiers.
The returned reference borrows the input; it cannot outlive that collection.
The panic in the fixture check verifies a known-present item, not user input.
