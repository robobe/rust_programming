# Topic 7: Putting the task model together

By the end, you will be able to

- Combine **[Task methods and ownership](#one-task-model)**.
- Implement **[identifier lookup](#find-by-identifier)** returning **[Option<&Task>](#find-by-identifier)**.
- Test **[state transitions and Result errors](#test-the-model-boundaries)**.

Prerequisite: [Topic 6: Result](06-result.md).
The snippets below build on the previous task model; the complete demo includes
all required definitions.

---

## One task model

Bring the preceding lessons together. A task owns an identifier, title, and
status. The status is Pending, Completed, or Cancelled with a reason. Its
methods keep each operation focused:

| Member | Responsibility |
| --- | --- |
| `Task::new(id, title)` | Take an owned String and create a Pending task |
| `title(&self)` | Read the title as borrowed text |
| `rename(&mut self, title)` | Replace the title from borrowed text |
| `into_title(self)` | Consume the task and return its owned title |
| `complete(&mut self)` | Complete Pending or return a TaskError |

Topic 2 accepted borrowed text in its constructor, while Topic 3 accepted an
owned String. This integrated example keeps Topic 3's owned constructor: pass
`String::from("Read Rust")`. The other receiver choices follow Topic 2.
`PartialEq` on the status and error enums allows assertions to compare them;
no `Clone` is needed.
<div class="quiz" data-answer="c" data-explanation="A self receiver takes ownership, while reference receivers borrow the existing task.">
<p>Which method consumes its task?</p>
<label><input type="radio" name="m3-t7-q1" value="a"> title(&self)</label><br>
<label><input type="radio" name="m3-t7-q1" value="b"> rename(&mut self, title)</label><br>
<label><input type="radio" name="m3-t7-q1" value="c"> into_title(self)</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Find by identifier

Identifiers stay meaningful when positions change. A simple borrowed loop
finds the first matching ID without consuming or cloning any task:

```rust
fn find_task(tasks: &[Task], id: u32) -> Option<&Task> {
    for task in tasks {
        if task.id == id {
            return Some(task);
        }
    }
    None
}
```

The slice API accepts a vector or array. The result borrows that input.
For `[task_42, task_9]`, asking for ID 9 returns the second task, while asking
for ID 0 returns `None`; zero is a position here, not a stored ID.
The caller supplies unique IDs. Duplicate IDs return the first match in this
teaching implementation; ID generation and validated storage come later.
<div class="quiz" data-answer="a" data-explanation="This function compares identifiers; it does not interpret its argument as a collection index.">
<p>A vector contains task IDs 42 and 9. What does find_task(&tasks, 0) return?</p>
<label><input type="radio" name="m3-t7-q2" value="a"> None, because no task has ID 0</label><br>
<label><input type="radio" name="m3-t7-q2" value="b"> The first task because its position is 0</label><br>
<label><input type="radio" name="m3-t7-q2" value="c"> A cloned task with a new ID</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Test the model boundaries

Test the behavior learners will rely on, including rejection paths:

- Construction starts Pending; reading, renaming, and consuming the title work.
- A Pending task completes successfully.
- Repeated completion returns `AlreadyCompleted` and preserves the task.
- Cancelled completion returns `Cancelled` and preserves the reason.
- Identifier lookup finds an existing task and handles missing and empty input.

The demo includes four unit tests covering these behaviors. Mutation and
printing remain separate: `complete` returns a result, and the caller presents
it. File storage, CLI parsing, private-field validation, and error propagation
belong to later modules.

---

## Complete runnable example

```rust
--8<-- "examples/03-structs-enums-and-data-modeling/07-task-model/main.rs"
```

Run from the repository root:

```bash
rustc examples/03-structs-enums-and-data-modeling/07-task-model/main.rs -o /tmp/rust-07-task-model
/tmp/rust-07-task-model
rustfmt --check examples/03-structs-enums-and-data-modeling/07-task-model/main.rs
rustc --test examples/03-structs-enums-and-data-modeling/07-task-model/main.rs -o /tmp/rust-task-model-tests
/tmp/rust-task-model-tests
```

Expected program output:

```text
task 42 completed: Practice task modeling
repeat: Err(AlreadyCompleted)
found id 42: Practice task modeling
missing id 999: true
```

---

## Exercise and completion goal

1. Add a third task with ID 100 and look it up by identifier after reordering
   the vector. Verify that its ID still finds it at its new position.
2. Add a test showing that a zero-length vector returns `None` for any ID.
3. Change the friendly output to distinguish missing lookup, successful
   completion, and rejected completion, without using `unwrap()`.
4. Explain each method's ownership choice and why a missing task uses `Option`
   while completion uses `Result`.

You have completed Module 3 when you can reconstruct this model, test both
success and rejection, and explain each borrowed and owned value.

---

## Check your understanding
<div class="quiz" data-answer="b" data-explanation="Focused methods return observable outcomes that tests can check independently of terminal output.">
<p>Which design keeps behavior testable and responsibilities clear?</p>
<label><input type="radio" name="m3-t7-q3" value="a"> Print and panic inside every method</label><br>
<label><input type="radio" name="m3-t7-q3" value="b"> Return domain outcomes from methods and let callers present them</label><br>
<label><input type="radio" name="m3-t7-q3" value="c"> Clone every task before reading it</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Module 4: Traits and generics](../../syllabus.md#4-traits-and-generics-planned) (planned). Return to the [module index](index.md).
