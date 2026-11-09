# Syllabus

This syllabus moves from toolchain basics to ownership, then into application
design and a tested command-line capstone. Modules 0–1 are available; Module 2 is
partially implemented (Topics 1–7 are published, Topic 8 is planned). Module 3 is published; Modules 4–13 below are planned. Follow the numbered order; later modules name any
additional prerequisites. A topic listed here is not necessarily a published lesson.

## Hands-on data structures and patterns

The practice ledger in
`design/learning/data-structures-and-patterns-practice.md` tracks prerequisites,
required checks, and planned/implemented status. It is linked from the repository's
`design/README.md`; all assigned exercises are currently planned.

- **Modules 1–2:** array search, in-place slice reversal, and duplicate removal.
- **Modules 3–5:** a Vec-backed stack, its generic version, and public API tests.
- **Module 6:** requirement-driven design patterns, queues with `VecDeque`,
  frequency tables with `HashMap`, and uniqueness with `HashSet`.
- **Module 8:** iterator-based collection pipelines.
- **Module 9:** linked structures, a binary search tree, and graph traversal with BFS/DFS.

Each exercise includes runnable code, edge-case checks, complexity discussion,
and a collapsed solution. Existing lessons receive these as follow-up additions;
future lessons include assigned practice when the prerequisites are satisfied.

## [0. Hello, Rust and tooling](modules/00-first-lesson/index.md)

- Install Rust and understand `rustup`, `rustc`, Cargo, `rustfmt`, Clippy, and
  rust-analyzer.
- Build, run, format, check, test, and package a Hello World application.
- Optional extensions: Debian packaging, compiler outputs, Cargo workspaces,
  and Raspberry Pi Pico cross-compilation. These are not prerequisites for Module 1.

Completion goal: build, run, format, and check a Hello World application.

## [1. Rust fundamentals](modules/01-fundamentals/index.md)

All eight topics are published: an overview, bindings, scalar types,
collections, functions, conditions, loops, and Cargo/testing practice. Follow
the [topic index](modules/01-fundamentals/index.md) in numbered order.

- Variables, mutability, scalar and compound types.
- Functions, expressions, control flow, and pattern basics.
- Arrays, tuples, `String`, and `Vec<T>`; loops, indexing, and basic methods.
- Structuring small programs with Cargo.
- Testing simple behavior and reading compiler feedback.

Completion goal: write and test a function that summarizes a list of numbers.

## [2. Ownership and borrowing](modules/02-ownership-and-borrowing/index.md)

- Values, variables, scope, stack and heap storage.
- Move semantics, `Copy`, and `Clone`.
- Shared and mutable borrowing.
- Collections, iterators, slices, and API boundaries.
- Introductory lifetimes: references must remain valid while used (planned Topic 8).
- Advanced ownership continues in Modules 8–10 and 13 after structs and traits.
- [Hands-on review](modules/02-ownership-and-borrowing/09-hands-on.md): ownership
  tracing, move repair, snapshots, borrowed iteration, numeric/text slices,
  and an advanced pipeline combining borrowing with ownership transfer.

Completion goal: implement functions that read and update caller-owned data
without unnecessary clones, and explain a borrow-checker diagnostic.

## [3. Structs, enums, and data modeling](modules/03-structs-enums-and-data-modeling/index.md) — all seven topics published

- Define structs and implement methods with `impl` blocks.
- Use associated functions and model domain data.
- Define enums and use `match`, `if let`, and pattern matching.
- Represent missing values with `Option<T>`.
- Model operation outcomes with `Result<T, E>`.

Completion goal: model a task with a struct, status enum, and methods, then
demonstrate success, a missing task with `Option`, and a rejected operation with
`Result`.

## 4. Traits and generics — planned

- Define and implement traits.
- Use generic functions, generic types, and trait bounds.
- Derive common traits such as `Debug`, `Clone`, and `Default`.
- Compare trait bounds, `impl Trait`, and `dyn Trait` at an introductory level.

Completion goal: implement one trait for two types and use both through a
generic function.

## 5. Error handling, modules, and testing — planned

- Distinguish recoverable errors from panics and interpret `Result`.
- Create custom error types and propagate errors with `?`.
- Organize modules, visibility, Cargo targets, and unit/integration tests.
- Read files and test malformed input and missing files.

Completion goal: build a library plus binary that loads data and reports errors
without using `unwrap()` for expected failures.

## 6. Rust software design and patterns — planned

