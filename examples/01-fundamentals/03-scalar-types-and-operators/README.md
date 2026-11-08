# Scalar types and operators

Prerequisites: stable Rust and Module 1 Topics 1–3. No external crates are
required. Run these exact commands from the repository root. Rust runs directly;
the course `.venv` is used for documentation tooling.

```bash
rustc examples/01-fundamentals/03-scalar-types-and-operators/main.rs -o /tmp/rust-fundamentals-3
/tmp/rust-fundamentals-3
```

Expected output:

```text
whole: 2, fraction: 2.5, initial: λ
```

The program includes assertions checking its demonstrated behavior.
Operands are fixed and nonzero where used as divisors. This is not an overflow-handling API.
