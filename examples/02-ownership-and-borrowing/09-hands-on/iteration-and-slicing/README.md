# Iteration and slicing: Exercises 5–6

Prerequisites: stable Rust and Module 2 Topics 4–7. These standalone Rust
programs need no dependencies. The course `.venv` is for documentation tooling,
so Rust commands run directly from the repository root.

## Exercise 5: numeric slices

```bash
rustc examples/02-ownership-and-borrowing/09-hands-on/iteration-and-slicing/numbers.rs -o /tmp/rust-review-numbers
/tmp/rust-review-numbers
```

Expected output:

```text
middle sum: 50
updated: [10, 21, 31, 40]
```

Assertions also check arrays and empty slices. Only the selected middle
elements change. Values are small `i32` numbers; the exercise does not add
overflow handling. Direct range indexing requires valid bounds.

## Exercise 6: borrowed text

```bash
rustc examples/02-ownership-and-borrowing/09-hands-on/iteration-and-slicing/text.rs -o /tmp/rust-review-text
/tmp/rust-review-text
```

Expected output:

```text
first word: Rust
still owned: Rust makes borrowing useful
```

Assertions also check a single word, empty input, whitespace-only input, and
leading whitespace with Unicode text. Words are separated by whitespace;
punctuation remains part of a word. The returned view borrows the input and
cannot outlive it.
