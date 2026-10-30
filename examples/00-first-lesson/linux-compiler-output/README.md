# Linux compiler output demo

From the repository root, run:

```bash
cargo build --manifest-path examples/00-first-lesson/linux-compiler-output/Cargo.toml
cargo build --manifest-path examples/00-first-lesson/linux-compiler-output/Cargo.toml --release
cargo run --manifest-path examples/00-first-lesson/linux-compiler-output/Cargo.toml
```

Inspect the release binary with:

```bash
file examples/00-first-lesson/linux-compiler-output/target/release/linux-compiler-output
ldd examples/00-first-lesson/linux-compiler-output/target/release/linux-compiler-output
```
