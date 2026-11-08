# Methods and associated functions demo

Prerequisite: the stable Rust toolchain from Module 0.

From the repository root, run:

```bash
rustc examples/03-structs-enums-and-data-modeling/02-methods/main.rs -o /tmp/rust-task-methods
/tmp/rust-task-methods
rustc --test examples/03-structs-enums-and-data-modeling/02-methods/main.rs -o /tmp/rust-task-methods-tests
/tmp/rust-task-methods-tests
```

Expected program output:

```text
created: Task { id: 1, title: "Read the Rust book" } (id: 1)
title: Read the Rust book
renamed: Task { id: 1, title: "Practice methods" }
owned title: Practice methods
```

Expected test output includes one passing test. The demo uses methods with
`&self`, `&mut self`, and `self`; it does not yet model task status or errors.