- Prerequisites: Modules 3–5.
- Encapsulation: private fields, validated constructors, and types that prevent
  invalid states.
- Apply SOLID pragmatically: focused responsibilities, useful extension points,
  trait behavioral contracts, small interfaces, and dependency inversion.
- Compose structs, enums, and traits; inject dependencies through constructors
  and function parameters.
- Design public APIs with deliberate ownership, borrowed parameters, meaningful
  errors, and minimal exposed implementation details.
- Use Builder, Strategy, Adapter, State, and Newtype patterns when concrete
  requirements justify them; compare enum-based and trait-based designs.
- Separate core application logic from CLI input, file storage, networking,
  and hardware; refactor incrementally with tests.

Completion goal: build a task service with a small storage trait, an in-memory
implementation for tests, and a file implementation. Test the same behavioral
contract for both, and explain why each abstraction is needed.

## 7. Command-line task tracker capstone — planned

- Parse command-line commands and arguments.
- Combine structs, enums, ownership, borrowing, and error handling.
- Persist tasks to a file using a stable data format.
- Split the application into focused Cargo modules.
- Apply Module 6 design principles to separate task logic from CLI and storage.
- Add unit tests, integration tests, and useful user-facing errors.

Completion goal: provide add, list, complete, and delete commands; preserve
tasks across runs; test valid commands, invalid input, and persistence failures.

## 8. Closures and advanced iterators — planned

- Prerequisites: Modules 2–5.

### Planned topic sequence

1. **Closures and captures:** closure syntax, borrowing captures, `move`,
   and the `Fn`, `FnMut`, and `FnOnce` traits.
2. **Iterator fundamentals and laziness:** `Iterator`, `next()`, and why
   adapters do no work until the iterator is consumed. Apply the same methods
   to ranges and collection iterators.
3. **Transforming and filtering:** `map()` changes each yielded value;
   `filter()` keeps values matching a predicate.
4. **Selecting and numbering:** `take()` limits the number of items, `skip()`
   omits initial items, and `enumerate()` pairs items with zero-based positions.
   Revisit `rev()` and `step_by()` and explore how adapter order changes results.
5. **Consuming results:** `sum()`, `count()`, and `collect()`, including the
   type annotations needed to choose a sum type or output collection.
6. **Ownership through pipelines:** compare `iter()`, `iter_mut()`, and
   `into_iter()` when closures read, change, or take ownership of elements.
7. **Custom iterators and practice:** implement `Iterator` for a small type,
   then build and test a filtering pipeline with empty and no-match cases.

Completion goal: build and test a filtering pipeline with a closure whose
capture mode you can explain.

## 9. Advanced lifetimes and smart pointers — planned

- Prerequisites: Modules 3–5 and 8.
- Borrowed structs, lifetime annotations, elision, and method receivers.
- Ownership in `Option` and `Result`; `as_ref` and `as_mut`.
- `Box`, `Rc`, and `RefCell`; interior mutability and ownership design.

Completion goal: compare owned and borrowed representations, and demonstrate
shared ownership with checked interior mutation.

## 10. Threads and concurrency — planned

- Prerequisites: Modules 8–9.
- Thread spawning, joining, channels, `Send`, and `Sync`.
- `Arc`, `Mutex`, and `RwLock`; lock scope and deadlock avoidance.

Completion goal: implement a threaded worker example and test that all results
are collected and all threads terminate.

## 11. Async Rust — planned

- Prerequisites: Modules 8–10.
- Futures, `async`/`.await`, executors, and task ownership.
- Async I/O, cancellation, timeouts, and avoiding blocking work in async tasks.

Completion goal: implement concurrent I/O with explicit timeout and error handling.

## 12. Performance and profiling — planned

- Prerequisites: Modules 7–9; Modules 10–11 for concurrent examples.
- Release builds, benchmarks, profiling, allocation costs, and cloning costs.
- Measure changes while preserving behavior and tests.

Completion goal: benchmark a working program, identify one bottleneck, and
compare performance before and after a tested improvement.

## 13. Unsafe Rust and FFI — planned

- Prerequisites: Modules 9–12.
- Raw pointers, safety contracts, and small safe interfaces around unsafe code.
- C interoperability, ABI boundaries, and allocation ownership across FFI.
- Self-referential designs and pinning, revisiting futures from Module 11.

Completion goal: wrap a small FFI call with documented ownership and safety
requirements and tests for its safe interface.

