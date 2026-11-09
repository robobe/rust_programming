# Putting the task model together

Prerequisites: stable Rust with rustfmt and the preceding Module 3 topics.
No external crates are needed. Run from the repository root. Rust commands run
directly; the course `.venv` is for documentation tooling.

```bash
rustc examples/03-structs-enums-and-data-modeling/07-task-model/main.rs -o /tmp/rust-07-task-model
/tmp/rust-07-task-model
rustfmt --check examples/03-structs-enums-and-data-modeling/07-task-model/main.rs
rustc --test examples/03-structs-enums-and-data-modeling/07-task-model/main.rs -o /tmp/rust-task-model-tests
/tmp/rust-task-model-tests
```

Expected program output:

```text
task 42 completed: Practice task modeling
repeat: Err(AlreadyCompleted)
found id 42: Practice task modeling
missing id 999: true
```

Assertions verify the illustrated behavior. The test command runs four unit tests.

## Limitations

Read Module 3 Topic 6 first. IDs are supplied by the caller and assumed unique;
a duplicate ID returns the first match. Lookup is linear and borrowed. Fields
remain accessible for teaching; encapsulation, title validation, ID generation,
and persistence belong to later modules. Cancellation is constructed explicitly
here rather than adding a cancellation API unrelated to this exercise.
