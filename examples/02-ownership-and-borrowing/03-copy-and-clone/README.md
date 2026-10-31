# Copy and Clone demo

From the repository root, run:

```bash
rustc examples/02-ownership-and-borrowing/03-copy-and-clone/main.rs \
  -o /tmp/rust-copy-clone
/tmp/rust-copy-clone
```

Expected output:

```text
attempts: 3, backup: 3
original: primary, duplicate: primary
settings: Settings { retries: 3, label: "demo" }
settings copy: Settings { retries: 3, label: "demo" }
fields: retries=3, label=demo
```

Prerequisite: the stable Rust toolchain. The example uses `Copy` implicitly for
`u8`, calls `clone()` explicitly for `String`, and derives `Clone` for a struct
that contains both a small value and a heap-backed value.
