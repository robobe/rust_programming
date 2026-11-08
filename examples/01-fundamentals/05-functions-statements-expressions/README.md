# Functions, statements, and expressions

Prerequisites: stable Rust and Module 1 Topics 1–5. No external crates are
required. Run these exact commands from the repository root. Rust runs directly;
the course `.venv` is used for documentation tooling.

```bash
rustc examples/01-fundamentals/05-functions-statements-expressions/main.rs -o /tmp/rust-fundamentals-5
/tmp/rust-fundamentals-5
```

Expected output:

```text
double: 6, clamped: 0
```

The program includes assertions checking its demonstrated behavior.
The early-return example previews Topic 6 conditions. Inputs are small enough that doubling fits i32.
