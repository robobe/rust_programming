# Rust Programming: From Zero to Useful

## Domain model

This course teaches stable Rust language fundamentals and Cargo workflow for
beginner programmers.

- **Rust program:** source compiled into a binary or library.
- **Ownership:** the rule that gives each value one owner and controls when it
  is dropped.
- **Borrowing:** temporary access to a value through a reference.
- **Cargo:** Rust's project manager and build tool.
- **Capstone:** a small command-line task tracker built incrementally.

- **Task:** a record with an identifier, owned title, and status.
- **Task status:** Pending, Completed, or Cancelled with a reason in the extended model.
- **Task position:** a zero-based collection index, distinct from its identifier.
- **Optional lookup:** an `Option<&Task>` that borrows an existing task or reports absence.
- **Completion outcome:** `Result<(), TaskError>`, rejecting completed or cancelled tasks.
