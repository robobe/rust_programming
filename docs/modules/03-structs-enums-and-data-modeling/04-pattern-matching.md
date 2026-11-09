# Topic 4: Pattern matching and destructuring

By the end, you will be able to

- Use **[patterns and destructuring](#match-and-destructure)** to access fields.
- Combine **[guards](#guards-and-ignored-fields)** with exhaustive arms.
- Choose **[borrowed or owned patterns](#borrowed-and-owned-patterns)** and **[if let](#borrowed-and-owned-patterns)**.

Prerequisite: [Topic 3: Enums](03-enum.md).
The snippets below build on the previous task model; the complete demo includes
all required definitions.

---

## Match and destructure

A pattern describes the shape of a value. Matching chooses an arm; destructuring
binds names to the fields inside that shape. Continue Topic 3's task model by
changing its unit-like `Cancelled` into a variant that stores a reason:

```rust
enum TaskStatus {
    Pending,
    Completed,
    Cancelled { reason: String },
}
```

Update constructions to `TaskStatus::Cancelled { reason: String::from("Duplicate") }`
and patterns to `TaskStatus::Cancelled { reason }` or `{ .. }`. Other variants
stay unchanged. A tuple-like variant such as `Message::Note(String)` is matched
with `Message::Note(text)`; enum payloads are accessed through patterns, not
with `.0` directly on the enum value.

```rust
let Task { id, title, .. } = &task;
```

This struct pattern borrows the task. `id` is `&u32`, `title` is `&String`, and
`..` ignores its other fields. The owned title remains in `task`.
<div class="quiz" data-answer="b" data-explanation="Matching a reference borrows the fields; it does not clone or transfer the title.">
<p>What does destructuring <code>&amp;task</code> do to its String title?</p>
<label><input type="radio" name="m3-t4-q1" value="a"> Moves it out of task</label><br>
<label><input type="radio" name="m3-t4-q1" value="b"> Borrows it, leaving task as its owner</label><br>
<label><input type="radio" name="m3-t4-q1" value="c"> Clones it automatically</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Guards and ignored fields

A guard is an extra Boolean condition after a pattern. If the condition is
false, matching continues to later arms. A guard alone cannot cover every
value of a variant, so keep an unguarded fallback:

```rust
fn status_label(status: &TaskStatus) -> &str {
    match status {
        TaskStatus::Pending => "pending",
        TaskStatus::Completed => "completed",
        TaskStatus::Cancelled { reason } if reason.is_empty() => "cancelled without a reason",
        TaskStatus::Cancelled { .. } => "cancelled with a reason",
    }
}
```

- **`=>`** — arm separator: `TaskStatus::Pending => "pending"` supplies the arm's value.
- **`..`** — remaining-fields pattern: `TaskStatus::Cancelled { .. }` ignores its fields.
- **`_`** — wildcard pattern: `Some(_)` ignores one contained value, as Topic 5 will show.

Every arm here returns a string literal. Enumerating the status variants also
makes the compiler flag this function when a new variant needs handling.
<div class="quiz" data-answer="a" data-explanation="A nonempty reason fails the guard; an unguarded arm handles that remaining case.">
<p>Why is the second Cancelled arm needed?</p>
<label><input type="radio" name="m3-t4-q2" value="a"> The guarded arm only covers cancelled values with an empty reason</label><br>
<label><input type="radio" name="m3-t4-q2" value="b"> Every variant must appear twice</label><br>
<label><input type="radio" name="m3-t4-q2" value="c"> A guard clones the reason</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Borrowed and owned patterns

Use `if let` when only one pattern needs action:

```rust
if let TaskStatus::Cancelled { reason } = &task.status {
    println!("borrowed reason: {reason}");
}
```

Here `reason` is borrowed. Matching `task.status` by value instead can move its
owned `String` reason out. After a partial move, that status field and the whole
`task` cannot be used as before; an independent unmoved field such as
`task.title` can still be used. Choose borrowing for inspection and moving when
you actually need ownership of the payload. `if let` silently skips other
variants unless you add `else`; use `match` when every case needs attention.

---

## Complete runnable example

```rust
--8<-- "examples/03-structs-enums-and-data-modeling/04-pattern-matching/main.rs"
```

Run from the repository root:

```bash
rustc examples/03-structs-enums-and-data-modeling/04-pattern-matching/main.rs -o /tmp/rust-04-pattern-matching
/tmp/rust-04-pattern-matching
rustfmt --check examples/03-structs-enums-and-data-modeling/04-pattern-matching/main.rs
```

Expected program output:

```text
task 7: Read Rust
borrowed reason: Duplicate
owned reason: Duplicate
```

---

## Exercise

1. Add a guarded arm for the reason `"Duplicate"` before the other Cancelled arms.
2. Verify labels for Pending, Completed, empty reasons, and nonempty reasons.
3. Replace the borrowed `if let` with a match handling all three variants.
4. Try printing `task.status` after its payload moves in the demo; read the
   diagnostic and remove the invalid print.

---

## Check your understanding
<div class="quiz" data-answer="c" data-explanation="Borrowed matching reads payloads while preserving the owner and its data.">
<p>Which approach is best when you only want to inspect enum payloads?</p>
<label><input type="radio" name="m3-t4-q3" value="a"> Move every payload, then clone it back</label><br>
<label><input type="radio" name="m3-t4-q3" value="b"> Use only integer status codes</label><br>
<label><input type="radio" name="m3-t4-q3" value="c"> Match a reference to the enum</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Topic 5: Option](05-option.md). Return to the [module index](index.md).
