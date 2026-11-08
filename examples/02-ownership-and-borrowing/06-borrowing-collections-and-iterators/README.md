# Borrowing collections and iterators demo

From the repository root, run:

```bash
rustc examples/02-ownership-and-borrowing/06-borrowing-collections-and-iterators/main.rs \
  -o /tmp/rust-collection-borrows
/tmp/rust-collection-borrows
```

Expected output:

```text
total: 6
updated: [2, 3, 4]
consumed into new Vec: [4, 6, 8]
```

Prerequisite: the stable Rust toolchain. The example demonstrates borrowing a
slice with `iter`, changing elements with `iter_mut`, and consuming the vector
with `into_iter`.
