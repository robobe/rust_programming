# Topic 2: Variables, mutability, and shadowing

By the end, you will be able to

- Declare **[immutable bindings](#immutable-bindings)** and **[mutable bindings](#mutable-bindings-and-mutation)**.
- Distinguish **[mutation](#mutable-bindings-and-mutation)** from **[shadowing](#shadowing)**.
- Define a **[typed constant](#typed-constants)** with **[const](#typed-constants)**.

Prerequisite: [Topic 1: Fundamentals overview](01-fundamentals-overview.md).

---

## Immutable bindings

A binding associates a name with a value. `let` introduces a binding, which is
immutable by default: you cannot assign a different value to that same binding
unless you declare it mutable.

```rust
let attempts: u8 = 2;
println!("{attempts}"); // 2
```

Rust infers many local types. Here `: u8` explicitly chooses an unsigned
8-bit integer; Topic 3 explains scalar types in detail. The short snippets on
this page belong inside `main`; the complete runnable example follows.

Read the syntax one symbol at a time:

- **`:`** — type-annotation marker: `attempts: u8` declares the binding's type.
- **`=`** — initializer separator in a `let` declaration: `let attempts = 2;` gives the new binding its initial value. In `attempts = 3;`, it is assignment to an existing mutable binding.
- **`;`** — statement terminator: `let attempts = 2;` ends the declaration.

---

## Mutable bindings and mutation

Use `mut` when a binding needs to hold a different value later. Mutation changes
that binding's value, but its type stays the same.

```rust
let mut attempts: u8 = 2;
attempts += 1;
println!("{attempts}"); // 3
```

- **`+=`** — addition assignment: `attempts += 1` increases the stored value by one.

Removing `mut` makes the assignment invalid. Adding `mut` does not let you
replace the integer with text: `attempts = String::from("three");` still fails
to compile because the type is different.

<div class="quiz" data-answer="b" data-explanation="Mutation changes a value of the same type; shadowing introduces a new binding.">
<p>Can assigning to a mutable integer binding change its type to text?</p>
<label><input type="radio" name="fundamentals-t2-q2" value="a"> Yes, mut allows any type</label><br>
<label><input type="radio" name="fundamentals-t2-q2" value="b"> No, assignment preserves the binding’s type</label><br>
<label><input type="radio" name="fundamentals-t2-q2" value="c"> Only if the text contains digits</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Shadowing

A second `let` creates a new binding. Reusing the same name shadows the earlier
binding: later uses of that name refer to the new one. The new binding may have
a different type and does not require the earlier binding to be mutable.

```rust
let attempts: u8 = 3;
let attempts = attempts.to_string();
println!("{attempts}"); // 3, now stored as a String
```

`to_string()` creates text from the integer. The printed text looks the same,
but the second binding's type is `String`, not `u8`. The new binding is itself
immutable unless its own declaration includes `mut`.

<div class="quiz" data-answer="b" data-explanation="Shadowing introduces a new binding, which may have a different type.">
<p>Why is the final <code>let attempts = attempts.to_string()</code> allowed?</p>
<label><input type="radio" name="fundamentals-1" value="a"> mut permits any type change</label><br>
<label><input type="radio" name="fundamentals-1" value="b"> A new binding shadows the old one</label><br>
<label><input type="radio" name="fundamentals-1" value="c"> All variables are strings internally</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Typed constants

A constant gives a name to a compile-time value. Use `const`, an explicit type,
and an initializer that can be evaluated at compile time. Conventionally,
constant names use uppercase words separated by underscores.

```rust
const LIMIT: usize = 10;
println!("limit: {LIMIT}"); // limit: 10
```

A constant cannot be declared with `mut`; it is not an updateable counter.
`usize` is an unsigned integer type used for lengths and indexes. The demo
compares its counter with this limit; it does not automatically enforce a limit
on all future updates.

<div class="quiz" data-answer="a" data-explanation="A const declaration requires an explicit type and a compile-time initializer; it cannot use mut.">
<p>Which declaration correctly defines a constant limit?</p>
<label><input type="radio" name="fundamentals-t2-constant" value="a"> <code>const LIMIT: usize = 10;</code></label><br>
<label><input type="radio" name="fundamentals-t2-constant" value="b"> <code>const mut LIMIT: usize = 10;</code></label><br>
<label><input type="radio" name="fundamentals-t2-constant" value="c"> <code>const LIMIT = 10;</code></label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Runnable example

```rust
--8<-- "examples/01-fundamentals/02-variables-mutability-shadowing/main.rs"
```

Run from the repository root:

```bash
rustc examples/01-fundamentals/02-variables-mutability-shadowing/main.rs -o /tmp/rust-fundamentals-2
/tmp/rust-fundamentals-2
```

Expected output:

```text
attempts: 3, limit: 10
```

---

## Exercise

Attempt each task before opening its solution. Each solution is a standalone
program; save it in a temporary `.rs` file, compile with `rustc`, and run it.

### 1. Change the counter

Start the runnable example's counter at 4, increase it by 2, and update the
assertions for both the integer and its shadowing text binding.

??? success "Solution: update the value and assertions"
    ```rust
    fn main() {
        const LIMIT: usize = 10;
        let mut attempts: usize = 4;
        attempts += 2;
        assert_eq!(attempts, 6);
        assert!(attempts < LIMIT);
        let attempts = attempts.to_string();
        assert_eq!(attempts, "6");
        println!("attempts: {attempts}, limit: {LIMIT}");
    }
    ```

    The integer becomes 6. After shadowing, the new binding holds the text
    `"6"`, so its assertion must compare against text rather than an integer.

    Expected output:

    ```text
    attempts: 6, limit: 10
    ```

---

### 2. Diagnose an immutable assignment

Remove `mut` from the counter declaration, compile, and read the error.
Then restore it and confirm the program runs again.

??? success "Solution: restore mut before assigning"
    Removing `mut` makes `attempts += 2` fail with **E0384: cannot assign twice
    to immutable variable**. `+=` updates an existing binding, so that binding
    must be mutable. This corrected program compiles:

    ```rust
    fn main() {
        let mut attempts: usize = 4;
        attempts += 2;
        assert_eq!(attempts, 6);
        println!("attempts: {attempts}");
    }
    ```

    Expected output:

    ```text
    attempts: 6
    ```

---

### 3. Explain why shadowing needs let

Why does converting the counter to text still need a new `let`, even when
the original binding is mutable? Demonstrate both mutation and shadowing.

??? success "Solution: a new binding can have a new type"
    ```rust
    fn main() {
        let mut attempts: usize = 4;
        attempts += 2;
        assert_eq!(attempts, 6);
        let attempts = attempts.to_string();
        assert_eq!(attempts, "6");
        println!("text: {attempts}");
    }
    ```

    `mut` allows changes to the existing `usize` value, not its type.
    Assigning `attempts = attempts.to_string();` would fail with **E0308:
    mismatched types**, because the right-hand side is a `String`.
    A new `let` creates a shadowing binding whose type can be `String`.

    Expected output:

    ```text
    text: 6
    ```

---

## Check your understanding

<div class="quiz" data-answer="c" data-explanation="mut permits value changes within one binding's type. A new let can shadow that binding with a different type, while const names a compile-time value.">
<p>Which summary correctly distinguishes mutation, shadowing, and constants?</p>
<label><input type="radio" name="fundamentals-t2-q3" value="a"> Mutation can change a binding's type, and const creates a mutable counter</label><br>
<label><input type="radio" name="fundamentals-t2-q3" value="b"> Shadowing updates the original binding without creating a new one</label><br>
<label><input type="radio" name="fundamentals-t2-q3" value="c"> Mutation keeps the type, shadowing creates a new binding, and a constant names a compile-time value</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Topic 3: Scalar types and operators](03-scalar-types-and-operators.md). Return to the [topic index](index.md).
