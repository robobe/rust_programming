# Result: representing operation outcomes

Prerequisites: stable Rust with rustfmt and the preceding Module 3 topics.
No external crates are needed. Run from the repository root. Rust commands run
directly; the course `.venv` is for documentation tooling.

```bash
rustc examples/03-structs-enums-and-data-modeling/06-result/main.rs -o /tmp/rust-06-result
/tmp/rust-06-result
rustfmt --check examples/03-structs-enums-and-data-modeling/06-result/main.rs
```

Expected program output:

```text
completed task 7: Read Rust
second completion: AlreadyCompleted
```

Assertions verify the illustrated behavior. 

## Limitations

Read Module 3 Topic 5 first. Cancelled and completed tasks cannot be completed
again. This small domain error is not yet a standard Error implementation;
error propagation and user-facing error reporting come in Module 5.
