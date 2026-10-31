# Values, variables, and scope demo

From the repository root, run:

```bash
rustc examples/02-ownership-and-borrowing/01-values-variables-scope/main.rs \
  -o /tmp/rust-scope-demo
/tmp/rust-scope-demo
```

Expected output:

```text
score: 42
message: Rust owns its data
inside: temporary value
outside: still in scope
```
