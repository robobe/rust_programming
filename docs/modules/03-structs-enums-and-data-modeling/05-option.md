# Topic 5: Option: representing missing values

By the end, you will be able to

- Represent absence with **[Option, Some, and None](#some-and-none)**.
- Interpret **[Option<&Task>](#borrowed-lookup-by-position)** and distinguish positions from identifiers.
- Handle optional values with **[match or if let](#handle-absence-explicitly)** instead of **[unwrap](#handle-absence-explicitly)**.

Prerequisite: [Topic 4: Pattern matching](04-pattern-matching.md).
The snippets below build on the previous task model; the complete demo includes
all required definitions.

---

## Some and None

A lookup may legitimately find nothing. `Option<T>` makes both possibilities
explicit: `Some(T)` contains a value; `None` contains no value. `T` is the
contained type, not a requirement to write your own generic code yet.

```rust
let present: Option<u32> = Some(42);
let missing: Option<u32> = None;
```

Both values have the same type. `Option` is an enum, and `Some` and `None` are
available without a type prefix. Choose it for expected absence, rather than
inventing a sentinel task with ID zero or panicking when a lookup misses.
<div class="quiz" data-answer="c" data-explanation="None contains no task; absence is represented directly in the return type.">
<p>What does <code>None</code> represent in a task lookup?</p>
<label><input type="radio" name="m3-t5-q1" value="a"> A task whose identifier is zero</label><br>
<label><input type="radio" name="m3-t5-q1" value="b"> An error message stored as text</label><br>
<label><input type="radio" name="m3-t5-q1" value="c"> No task was found</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Borrowed lookup by position

`get` checks the position and returns `None` when it is out of bounds:

```rust
fn task_at(tasks: &[Task], position: usize) -> Option<&Task> {
    tasks.get(position)
}
```

`Option<&Task>` means either a borrowed task or no task. It does not copy the
Task or transfer the collection. Its reference cannot outlive the input data.

| Operation | Meaning |
| --- | --- |
| `task_at(&tasks, 0)` | Read the first task, if any |
| `task_at(&tasks, 99)` | Read position 99, if it exists |
| `tasks[99]` | Panic if position 99 is out of bounds |

A position is not an identifier. In the demo, position 0 contains task ID 42.
Identifier lookup is implemented separately in Topic 7.
<div class="quiz" data-answer="b" data-explanation="Some holds a reference into the input collection; None represents the missing case.">
<p>What does <code>Option&lt;&amp;Task&gt;</code> let a caller receive?</p>
<label><input type="radio" name="m3-t5-q2" value="a"> A mandatory owned Task</label><br>
<label><input type="radio" name="m3-t5-q2" value="b"> A borrowed Task when present, or None</label><br>
<label><input type="radio" name="m3-t5-q2" value="c"> A clone of every task</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Handle absence explicitly

```rust
match task_at(&tasks, 0) {
    Some(task) => println!("{}: {}", task.id, task.title),
    None => println!("No task at this position"),
}
```

Use `if let Some(task) = task_at(&tasks, 0)` when only the present case needs
an action. Add `else` if absence also needs a response. Avoid `unwrap()` for
expected absence: calling it on `None` panics rather than letting your program
choose a useful response.

---

## Complete runnable example

```rust
--8<-- "examples/03-structs-enums-and-data-modeling/05-option/main.rs"
```

Run from the repository root:

```bash
rustc examples/03-structs-enums-and-data-modeling/05-option/main.rs -o /tmp/rust-05-option
/tmp/rust-05-option
rustfmt --check examples/03-structs-enums-and-data-modeling/05-option/main.rs
```

Expected program output:

```text
position 0: task 42 - Read Rust
position 99: no task
optional title: Practice
```

---

## Exercise

1. Look up the last valid position and the first invalid position. Handle both
   with `match`, without `unwrap()`.
2. Repeat the lookup on an empty task slice and assert `is_none()`.
3. Change task ID 42 to 100 and confirm that position 0 still finds that task.
4. Print a borrowed title, then print the collection length to show the caller
   retains its vector.

---

## Check your understanding
<div class="quiz" data-answer="a" data-explanation="Option models presence or absence; use Result when an operation must report a failure reason.">
<p>When is Option a suitable return type?</p>
<label><input type="radio" name="m3-t5-q3" value="a"> When the answer may legitimately be absent</label><br>
<label><input type="radio" name="m3-t5-q3" value="b"> Only when every failure needs a detailed reason</label><br>
<label><input type="radio" name="m3-t5-q3" value="c"> Only for values stored on the stack</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Topic 6: Result](06-result.md). Return to the [module index](index.md).
