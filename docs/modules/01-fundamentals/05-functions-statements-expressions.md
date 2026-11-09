# Topic 5: Functions, statements, and expressions

By the end, you will be able to

- Define **[functions, typed parameters, and return types](#functions-parameters-and-return-types)**.
- Return a value using a **[tail expression](#tail-expressions-return-the-final-value)** or an **[explicit return](#explicit-return-and-early-exits)**.
- Distinguish **[statements and expressions](#statements-expressions-and-unit)** and explain **[semicolons and unit ()](#statements-expressions-and-unit)**.

Prerequisite: [Topic 4: Arrays, tuples, strings, and vectors](04-arrays-tuples-strings-vectors.md).
Snippets that declare helper functions can be placed before `main`; calls and
local bindings belong inside `main`. The complete demo below is runnable as-is.

---

## Functions, parameters, and return types

A function names reusable behavior. Parameters name the inputs it receives;
a return value passes a result back to the caller. The return type states
what kind of value the caller can expect.

```rust
fn double(value: i32) -> i32 {
    value * 2
}
```

- **`fn`** — function keyword: `fn double` defines a function named `double`.
- **`:`** — parameter type marker: `value: i32` declares an integer parameter.
- **`->`** — return-type marker: `-> i32` declares an integer result.
- **`{ ... }`** — function body: `{ value * 2 }` contains the work to perform.

At the call `double(3)`, the argument `3` supplies the parameter's value.
The result is `6`; merely defining the function does not run its body.
<div class="quiz" data-answer="b" data-explanation="The arrow introduces the return type; parameter types are declared separately.">
<p>What does <code>-&gt; i32</code> tell the caller?</p>
<label><input type="radio" name="fundamentals-t5-review-1" value="a"> Every local binding must be i32</label><br>
<label><input type="radio" name="fundamentals-t5-review-1" value="b"> The function returns an i32 value</label><br>
<label><input type="radio" name="fundamentals-t5-review-1" value="c"> The function accepts exactly 32 arguments</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Tail expressions: return the final value

An expression evaluates to a value: `value * 2` produces a number, and
`double(3)` produces the function's result. A **tail expression** is the final
expression in a block, written **without a trailing semicolon**. Its value
becomes the value of that block. At the end of a function body, it is returned
to the caller.

!!! important "The final expression is the returned value"
    ```rust hl_lines="2"
    fn double(value: i32) -> i32 {
        value * 2
    }
    ```

    **`value * 2` has no trailing `;`, so its result is returned.**
    With `double(3)`, the expression produces `6`, which goes back to the caller.

```rust
let result = double(3);
println!("result: {result}"); // result: 6
```

The semicolon after `let result = double(3);` ends the caller's declaration;
it does not change the function's return. The result has already been stored
in `result`. Returning and printing are separate operations: `double` returns
a number; `println!` displays it.
<div class="quiz" data-answer="a" data-explanation="The function evaluates value * 2 and returns 6, which initializes result.">
<p>In <code>let result = double(3);</code>, what supplies the value stored in result?</p>
<label><input type="radio" name="fundamentals-t5-review-2" value="a"> The value 6 from double’s final expression</label><br>
<label><input type="radio" name="fundamentals-t5-review-2" value="b"> The text printed by println!</label><br>
<label><input type="radio" name="fundamentals-t5-review-2" value="c"> The semicolon after the call</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Statements, expressions, and unit

A `let` declaration is a statement that introduces a binding. An expression
can be used inside that statement to compute its initial value. Appending a
semicolon to an ordinary expression such as `value * 2` makes it an expression
statement whose result is discarded.

- **`*`** — multiplication: `value * 2` produces twice the integer value.
- **`;`** — statement terminator: `value * 2;` evaluates and discards that result.
- **`()`** — unit type and value: a block without a tail expression normally evaluates to `()`.

This version is deliberately incorrect and **does not compile**:

```rust
fn double(value: i32) -> i32 {
    value * 2;
}
```

After discarding the multiplication result, the body has no tail expression
and yields unit `()`. That does not satisfy the declared `i32` return type.
The compiler reports **E0308: mismatched types**. Remove the final semicolon
to restore the tail expression.

A function without a declared return type normally returns unit. For example,
`fn announce() { println!("Ready"); }` prints a message but returns no useful
payload. The last expression must match the return type; being last does not
turn printed text into a returned string.
<div class="quiz" data-answer="c" data-explanation="The semicolon discards the integer result, leaving no tail expression to satisfy -&gt; i32.">
<p>What happens if double ends with <code>value * 2;</code> and has no explicit return?</p>
<label><input type="radio" name="fundamentals-t5-review-3" value="a"> The caller receives zero</label><br>
<label><input type="radio" name="fundamentals-t5-review-3" value="b"> The semicolon is ignored</label><br>
<label><input type="radio" name="fundamentals-t5-review-3" value="c"> Compilation fails: the body yields unit instead of i32</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Explicit return and early exits

`return expression;` explicitly sends a value to the caller and exits the
function immediately. `fn double(value: i32) -> i32 { return value * 2; }`
returns the same result as the tail-expression version. **A semicolon after
`return value * 2;` does not discard the returned value**: the `return` has
already transferred control and supplied the result.

Use an early return when one case should finish before the rest of the body:

```rust
fn nonnegative(value: i32) -> i32 {
    if value < 0 {
        return 0;
    }
    value
}
```

For `nonnegative(-2)`, `return 0;` exits immediately, so the final `value` is
not evaluated. For `nonnegative(3)`, that branch is skipped and the tail
expression returns 3. The comparison `<` means “less than”; Topic 6 develops
Boolean conditions and `if` in detail.
<div class="quiz" data-answer="b" data-explanation="return 0 exits the function; execution never reaches the final value expression for a negative input.">
<p>What does <code>nonnegative(-2)</code> return?</p>
<label><input type="radio" name="fundamentals-t5-review-4" value="a"> -2, because value is the last expression</label><br>
<label><input type="radio" name="fundamentals-t5-review-4" value="b"> 0, because the early return exits before the tail expression</label><br>
<label><input type="radio" name="fundamentals-t5-review-4" value="c"> Both 0 and -2</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

### A block can return an expression too

The same rule applies to nested blocks:

```rust
let result = {
    let base = 3;
    base * 2
};
println!("block result: {result}"); // block result: 6
```

The block's final `base * 2` supplies 6. The semicolon after the closing brace
ends the outer `let` statement; it does not discard the block's tail value.
An `if` can also be an expression, with compatible branch values:

```rust
fn describe(value: i32) -> &'static str {
    if value >= 0 { "nonnegative" } else { "negative" }
}
```

Here the entire `if` is the function's tail expression. `>=` means “greater
than or equal to”. `&'static str` refers here to string literals that remain
valid for the whole program; detailed lifetime syntax comes later. Topic 6
explains how to choose and combine branches.

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

## Exercises

Try each task before opening its solution. Each solution is a standalone
program: save it in a temporary `.rs` file, compile with `rustc`, and run it.

### 1. Return a calculated value

Write `fn triple(value: i32) -> i32` using a tail expression. Assert its
results for -2, 0, and 3, then print the result for 3.

??? success "Solution: return the multiplication expression"
    ```rust
    fn triple(value: i32) -> i32 {
        value * 3
    }

    fn main() {
        assert_eq!(triple(-2), -6);
        assert_eq!(triple(0), 0);
        assert_eq!(triple(3), 9);
        println!("triple: {}", triple(3));
    }
    ```

    The final multiplication has no semicolon, so its integer result is
    returned. Expected output: `triple: 9`. Keep test values small enough
    that multiplication fits `i32`.

---

### 2. Diagnose the extra semicolon

Add a semicolon after the multiplication in `triple`. Compile, explain the
error, then repair it. Also show a valid version using explicit `return`.

??? success "Solution: distinguish a discarded result from return"
    `value * 3;` leaves the body yielding `()` instead of `i32`, producing
    E0308. Removing that semicolon repairs the tail expression. An explicit
    return is also valid, including its semicolon:

    ```rust
    fn triple(value: i32) -> i32 {
        return value * 3;
    }

    fn main() {
        assert_eq!(triple(3), 9);
        println!("explicit return: {}", triple(3));
    }
    ```

    Expected output: `explicit return: 9`.

---

### 3. Trace the early return

Explain why `nonnegative(-2)` skips the tail expression. Add assertions for
negative, zero, and positive inputs.

??? success "Solution: check both return paths"
    ```rust
    fn nonnegative(value: i32) -> i32 {
        if value < 0 {
            return 0;
        }
        value
    }

    fn main() {
        assert_eq!(nonnegative(-2), 0);
        assert_eq!(nonnegative(0), 0);
        assert_eq!(nonnegative(3), 3);
        println!("clamped: {}", nonnegative(-2));
    }
    ```

    Negative input reaches `return 0;` and exits immediately. Zero and
    positive input reach the final `value` expression.
    Expected output: `clamped: 0`.

---

## Check your understanding
<div class="quiz" data-answer="c" data-explanation="Both tail expressions and explicit return can provide a function result; printing is a separate effect.">
<p>Which statement correctly describes function results?</p>
<label><input type="radio" name="fundamentals-t5-review-5" value="a"> Functions return their printed output automatically</label><br>
<label><input type="radio" name="fundamentals-t5-review-5" value="b"> Every function must use the return keyword</label><br>
<label><input type="radio" name="fundamentals-t5-review-5" value="c"> A tail expression supplies the body’s value, while return exits explicitly with a value</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Topic 6: Conditions and pattern basics](06-conditions-and-patterns.md). Return to the [topic index](index.md).
