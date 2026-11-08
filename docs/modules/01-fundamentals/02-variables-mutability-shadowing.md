# Topic 2: Variables, mutability, and shadowing

By the end, you will be able to

- Declare immutable and mutable bindings.
- Distinguish mutation from shadowing.
- Define a typed constant.

Prerequisite: [Topic 1: Fundamentals overview](01-fundamentals-overview.md).

---

## Names and change

A variable gives a value a name. Rust infers many types, but function interfaces
and ambiguous values need explicit types. A binding is immutable by default.

```rust
let mut attempts: u8 = 2;
attempts += 1;             // Mutation requires mut.
let attempts = attempts.to_string(); // Shadowing creates a new binding.
println!("{attempts}");
```

Shadowing with another `let` can change the type; assigning to an existing
`mut` binding cannot change its type. Use `const LIMIT: usize = 10;` for a
named constant whose value is computed at compile time.

### Checkpoint 1

<div class="quiz" data-answer="b" data-explanation="Shadowing introduces a new binding, which may have a different type.">
<p>Why is the final <code>let attempts = attempts.to_string()</code> allowed?</p>
<label><input type="radio" name="fundamentals-1" value="a"> mut permits any type change</label><br>
<label><input type="radio" name="fundamentals-1" value="b"> A new binding shadows the old one</label><br>
<label><input type="radio" name="fundamentals-1" value="c"> All variables are strings internally</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>
A constant needs an explicit type and a compile-time value. For example,
`const LIMIT: usize = 10;` does not create a mutable counter.

<div class="quiz" data-answer="b" data-explanation="Mutation changes a value of the same type; shadowing introduces a new binding.">
<p>Can assigning to a mutable integer binding change its type to text?</p>
<label><input type="radio" name="fundamentals-t2-q2" value="a"> Yes, mut allows any type</label><br>
<label><input type="radio" name="fundamentals-t2-q2" value="b"> No, assignment preserves the binding’s type</label><br>
<label><input type="radio" name="fundamentals-t2-q2" value="c"> Only if the text contains digits</label><br>
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

Start the counter at 4, increase it by 2, and update the assertions. Remove
`mut`, read the error, then restore it. Explain why shadowing still needs a
new `let` even when the original binding is mutable.

## Check your understanding
<div class="quiz" data-answer="c" data-explanation="let introduces a binding; reusing the name shadows the previous binding.">
<p>Which choice creates a new binding?</p>
<label><input type="radio" name="fundamentals-t2-q3" value="a"> <code>attempts += 1;</code></label><br>
<label><input type="radio" name="fundamentals-t2-q3" value="b"> <code>println!("{attempts}");</code></label><br>
<label><input type="radio" name="fundamentals-t2-q3" value="c"> <code>let attempts = attempts.to_string();</code></label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Topic 3: Scalar types and operators](03-scalar-types-and-operators.md). Return to the [topic index](index.md).
