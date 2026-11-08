# Topic 6: Conditions and pattern basics

By the end, you will be able to

- Use **[Boolean conditions](#boolean-conditions)** and **[if expressions](#if-expressions)**.
- Match simple **[patterns](#match-patterns)** and a **[wildcard](#wildcards-and-exhaustive-matches)**.
- Explain **[exhaustive matches](#wildcards-and-exhaustive-matches)**.

Prerequisite: [Topic 5: Functions, statements, and expressions](05-functions-statements-expressions.md).

---

## Choose a branch

### Boolean conditions

`if` needs a Boolean condition. Use these operators to compare values and
combine conditions:

- **`==`** — equality: `count == 0` checks whether `count` equals zero.
- **`>`** — greater than: `count > 0` checks whether `count` is above zero.
- **`<`** — less than: `count < 10` checks whether `count` is below ten.
- **`&&`** — Boolean AND: `count > 0 && count < 10` requires both conditions to be true.
- **`||`** — Boolean OR: `count == 0 || count == 1` requires at least one condition to be true.
- **`!`** — Boolean NOT: `!enabled` reverses the Boolean value of `enabled`.

---

### `if` expressions

An `if` can produce a value:

```rust
let label = if count == 0 {
    "empty"
} else {
    "nonempty"
};
```

Its branches must have compatible types. Unlike some
languages, Rust does not treat integers as truth values: `if count` is invalid.

---

### `match` patterns

`match` chooses a branch by pattern and must cover all possible inputs.
In this example, the patterns `0` and `1` match those exact counts.
`&'static str` here means a reference to a string literal valid for the whole
program; detailed lifetime syntax comes later.

```rust
fn size_label(count: usize) -> &'static str {
    match count {
        0 => "empty",
        1 => "single",
        _ => "multiple", // Wildcard covers every remaining value.
    }
}
```

Read the symbols in this function as follows:

- **`->`** — return-type marker: `-> &'static str` declares the function's return type.
- **`=>`** — match-arm separator: `0 => "empty"` connects a pattern to the expression evaluated when it matches.
- **`_`** — wildcard pattern: `_ => "multiple"` handles any count not matched by an earlier arm.

---

### Wildcards and exhaustive matches

The **`_` wildcard pattern** matches any value not handled by an earlier arm.
It covers counts 2 and above here, making the match exhaustive: every possible
input has a branch. Without it, the compiler rejects this match because `0`
and `1` alone do not cover every `usize` value.

### Checkpoint 4

<div class="quiz" data-answer="b" data-explanation="The wildcard covers the remaining counts, making this match exhaustive.">
<p>What does the wildcard <code>_</code> do in size_label?</p>
<label><input type="radio" name="fundamentals-4" value="a"> It ignores the entire match</label><br>
<label><input type="radio" name="fundamentals-4" value="b"> It handles all values not matched by earlier arms</label><br>
<label><input type="radio" name="fundamentals-4" value="c"> It means only zero</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>
<div class="quiz" data-answer="b" data-explanation="An if condition must be Boolean; comparing count with zero produces a bool.">
<p>Which condition is valid when count is an integer?</p>
<label><input type="radio" name="fundamentals-t6-q2" value="a"> <code>if count</code></label><br>
<label><input type="radio" name="fundamentals-t6-q2" value="b"> <code>if count &gt; 0</code></label><br>
<label><input type="radio" name="fundamentals-t6-q2" value="c"> <code>if "yes"</code></label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Runnable example

```rust
--8<-- "examples/01-fundamentals/06-conditions-and-patterns/main.rs"
```

Run from the repository root:

```bash
rustc examples/01-fundamentals/06-conditions-and-patterns/main.rs -o /tmp/rust-fundamentals-6
/tmp/rust-fundamentals-6
```

Expected output:

```text
if: nonempty, match: multiple
```

---

## Exercise

Add a `2 => "pair"` arm before the wildcard, and test 0, 1, 2, and 3.
Remove the wildcard temporarily, read the exhaustiveness error, then restore
it. Rewrite the empty/nonempty decision as a small function.

## Check your understanding
<div class="quiz" data-answer="a" data-explanation="Exhaustiveness ensures there is a branch for every possible value.">
<p>Why must match cover every possible input?</p>
<label><input type="radio" name="fundamentals-t6-q3" value="a"> Every input needs a defined branch</label><br>
<label><input type="radio" name="fundamentals-t6-q3" value="b"> It must always clone the input</label><br>
<label><input type="radio" name="fundamentals-t6-q3" value="c"> It can only accept Boolean inputs</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Topic 7: Loops and ranges](07-loops-and-ranges.md). Return to the [topic index](index.md).
