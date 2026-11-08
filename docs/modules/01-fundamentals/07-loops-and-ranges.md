# Topic 7: Loops and ranges

By the end, you will be able to

- Use **[exclusive and inclusive ranges](#ranges-choose-the-values-to-visit)** and **[reverse iteration](#count-in-reverse)**.
    - Choose a **[step size with `step_by()`](#choose-a-step-size)**.
    - Count backward with **[`rev()`](#count-in-reverse)**.
- Choose **[for](#for-visit-each-value)**, **[while](#while-repeat-while-a-condition-is-true)**, and **[loop](#loop-repeat-until-break)**.
- Control repetition with **[break](#break-and-continue)** and **[continue](#focus-on-continue)**.

Prerequisite: [Topic 6: Conditions and pattern basics](06-conditions-and-patterns.md).

---

## Repeat a step

Use `for` to visit items or a range, `while` to repeat while a condition
holds, and `loop` to repeat until an explicit `break`.

### Ranges: choose the values to visit

A range describes a sequence of values using a start and an end. In integer
loops, values advance by one. The punctuation determines whether the end is
included; the start is included in both forms below.

| Range | Values visited | Meaning |
| --- | --- | --- |
| `1..4` | 1, 2, 3 | Excludes the end, 4 |
| `1..=4` | 1, 2, 3, 4 | Includes the end, 4 |
| `0..3` | 0, 1, 2 | Three values starting at zero |
| `3..3` | None | An exclusive range with equal bounds is empty |
| `3..=3` | 3 | An inclusive range with equal bounds visits once |

Use an exclusive range for zero-based positions: a collection of length 3 has
indices `0..3`. Use an inclusive range when the final value itself belongs in
the calculation, such as summing 1 through 5.

Writing the larger number first does **not** make a countdown: `4..1` is empty.
To visit the same values backward, reverse an ascending range with `.rev()`.
These examples use integer ranges; ranges do not automatically interpret
floating-point bounds as a series of fractional steps.

!!! note "More iterator methods later in the course"
    Ranges support **iterator methods** beyond `rev()` and `step_by()`.
    [Module 2, Topic 6](../02-ownership-and-borrowing/06-borrowing-collections-and-iterators.md)
    introduces `sum()`, `map()`, and `collect()` through collection iterators.
    [Module 8: Closures and advanced iterators](../../syllabus.md#8-closures-and-advanced-iterators-planned)
    will cover these in depth, plus `filter()`, `take()`, `skip()`,
    `enumerate()`, and `count()`. Module 8 is planned; for now, focus on ranges,
    stepping, and the three loop forms below.

### Choose a step size

Use **`.step_by(2)`** to visit every second value instead of every value:

```rust
for number in (0..=10).step_by(2) {
    println!("{number}"); // 0, 2, 4, 6, 8, 10, each on its own line
}
```

For a countdown, reverse the range before stepping:

```rust
for number in (0..=10).rev().step_by(2) {
    println!("{number}"); // 10, 8, 6, 4, 2, 0, each on its own line
}
```

The step is a positive `usize`: **`.step_by(0)` panics**. Use `.rev()` for
backward iteration rather than a negative step. The first value is visited,
then the iterator skips values according to the step; the end is reached only
if it falls on a step. For example, `(0..=5).step_by(2)` visits 0, 2, 4.

### Count in reverse

!!! tip "Countdown: count backward with .rev()"
    **Use `.rev()` on an ascending range to count backward.**
    `(1..=3).rev()` visits **3, 2, 1**, including both endpoints.

    ```rust
    for number in (1..=3).rev() {
        println!("{number}");
    }
    ```

    Output:

    ```text
    3
    2
    1
    ```

The parentheses group the range before calling `.rev()`. Compare
`(1..4).rev()`: because 4 is excluded, it visits 3, 2, 1. Reversing an empty
range still visits nothing. The complete runnable example below also checks
the countdown order with an assertion.


---

### `for`: visit each value

Use `for` when you want to visit each value in a range or collection.
The range `1..=3` includes 3; `1..3` stops before 3.

```rust
let mut total = 0;
for number in 1..=3 {
    total += number;
}
println!("{total}"); // 6
```

<div class="quiz" data-answer="b" data-explanation="The end of a .. range is excluded; ..= includes it.">
<p>Which values does <code>1..3</code> visit?</p>
<label><input type="radio" name="fundamentals-t7-q1" value="a"> 1, 2, 3</label><br>
<label><input type="radio" name="fundamentals-t7-q1" value="b"> 1, 2</label><br>
<label><input type="radio" name="fundamentals-t7-q1" value="c"> 0, 1, 2</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

### `while`: repeat while a condition is true

`while` checks its condition before each pass. This countdown runs twice;
starting at zero would skip the body entirely.

```rust
let mut remaining = 2;
while remaining > 0 {
    remaining -= 1;
}
println!("{remaining}"); // 0
```

Ensure the condition can change, or the loop may never finish.

---

### `loop`: repeat until `break`

`loop` repeats without a condition at the top. Use `break` to exit it.
Here, `break attempts` also supplies the value assigned to `finished`.

```rust
let mut attempts = 0;
let finished = loop {
    attempts += 1;
    if attempts == 2 {
        break attempts; // A loop can return a value.
    }
};
println!("{finished}"); // 2
```

---

### `break` and `continue`

`continue` skips to the next iteration; `break` exits the loop. The snippets
above belong inside `main`; the complete runnable program follows.

---

## Runnable example

```rust
--8<-- "examples/01-fundamentals/07-loops-and-ranges/main.rs"
```

Run from the repository root:

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

---

## Focus on `continue`

Here is just the odd-number loop from the example. `%` gives the remainder,
so `number % 2 == 0` checks whether the number is even.

```rust hl_lines="4"
let mut odd_total = 0;
for number in 1..=4 {
    if number % 2 == 0 {
        continue;
    }
    odd_total += number;
}
```

<div class="quiz" data-answer="c" data-explanation="continue advances to the next iteration, so even numbers are not added.">
<p>When <code>number</code> is 2, what does <code>continue</code> do?</p>
<label><input type="radio" name="fundamentals-t7-q2" value="a"> Stops the loop, so 3 and 4 are never visited</label><br>
<label><input type="radio" name="fundamentals-t7-q2" value="b"> Adds 2 to odd_total, then moves on to 3</label><br>
<label><input type="radio" name="fundamentals-t7-q2" value="c"> Skips odd_total += number for 2, then starts the next pass with number equal to 3</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Exercise

Try each task before expanding its solution. Each solution is a standalone
program: save it in a temporary `.rs` file, compile with `rustc`, and run it.

### 1. Sum an inclusive range

Sum 1 through 5 and assert that the result is 15.

??? success "Solution: sum 1 through 5"
    ```rust
    fn main() {
        let mut total = 0;
        for number in 1..=5 {
            total += number;
        }
        assert_eq!(total, 15);
        println!("total: {total}");
    }
    ```

    `..=5` includes 5, so the sum is `1 + 2 + 3 + 4 + 5 = 15`.
    Expected output: `total: 15`.

### 2. Sum only odd numbers

Sum only the odd numbers from 1 through 5 and assert that the result is 9.
Use `continue` to skip even numbers.

??? success "Solution: skip even numbers"
    ```rust
    fn main() {
        let mut total = 0;
        for number in 1..=5 {
            if number % 2 == 0 {
                continue;
            }
            total += number;
        }
        assert_eq!(total, 9);
        println!("odd total: {total}");
    }
    ```

    For 2 and 4, `continue` skips the addition. Only `1 + 3 + 5` contributes
    to the sum. Expected output: `odd total: 9`.

### 3. Iterate over an empty range

Try an empty range such as `3..3` and check that the accumulator stays zero.

??? success "Solution: an empty range adds nothing"
    ```rust
    fn main() {
        let mut total = 0;
        for number in 3..3 {
            total += number;
        }
        assert_eq!(total, 0);
        println!("empty-range total: {total}");
    }
    ```

    `3..3` starts at 3 but excludes its end, also 3. There are no values to
    visit, so the body never runs. Expected output: `empty-range total: 0`.

### 4. Start the countdown at zero

Change the countdown start to 0 and confirm the loop runs zero times.
Count loop iterations to verify this directly.

??? success "Solution: check the condition before the first pass"
    ```rust
    fn main() {
        let mut remaining = 0;
        let mut iterations = 0;
        while remaining > 0 {
            iterations += 1;
            remaining -= 1;
        }
        assert_eq!(remaining, 0);
        assert_eq!(iterations, 0);
        println!("remaining: {remaining}, iterations: {iterations}");
    }
    ```

    `while` checks `remaining > 0` before entering the body. It is false
    immediately, so neither the counter nor `remaining` changes.
    Expected output: `remaining: 0, iterations: 0`.

---

## Check your understanding
<div class="quiz" data-answer="a" data-explanation="while checks its Boolean condition before each iteration; it may run zero times.">
<p>Which loop fits repetition while a changing condition remains true?</p>
<label><input type="radio" name="fundamentals-t7-q3" value="a"> while</label><br>
<label><input type="radio" name="fundamentals-t7-q3" value="b"> match</label><br>
<label><input type="radio" name="fundamentals-t7-q3" value="c"> const</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Topic 8: Cargo practice, tests, and compiler feedback](08-cargo-tests-compiler-feedback.md). Return to the [topic index](index.md).
