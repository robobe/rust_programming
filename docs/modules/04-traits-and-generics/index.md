# Module 4: Traits and generics

By the end, you will be able to

- Define **[generic functions](#1-generic-functions)** and **[generic types](#2-generic-structs-enums-and-impl-blocks)**.
- Specify shared behavior with **[traits](#3-defining-and-implementing-traits)** and **[trait bounds](#4-trait-bounds-and-where-clauses)**.
- Choose **[derived traits](#5-common-standard-traits-and-derive)** and understand **[associated types](#6-associated-types-and-implementation-rules)**.
- Compare **[impl Trait](#7-impl-trait-and-static-dispatch)** with **[dyn Trait](#8-dyn-trait-and-dynamic-dispatch)**.
- Build and test a **[generic stack](#9-hands-on-a-generic-stack)** and **[shared-behavior example](#10-hands-on-one-trait-two-types)**.

Prerequisite: [Module 3: Structs, enums, and data modeling](../03-structs-enums-and-data-modeling/index.md).
**Topic 1 is published; Topics 2–10 remain planned.**
Start with Topic 1. The other sections describe lessons to be implemented.

---

## 1. Generic functions

- Use type parameters to reuse behavior across concrete types.
- Read `<T>`, type inference, and explicit type arguments with `::<T>`.
- Distinguish generic types from values supplied as function arguments.
- Notice that arbitrary operations on `T` need capabilities declared by bounds.

Study [Topic 1: Generic functions](01-generic-functions.md).

---

## 2. Generic structs, enums, and impl blocks

- Define a small container with `struct Container<T>`.
- Write `impl<T> Container<T>` methods and a concrete specialized impl.
- Revisit `Option<T>` and `Result<T, E>` as generic enums.
- Choose borrowed or owned parameters and return values deliberately.

---

## 3. Defining and implementing traits

- Describe a shared behavior with `trait` and implement it with `impl Trait for Type`.
- Distinguish inherent methods from trait methods.
- Write required methods and a default method.
- Reuse `&self`, `&mut self`, and `self` receiver choices from Module 3.
- Introduce a small reporting trait for a task and a second domain type.

---

## 4. Trait bounds and where clauses

- Express required capabilities with `T: Trait`.
- Combine bounds and improve readability with `where` clauses.
- Call trait methods inside a generic function.
- Use borrowed arguments when ownership transfer is unnecessary.
- Avoid adding `Copy` or `Clone` bounds merely to silence ownership errors.

---

## 5. Common standard traits and derive

- Use `#[derive(Debug, Clone, Default, PartialEq)]` when fields support them.
- Distinguish `Copy` from explicit `Clone` and revisit their ownership effects.
- Compare debug output with `Display` for deliberate user-facing formatting.
- Introduce `Eq`, `PartialOrd`, and `Ord` for equality and ordering contracts.
- Understand that deriving a trait adds behavior rather than changing a type's purpose.

---

## 6. Associated types and implementation rules

- Read a small trait with `type Item` and return `Self::Item`.
- Contrast an associated type with a trait's generic parameter.
- Understand the introductory orphan rule: implementing a foreign trait requires
  an appropriate local type, subject to Rust's coherence rules.
- Recognize conflicting implementations and avoid overlapping blanket impls.
- Defer custom `Iterator` implementation to Module 8 and the Newtype pattern to Module 6.

---

## 7. impl Trait and static dispatch

- Compare a generic bound with argument-position `impl Trait`.
- Return an opaque concrete implementation with return-position `impl Trait`.
- Understand that the returned concrete type must be consistent across branches.
- Explain static dispatch and monomorphization at an introductory level.
- Prefer a simple concrete signature when polymorphism has no practical benefit.

---

## 8. dyn Trait and dynamic dispatch

- Call shared behavior through borrowed `&dyn Trait` values.
- Contrast runtime dispatch with generic static dispatch.
- Use references to different implementors in a small heterogeneous collection.
- Introduce dyn compatibility: not every trait can be used as a trait object.
- Leave owned `Box<dyn Trait>` designs and detailed lifetime rules to Module 9.

---

## 9. Hands-on: a generic stack

- Generalize the Module 3 integer stack into `Stack<T>` backed by `Vec<T>`.
- Define `new`, `push`, `pop`, `peek`, `len`, and `is_empty` with small interfaces.
- Return `Option<T>` from `pop` and `Option<&T>` from `peek`.
- Move values rather than requiring `T: Clone` or `T: Copy`.
- Test LIFO order, empty operations, interleaved operations, and owned `String` values.
- Explain amortized constant-time push, constant-time pop/peek, and linear storage.

The prerequisite integer-stack exercise and this generic extension are tracked
in `design/learning/data-structures-and-patterns-practice.md`. Both remain planned.
Provide the integer-stack prerequisite before implementing this exercise.

---

## 10. Hands-on: one trait, two types

- Implement the reporting trait for a task and a second type.
- Use both through one borrowed generic function.
- Compare the same operation through `&dyn Trait` without changing its behavior.
- Test each implementation and the shared caller with ordinary and empty text.
- Include runnable examples, contextual quizzes, and collapsed exercise solutions.

---

## Completion goal and next module

Implement one trait for two types and use both through a generic function.
Build a stack that stores integers and owned strings without unnecessary bounds,
then explain the ownership and dispatch choices in both exercises.

Next: [Module 5: Error handling, modules, and testing](../../syllabus.md#5-error-handling-modules-and-testing-planned)
(planned). Requirement-driven design patterns follow in Module 6.
