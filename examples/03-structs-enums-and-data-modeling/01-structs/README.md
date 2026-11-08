# Structs demo

Prerequisite: the stable Rust toolchain from Module 0. The course `.venv` is
used for the documentation build, not for compiling this Rust demo.

From the repository root, run:

```bash
rustc examples/03-structs-enums-and-data-modeling/01-structs/main.rs -o /tmp/rust-task-structs
/tmp/rust-task-structs
```

Expected output:

```text
task: Task { id: 1, title: "Read the Rust book" }
title: Read the Rust book
updated: Task { id: 1, title: "Practice structs" }
moved title: Practice structs
remaining id: 1
```

This demo uses one named-field struct. It has no task status, methods, storage,
or error handling; those arrive in later topics. The binary is written to
`/tmp/rust-task-structs` and is not persisted in the repository.
