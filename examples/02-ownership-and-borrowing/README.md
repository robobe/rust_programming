# Ownership and borrowing example

From the repository root, run:

```bash
rustc examples/02-ownership-and-borrowing/main.rs -o /tmp/rust-ownership
/tmp/rust-ownership
```

Expected output:

```text
Rust has 4 letters
The owner can still use: Rust
```

Prerequisite: the stable Rust toolchain. The example demonstrates a shared
borrow and does not yet cover mutable references.
