# Slices and API boundaries demo

From the repository root, run:

```bash
rustc examples/02-ownership-and-borrowing/07-slices-and-api-boundaries/main.rs \
  -o /tmp/rust-slices
/tmp/rust-slices
```

Expected output:

```text
first word: Rust
array sum: 10
middle sum: 5
Vec sum: 18
text: a literal
text: Rust makes views useful
```

Prerequisite: the stable Rust toolchain. The demo uses `&str`, `&[i32]`, a
sub-slice, and `AsRef<str>` without cloning or moving the source values.
