# Task Status Example

This example demonstrates Rust enums with variants, pattern matching, and the Task model.

## Prerequisites

- Install Rust: https://rustup.rs/
- Clone this repository
- Navigate to the examples directory

## Running the Example

From the repository root, run:

```bash
uv run cargo run --example 03-enums
```

Or from the examples directory:

```bash
cd examples/03-structs-enums-and-data-modeling/03-enums
uv run cargo run
```

## Expected Output

```
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
- No persistence or validation
- Simple string handling
