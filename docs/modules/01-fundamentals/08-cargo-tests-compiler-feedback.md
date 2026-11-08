# Topic 8: Cargo practice, tests, and compiler feedback

By the end, you will be able to

- Run, format, check, and test a Cargo program.
- Combine functions, collections, decisions, and loops.
- Distinguish compiler diagnostics from failing assertions.

Prerequisite: [Topic 7: Loops and ranges](07-loops-and-ranges.md).

---

## Cargo project and complete demo

The demo uses this layout:

```text
number-summary/
├── Cargo.toml       # Package name, version, and Rust edition.
└── src/
    └── main.rs      # Entry point, helper functions, and unit tests.
```

Cargo generates `Cargo.lock` to record dependency resolution and `target/`
for build output. No third-party dependencies are required here.

The program collects numbers, borrows them to calculate a count and sum,
destructures the returned tuple, and prints the result.

```mermaid
flowchart LR
    A[Create numbers] --> B[Pass a borrowed slice]
    B --> C[Loop and accumulate total]
    C --> D[Return count and total tuple]
    D --> E[Print summary]
```

```rust
--8<-- "examples/01-fundamentals/number-summary/src/main.rs"
```

<div class="quiz" data-answer="b" data-explanation="Widening handles the demonstrated i32 sums, though sufficiently large totals could still overflow.">
<p>Why does the demo use an i64 accumulator?</p>
<label><input type="radio" name="fundamentals-t8-q1" value="a"> To make every possible input overflow-proof</label><br>
<label><input type="radio" name="fundamentals-t8-q1" value="b"> To widen i32 values before adding the small sample inputs</label><br>
<label><input type="radio" name="fundamentals-t8-q1" value="c"> To turn numbers into text</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

The `for &value in values` pattern copies each small integer out of its shared
reference; it does not consume the caller's collection. The accumulator uses
`i64`, so addition of the sample `i32` values fits comfortably. This is a
small-input teaching example, not an arbitrary-size numeric aggregation API.

From the repository root:

```bash
cargo run --manifest-path examples/01-fundamentals/number-summary/Cargo.toml
cargo test --manifest-path examples/01-fundamentals/number-summary/Cargo.toml
cargo fmt --manifest-path examples/01-fundamentals/number-summary/Cargo.toml -- --check
cargo check --manifest-path examples/01-fundamentals/number-summary/Cargo.toml
```

Expected program output:

```text
Number summary
count: 4, total: 12
size: multiple
positive total: true
array count: 3, total: 12
countdown: 2
countdown: 1
```

---

## Tests and compiler feedback

`#[cfg(test)]` includes the test module only in test builds. `#[test]`
identifies a test, and `assert_eq!` compares actual and expected results.
`use super::*;` makes the parent module's functions available to the tests.

The demo tests mixed values, empty input, negative values, widening before
addition, and every branch of `size_label`. Expected result: **5 tests pass**.

Try these deliberate mistakes one at a time, then undo each:

| Change | Feedback to look for | Repair |
| --- | --- | --- |
| Remove `mut` from `total` | Cannot assign twice to an immutable variable | Restore `mut` |
| Add a semicolon to the returned tuple | Mismatched return type | Restore the tail expression |
| Change an expected sum to `99` in a test | Assertion failure showing actual and expected values | Correct the expectation |

Compiler errors prevent a build. A failing assertion means the program compiled
but its behavior did not match the test. Read the first diagnostic, its source
location, and the expected/found types before changing code.

<div class="quiz" data-answer="a" data-explanation="A test assertion checks runtime behavior; a failing assertion differs from a compiler error.">
<p>What does a failed assert_eq! in a test mean?</p>
<label><input type="radio" name="fundamentals-t8-q2" value="a"> The test compiled but the observed result differed from the expectation</label><br>
<label><input type="radio" name="fundamentals-t8-q2" value="b"> The compiler cannot parse the source</label><br>
<label><input type="radio" name="fundamentals-t8-q2" value="c"> Cargo always failed to find main.rs</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Exercise and completion goal

1. Change the input values and predict the printed count and total.
2. Add a function that counts positive numbers using a loop and an `if`.
3. Test that function with mixed input and an empty slice.
4. Trigger and repair the immutable-variable and missing-return errors above.

You are ready for Module 2 when you can implement and test the summary
function, explain its tuple result, and identify why each `mut` is needed.




---

## Check your understanding

<div class="quiz" data-answer="b" data-explanation="Explicit interfaces, focused functions, and tests make behavior easier to reason about and verify.">
<p>Which approach best combines the fundamentals into a reliable program?</p>
<label><input type="radio" name="fundamentals-5" value="a"> Keep all logic in printing statements and skip empty input</label><br>
<label><input type="radio" name="fundamentals-5" value="b"> Choose types and functions deliberately, then test representative inputs and edge cases</label><br>
<label><input type="radio" name="fundamentals-5" value="c"> Use mut for every binding so the compiler accepts all changes</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Module 2: Ownership and borrowing](../02-ownership-and-borrowing/index.md). Return to the [topic index](index.md).
