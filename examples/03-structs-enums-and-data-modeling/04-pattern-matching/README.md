# Pattern matching and destructuring

Prerequisites: stable Rust with rustfmt and the preceding Module 3 topics.
No external crates are needed. Run from the repository root. Rust commands run
directly; the course `.venv` is for documentation tooling.

```bash
rustc examples/03-structs-enums-and-data-modeling/04-pattern-matching/main.rs -o /tmp/rust-04-pattern-matching
/tmp/rust-04-pattern-matching
rustfmt --check examples/03-structs-enums-and-data-modeling/04-pattern-matching/main.rs
```

Expected program output:

```text
task 7: Read Rust
borrowed reason: Duplicate
owned reason: Duplicate
```

Assertions verify the illustrated behavior. 

## Limitations

Read Module 3 Topic 3 first. This demo changes Cancelled into a struct-like
variant with a reason; code using the earlier unit-like variant must be updated.
Borrowing preserves owned fields; moving a variant can partially move its task.
