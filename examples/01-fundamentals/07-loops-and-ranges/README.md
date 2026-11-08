# Loops and ranges

Prerequisites: stable Rust and Module 1 Topics 1–7. No external crates are
required. Run these exact commands from the repository root. Rust runs directly;
the course `.venv` is used for documentation tooling.

```bash
rustc examples/01-fundamentals/07-loops-and-ranges/main.rs -o /tmp/rust-fundamentals-7
/tmp/rust-fundamentals-7
```

Expected output:

```text
sum: 6, remaining: 0, finished: 2, odd sum: 4
4
3
2
1
```

The program includes assertions checking its demonstrated behavior.
Loops have fixed small bounds. A while loop must make progress toward a false condition.
