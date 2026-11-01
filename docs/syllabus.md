# Syllabus

This syllabus moves from toolchain basics to ownership, then into application
design and a tested command-line capstone. Modules marked **planned** describe
the next course stages; the linked modules are available now.

## [0. Hello, Rust and tooling](modules/00-first-lesson/index.md)

- Install Rust and understand `rustup`, `rustc`, Cargo, `rustfmt`, Clippy, and
  rust-analyzer.
- Build, run, format, check, test, and package a Hello World application.
- Organize Cargo workspaces and cross-compile for a Raspberry Pi Pico.

## 1. Rust fundamentals — planned

- Variables, mutability, scalar and compound types.
- Functions, expressions, control flow, and pattern basics.
- Structuring small programs with Cargo.
- Testing simple behavior and reading compiler feedback.

## [2. Ownership and borrowing](modules/02-ownership-and-borrowing/index.md)

- Values, variables, scope, stack and heap storage.
- Move semantics, `Copy`, and `Clone`.
- Shared and mutable borrowing.
- Collections, iterators, slices, and API boundaries.
- Lifetimes, smart pointers, and advanced ownership patterns.

## 3. Structs, enums, and data modeling — planned

- Define structs and implement methods with `impl` blocks.
- Use associated functions and model domain data.
- Define enums and use `match`, `if let`, and pattern matching.
- Represent missing values with `Option<T>`.
- Model operation outcomes with `Result<T, E>`.

## 4. Traits, generics, error handling, and testing — planned

- Define and implement traits.
- Use generic functions, generic types, and trait bounds.
- Derive common traits such as `Debug`, `Clone`, and `Default`.
- Create custom error types and propagate errors with `?`.
- Organize modules, visibility, Cargo targets, and unit/integration tests.

## 5. Command-line task tracker capstone — planned

- Parse command-line commands and arguments.
- Combine structs, enums, ownership, borrowing, and error handling.
- Persist tasks to a file using a stable data format.
- Split the application into focused Cargo modules.
- Add unit tests, integration tests, and useful user-facing errors.

## 6+. Advanced Rust — planned

- Lifetimes in larger APIs and borrowed data structures.
- Closures, advanced iterators, smart pointers, and shared ownership.
- Threads, synchronization, and concurrency boundaries.
- Async Rust and task-based I/O.
- Performance analysis, unsafe Rust, and FFI contracts.

