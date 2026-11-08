# Number summary

Prerequisites: stable Rust with Cargo and rustfmt, installed in Module 0.
Run from the repository root:

```bash
cargo run --manifest-path examples/01-fundamentals/number-summary/Cargo.toml
cargo test --manifest-path examples/01-fundamentals/number-summary/Cargo.toml
cargo fmt --manifest-path examples/01-fundamentals/number-summary/Cargo.toml -- --check
```

Expected output:

```text
Number summary
count: 4, total: 12
size: multiple
positive total: true
array count: 3, total: 12
countdown: 2
countdown: 1
```

Five tests check mixed, empty, negative, and large i32 inputs plus size labels.
The i64 accumulator is intended for small inputs; sufficiently large totals
can still overflow. No external crates are required. Rust commands do not need
the course Python environment; use `uv run mkdocs build --strict` from the
repository root to build the associated lesson through the course environment.
