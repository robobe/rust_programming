# Topic 5: Functions, statements, and expressions

By the end, you will be able to

- Define typed parameters and return values.
- Use tail expressions and early returns.
- Explain semicolons and the unit type.

Prerequisite: [Topic 4: Arrays, tuples, strings, and vectors](04-arrays-tuples-strings-vectors.md).

---

## Give behavior a name

Functions name reusable behavior. Parameters have types, and `->` introduces
the return type. An expression produces a value; a semicolon usually turns
that expression into a statement whose result is discarded.

```rust
fn double(value: i32) -> i32 {
    value * 2 // The final expression returns the result.
}

fn describe(value: i32) -> &'static str {
    if value >= 0 { "nonnegative" } else { "negative" }
}
```

An `if` can itself be an expression. Both branches here return text of the
same type. `&'static str` here refers to string literals that remain valid
for the whole program; detailed lifetime syntax comes later.
Use `return value;` when an early return makes the function clearer.

Adding a semicolon after `value * 2` makes `double` return `()`, the unit
type (“no meaningful result”), instead of the required `i32`.

### Checkpoint 3

<div class="quiz" data-answer="a" data-explanation="A trailing semicolon discards the expression result, so the declared return type is not satisfied.">
<p>What happens if double ends with <code>value * 2;</code>?</p>
<label><input type="radio" name="fundamentals-3" value="a"> Compilation fails because the function returns unit instead of i32</label><br>
<label><input type="radio" name="fundamentals-3" value="b"> The function automatically returns zero</label><br>
<label><input type="radio" name="fundamentals-3" value="c"> Rust ignores the semicolon</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

<div class="quiz" data-answer="b" data-explanation="The arrow introduces the declared return type.">
<p>What does <code>-&gt; i32</code> describe?</p>
<label><input type="radio" name="fundamentals-t5-q2" value="a"> The number of arguments</label><br>
<label><input type="radio" name="fundamentals-t5-q2" value="b"> The function return type</label><br>
<label><input type="radio" name="fundamentals-t5-q2" value="c"> The type of every local binding</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Runnable example

```rust
--8<-- "examples/01-fundamentals/05-functions-statements-expressions/main.rs"
```

Run from the repository root:

```bash
rustc examples/01-fundamentals/05-functions-statements-expressions/main.rs -o /tmp/rust-fundamentals-5
/tmp/rust-fundamentals-5
```

Expected output:

```text
double: 6, clamped: 0
```

---

## Exercise

Write `fn triple(value: i32) -> i32` and assert its results for -2, 0, and 3.
Add a semicolon to its final expression, read the diagnostic, then repair it.
In the demo, explain how the early return skips the final expression.

## Check your understanding
<div class="quiz" data-answer="c" data-explanation="Tail expressions supply return values, while a semicolon discards the expression result.">
<p>What is a useful distinction between an expression and a statement?</p>
<label><input type="radio" name="fundamentals-t5-q3" value="a"> Every statement returns an integer</label><br>
<label><input type="radio" name="fundamentals-t5-q3" value="b"> Expressions cannot appear inside functions</label><br>
<label><input type="radio" name="fundamentals-t5-q3" value="c"> An expression produces a value; a semicolon usually discards it</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Topic 6: Conditions and pattern basics](06-conditions-and-patterns.md). Return to the [topic index](index.md).
