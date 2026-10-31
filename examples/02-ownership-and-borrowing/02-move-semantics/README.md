# Move semantics demo

From the repository root, run:

```bash
rustc examples/02-ownership-and-borrowing/02-move-semantics/main.rs \
  -o /tmp/rust-move-semantics
/tmp/rust-move-semantics
```

Expected output:

```text
after move: hello
inside function: hello
returned: ownership returned
```

Prerequisite: the stable Rust toolchain. To see the compiler's ownership
diagnostic, uncomment the `println!("{first}")` attempt described in the
lesson and compile again.
