# Ownership hands-on: selected skills from Topics 2.1–2.7

Prerequisite: stable Rust and Module 2 Topics 1–3. Work in `starter.rs`.
From the repository root:

```bash
rustc examples/02-ownership-and-borrowing/09-hands-on/starter.rs -o /tmp/rust-ownership-practice
/tmp/rust-ownership-practice
```

The starter intentionally contains `todo!()` placeholders: compilation may warn
about unreachable code, and execution panics until you complete it. Once
completed, both assertions must pass and output must be:

```text
snapshot: Read Rust, updated: Read Rust!
```

After attempting the challenge, check the reference solution:

```bash
rustc examples/02-ownership-and-borrowing/09-hands-on/solution.rs -o /tmp/rust-ownership-solution
/tmp/rust-ownership-solution
```

Use `rustc` directly for these Rust exercises. The course `.venv` is used for
documentation tooling. Exercises 1–4 practice Topics 1–3. The reference solution
also asserts the empty-title case without printing additional output.

Exercises 5–6 practice borrowed iteration, mutable sub-slices, and borrowed
text. See [iteration and slicing](iteration-and-slicing/README.md) for their
prerequisites, exact commands, expected output, and limitations.
Lifetime annotations remain outside this practice set.

Exercise 7 combines borrowed edits, shared reads, and ownership-consuming
iteration. See the [owned-title pipeline](owned-title-pipeline/README.md) for
prerequisites, commands, expected output, and limitations.
