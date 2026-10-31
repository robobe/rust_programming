# Cargo workspace demo

From the repository root, run:

```bash
cargo fmt --all --manifest-path examples/00-first-lesson/cargo-workspace/Cargo.toml
cargo check --manifest-path examples/00-first-lesson/cargo-workspace/Cargo.toml
cargo test --manifest-path examples/00-first-lesson/cargo-workspace/Cargo.toml
cargo run --manifest-path examples/00-first-lesson/cargo-workspace/app/Cargo.toml
cargo build --release --manifest-path examples/00-first-lesson/cargo-workspace/Cargo.toml -p app
```

Expected output:

```text
Hello, workspace!
2 + 3 = 5
```
