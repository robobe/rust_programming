# Advanced owned-title pipeline: Exercise 7

Prerequisites: stable Rust and Module 2 Topics 2, 5, 6, and 7. No dependencies
are required. Run from the repository root; Rust runs directly, while the
course `.venv` is used for documentation tooling.

```bash
rustc examples/02-ownership-and-borrowing/09-hands-on/owned-title-pipeline/main.rs -o /tmp/rust-title-pipeline
/tmp/rust-title-pipeline
```

Expected output:

```text
selected bytes: 10
before move: ["Read", "Code!", "Test!", "Go"]
kept: ["Code!", "Test!"]
```

Assertions check the selected range, unchanged outer titles, stable output
order, exact length threshold, empty input, no matches, zero threshold, and
UTF-8 byte length. `String::len()` counts bytes, not characters. Slicing
requires valid bounds. The returned vector may allocate storage, but the
strings are moved into it without cloning their text.
