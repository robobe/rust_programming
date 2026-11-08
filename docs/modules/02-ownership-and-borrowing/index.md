# Module 2: Ownership and borrowing

This module introduces ownership, borrowing, slices, and basic lifetimes after
[Module 1 fundamentals](../01-fundamentals/index.md). Topics 1–7 are published; Topic 8 is planned.
Complete these foundations before
[Module 3: Structs, enums, and data modeling](../03-structs-enums-and-data-modeling/index.md).

## Learning path

### 1. Values, variables, and scope

- Stack values versus heap-backed values
- Scope and when a value is dropped
- The difference between a value and the variable that owns it
- Why `String`, `Vec<T>`, and other heap-backed types matter

Study [Values, variables, and scope](01-values-variables-scope.md) first.

### 2. Move semantics

- Assignment moves ownership for non-`Copy` types
- Why using a moved value is a compile-time error
- Passing ownership into a function
- Returning ownership from a function
- Reading compiler errors as ownership feedback

Continue with [Move semantics](02-move-semantics.md).

### 3. Copy and Clone

- Which simple types implement `Copy`
- Why `Copy` is different from an explicit `clone()`
- The cost and intent of cloning heap-backed data
- Designing types that should or should not be copied

Continue with [Copy and Clone](03-copy-and-clone.md).

### 4. Shared borrowing

- References with `&T`
- Passing a reference without transferring ownership
- Multiple shared references at the same time
- Reference scope and the last use of a reference
- Slices such as `&str` and `&[T]`

Continue with [Shared borrowing](ownership-and-borrowing.md).

### 5. Mutable borrowing

- Mutable references with `&mut T`
- The one-mutable-reference rule
- Why mutable and shared references cannot overlap
- Reborrowing and short mutable-borrow scopes
- Designing functions that mutate through references

Continue with [Mutable borrowing](05-mutable-borrowing.md).

### 6. Borrowing collections and iterators

- Borrowing elements from `String`, `Vec<T>`, and maps
- Iterating with `iter`, `iter_mut`, and `into_iter`
- Avoiding mutation while an element is borrowed
- Returning references into collections safely

Continue with [Borrowing collections and iterators](06-borrowing-collections-and-iterators.md).

### 7. Slices and API boundaries

- Prefer `&str` over `&String` in read-only APIs
- Prefer slices such as `&[T]` over concrete collections where possible
- Choosing owned values versus borrowed parameters
- Using `AsRef`, `Borrow`, and generic bounds when they add real value

Continue with [Slices and API boundaries](07-slices-and-api-boundaries.md).

### 8. Lifetimes: describing relationships

- Why a reference needs a valid lifetime
- Lifetime annotations describe relationships, not durations
- Shared lifetime parameters in functions
- Returning a reference tied to an input
- Why lifetimes do not make data live longer

### 9. Ownership hands-on review

- Explain scope, ownership, moves, Copy, and Clone in your own words.
- Trace ownership, repair compiler errors, and complete a snapshot program.
- Iterate over borrowed slices, update a selected range, and return borrowed text.
- Combine borrowed edits and reads with a final consuming title-selection step.
- Current practice set covers selected skills from Topics 2.1–2.7.

Finish with [Ownership hands-on review](09-hands-on.md).

## Advanced continuation — planned for later modules

The headings below retain their original topic numbers for existing links.
They are no longer prerequisites for finishing Module 2. Study structs,
traits, and error handling first, then follow the [syllabus](../../syllabus.md).
Topics 9–11 belong to Module 9; Topic 12 spans Modules 9–10; Topic 13 spans
Modules 9–10; Topic 14 belongs to Module 8; Topic 15 spans Modules 10–13.

### 9. Structs that contain references

- Lifetime parameters on structs and enums
- Borrowed views versus owned data
- `'static` references and what they really mean
- When a lifetime makes an API harder to use
- Replacing a borrowed struct with an owned type when appropriate

### 10. Lifetime elision and method receivers

- The compiler's lifetime elision rules
- Why common functions need no explicit annotations
- Lifetimes in methods with `&self` and `&mut self`
- When explicit annotations improve clarity

### 11. Ownership in enums and error handling

- Moving values into and out of `Option<T>` and `Result<T, E>`
- Borrowing with `as_ref` and `as_mut`
- Matching without moving unnecessarily
- Ownership while propagating errors with `?`

### 12. Smart pointers and shared ownership

- `Box<T>` for heap allocation and recursive types
- `Rc<T>` for shared ownership in single-threaded code
- `Arc<T>` for shared ownership across threads
- `RefCell<T>` for checked-at-runtime interior mutability
- `Mutex<T>` and `RwLock<T>` for synchronized mutation

### 13. Interior mutability and ownership design

- When mutation through an immutable binding is useful
- The difference between compile-time and runtime borrow checking
- Avoiding `RefCell` borrow panics
- Combining `Arc` with `Mutex` or `RwLock`
- Choosing the simplest ownership model that fits the design

### 14. Closures, iterators, and captured ownership

- Closures borrowing from their environment
- `move` closures and ownership transfer
- `Fn`, `FnMut`, and `FnOnce`
- Returning closures and captured data
- Ownership through iterator adapters

### 15. Advanced ownership patterns

- Self-referential data and why it is difficult
- Pinning and `Pin<T>`
- Ownership across `async` tasks and `.await`
- `Send` and `Sync` as thread-safety boundaries
- FFI ownership and `unsafe` contracts
- Designing zero-copy APIs without exposing fragile lifetimes

## Recommended checkpoints

At each stage, be able to answer these questions:

1. Who owns this value right now?
2. Is this operation moving, copying, cloning, or borrowing it?
3. Can this reference outlive the value it points to?
4. Would an owned value make this API simpler?

The goal is not to add lifetime annotations everywhere. The goal is to make
ownership explicit enough that the compiler and the API communicate the same
design.
