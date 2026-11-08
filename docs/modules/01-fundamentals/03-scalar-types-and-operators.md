# Topic 3: Scalar types and operators

By the end, you will be able to

- Choose scalar types and use type annotations.
- Predict integer and floating-point division.
- Use arithmetic, comparisons, and Boolean operators.

Prerequisite: [Topic 2: Variables, mutability, and shadowing](02-variables-mutability-shadowing.md).

---

## Choose a type for the value

Rust infers many local types from their values and uses. An annotation such as
`let temperature: f64 = 21.5;` makes the choice explicit. Function parameters
require declared types. Single quotes denote a `char`, such as 'λ'; double
quotes denote text.

| Type | Meaning | Example |
| --- | --- | --- |
| `i32` | Signed 32-bit integer | `-12` |
| `u8` | Unsigned 8-bit integer | `255` |
| `usize` | Unsigned type used for lengths and indexes | `values.len()` |
| `f64` | Floating-point number | `21.5` |
| `bool` | Boolean | `true` |
| `char` | Unicode scalar value | `'λ'` |

Integer division truncates toward zero: `5 / 2` is `2`, while
`5.0 / 2.0` is `2.5`. Choose types to fit the data; integer overflow is
not automatic promotion to a larger type.

<div class="quiz" data-answer="a" data-explanation="Integer division truncates toward zero; use floating-point operands for a fractional result.">
<p>What is the value of integer division <code>5 / 2</code>?</p>
<label><input type="radio" name="fundamentals-t3-q1" value="a"> 2</label><br>
<label><input type="radio" name="fundamentals-t3-q1" value="b"> 2.5</label><br>
<label><input type="radio" name="fundamentals-t3-q1" value="c"> 3</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Arithmetic and Boolean operators

Use `+`, `-`, `*`, `/`, and `%` for arithmetic and remainder. Comparisons such
as `==`, `!=`, `<`, and `>=` produce `bool`. Combine Boolean conditions with
`&&` (AND), `||` (OR), and `!` (NOT). Assignment `=` is different from comparison
`==`. A `u8` fits 0 through 255; choose a larger type when needed. Overflow
can panic with overflow checks enabled; wrapping behavior is not promotion.
<div class="quiz" data-answer="b" data-explanation="The double equals operator compares values and produces a Boolean.">
<p>Which expression compares two values?</p>
<label><input type="radio" name="fundamentals-t3-q2" value="a"> <code>count = 3</code></label><br>
<label><input type="radio" name="fundamentals-t3-q2" value="b"> <code>count == 3</code></label><br>
<label><input type="radio" name="fundamentals-t3-q2" value="c"> <code>let count = 3</code></label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Runnable example

```rust
--8<-- "examples/01-fundamentals/03-scalar-types-and-operators/main.rs"
```

Run from the repository root:

```bash
rustc examples/01-fundamentals/03-scalar-types-and-operators/main.rs -o /tmp/rust-fundamentals-3
/tmp/rust-fundamentals-3
```

Expected output:

```text
whole: 2, fraction: 2.5, initial: λ
```

---

## Exercise

Change the count to 7 and predict the quotient and remainder. Add a comparison
checking whether the remainder is zero. Then assign 256 to a `u8`, read the
compiler error, and choose a type that can represent it.

## Check your understanding
<div class="quiz" data-answer="c" data-explanation="bool has the two values true and false and is used for conditions.">
<p>Which type represents a true/false condition?</p>
<label><input type="radio" name="fundamentals-t3-q3" value="a"> <code>char</code></label><br>
<label><input type="radio" name="fundamentals-t3-q3" value="b"> <code>usize</code></label><br>
<label><input type="radio" name="fundamentals-t3-q3" value="c"> <code>bool</code></label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Topic 4: Arrays, tuples, strings, and vectors](04-arrays-tuples-strings-vectors.md). Return to the [topic index](index.md).
