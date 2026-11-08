# Module 1: Rust fundamentals

By the end, you will be able to write and test a small Rust program that
summarizes a list of numbers.

- Declare values and choose suitable types.
- Work with collections and text.
- Write functions, decisions, and loops.
- Run a Cargo project, test behavior, and repair compiler errors.

Prerequisite: [Module 0 setup and Hello World](../00-first-lesson/index.md).
All eight topics are published. Follow the numbered order, then complete the
number-summary exercise in Topic 8 before moving to Module 2.

---

## 1. Fundamentals overview

- Understand the edit, compile, and run cycle.
- Run and change a small program.
- Preview the module learning path.

Start with [Topic 1: Fundamentals overview](01-fundamentals-overview.md).

---

## 2. Variables, mutability, and shadowing

- Declare bindings with `let` and distinguish immutable and mutable bindings.
- Use shadowing to introduce a new binding, including one of a different type.
- Define typed constants with `const`.

Continue with [Topic 2: Variables, mutability, and shadowing](02-variables-mutability-shadowing.md).

---

## 3. Scalar types and operators

- Choose integers, floating-point numbers, Booleans, and characters.
- Understand type inference, explicit annotations, and integer division.
- Recognize overflow and use comparisons and Boolean operators.

Continue with [Topic 3: Scalar types and operators](03-scalar-types-and-operators.md).

---

## 4. Arrays, tuples, strings, and vectors

- Choose fixed-size arrays, tuples, or growable vectors.
- Index sequences, check lengths, and use basic methods.
- Destructure tuples and distinguish `String` from borrowed text.
- Recognize UTF-8 byte lengths and why strings cannot be indexed by integer.

Continue with [Topic 4: Arrays, tuples, strings, and vectors](04-arrays-tuples-strings-vectors.md).

---

## 5. Functions, statements, and expressions

- Define typed parameters and return values.
- Return a tail expression or use an early `return`.
- Explain how a semicolon affects an expression and the unit type `()`.

Continue with [Topic 5: Functions, statements, and expressions](05-functions-statements-expressions.md).

---

## 6. Conditions and pattern basics

- Use `if` and `else` for decisions and as expressions.
- Choose branches with `match`.
- Cover remaining values with a wildcard pattern.

Continue with [Topic 6: Conditions and pattern basics](06-conditions-and-patterns.md).

---

## 7. Loops and ranges

- Iterate with `for` and distinguish [exclusive and inclusive ranges](07-loops-and-ranges.md#ranges-choose-the-values-to-visit).
    - [Choose a step size with `step_by()`](07-loops-and-ranges.md#choose-a-step-size).
    - [Count in reverse with `rev()`](07-loops-and-ranges.md#count-in-reverse).
- Repeat with `while` and `loop`.
- Use `break`, `continue`, and values returned by a loop.

Continue with [Topic 7: Loops and ranges](07-loops-and-ranges.md).

---

## 8. Cargo practice, tests, and compiler feedback

- Recognize `Cargo.toml`, `Cargo.lock`, `src/main.rs`, and `target/`.
- Run, format, check, and test a small program.
- Add a function and test typical and empty inputs.
- Distinguish compiler errors from failed assertions and repair both.

Continue with [Topic 8: Cargo practice, tests, and compiler feedback](08-cargo-tests-compiler-feedback.md).

---

## Completion and next module

You are ready to continue when you can implement and test the number-summary
function, explain its tuple result, and identify why each `mut` is needed.

Next: [Module 2: Ownership and borrowing](../02-ownership-and-borrowing/index.md).
