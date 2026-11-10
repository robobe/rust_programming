# Task Status Example

This example demonstrates Rust enums with variants, pattern matching, and the Task model.

## Prerequisites and commands

Use stable Rust with rustfmt and complete Module 3 Topics 1–2 first. No
external crates are needed. Rust runs directly; the course `.venv` is used
for documentation tooling. Run from the repository root:

```bash
rustc examples/03-structs-enums-and-data-modeling/03-enums/main.rs -o /tmp/rust-task-enums
/tmp/rust-task-enums
rustc --test examples/03-structs-enums-and-data-modeling/03-enums/main.rs -o /tmp/rust-task-enums-tests
/tmp/rust-task-enums-tests
rustfmt --check examples/03-structs-enums-and-data-modeling/03-enums/main.rs
```

The test command runs one test checking construction, completion, and cancellation.

## Expected Output

```text
Created task: Task { id: 1, title: "Read the Rust book", status: Pending }
Status: Pending
Completed!
Status: Completed
```

## What This Example Demonstrates

1. **Struct**: The `Task` struct holds id, title, and status
2. **Enum**: The `TaskStatus` enum defines possible states
3. **Pattern Matching**: Matching against enum variants
4. **Methods**: Methods that modify the task status

## Limitations

- The example is minimal and focuses on syntax demonstration
- No persistence or validation; status setters are unconditional, with checked transitions introduced in Topic 6
- Simple string handling
