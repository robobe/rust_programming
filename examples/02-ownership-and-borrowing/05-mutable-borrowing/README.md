# Mutable borrowing demo

From the repository root, run:

```bash
rustc examples/02-ownership-and-borrowing/05-mutable-borrowing/main.rs \
  -o /tmp/rust-mutable-borrowing
/tmp/rust-mutable-borrowing
```

Expected output:

```text
score: 42
message: Rust!
```

Prerequisite: the stable Rust toolchain. The example gives functions temporary
exclusive access through `&mut`; the caller remains the owner.
