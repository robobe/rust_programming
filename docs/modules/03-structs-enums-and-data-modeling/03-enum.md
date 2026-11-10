# Topic 3: Enums: representing alternatives

By the end, you will be able to

- Define an **[enum and its variants](#enums-and-unit-like-variants)**.
- Choose **[unit-like](#enums-and-unit-like-variants)**, **[tuple-like](#tuple-like-variants)**, and **[struct-like variants](#struct-like-variants)**.
- Read variant data through **[pattern matching](#matching-and-exhaustiveness)** and explain **[exhaustiveness](#matching-and-exhaustiveness)**.
- Model task status with **[TaskStatus](#add-status-to-the-task-model)**.
- Distinguish **[custom enums, Option, and Result](#custom-enums-option-and-result)**.

Prerequisite: [Topic 2: Methods and associated functions](02-methods.md).
Each short snippet is independent: put its enum definitions before `main` and
its local bindings and calls inside `main`. The complete demo is runnable as-is.

---

## Enums and unit-like variants

A struct groups related fields together. An enum describes alternatives:
a value is exactly one variant at a time. A task cannot be simultaneously
Pending and Completed when its status is one enum value.

```rust
enum TaskStatus {
    Pending,
    Completed,
    Cancelled,
}

let status = TaskStatus::Pending;
match status {
    TaskStatus::Pending => println!("Pending"),
    TaskStatus::Completed => println!("Completed"),
    TaskStatus::Cancelled => println!("Cancelled"),
}
```

These are **unit-like variants**: they carry no extra data. Unlike an integer
status code, a value such as 99 cannot accidentally become a TaskStatus.

- **`enum`** — type-definition keyword: `enum TaskStatus` names the alternatives.
- **`::`** — variant path separator: `TaskStatus::Pending` selects a variant.

<div class="quiz" data-answer="b" data-explanation="An enum value contains one selected variant; mut permits replacing that value, not combining variants.">
<p>Can a single TaskStatus value be both Pending and Completed?</p>
<label><input type="radio" name="enums-refactor-1" value="a"> Yes, if the binding uses mut</label><br>
<label><input type="radio" name="enums-refactor-1" value="b"> No, it holds one variant at a time</label><br>
<label><input type="radio" name="enums-refactor-1" value="c"> Yes, every enum stores all its variants</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Tuple-like variants

Tuple-like variants carry unnamed fields. Different variants can store
different types and different numbers of values:

```rust
enum Message {
    Quit,
    Note(String),
    Move(i32, i32),
}

let message = Message::Move(3, 4);
match message {
    Message::Quit => println!("quit"),
    Message::Note(text) => println!("note: {text}"),
    Message::Move(x, y) => println!("move: {x}, {y}"),
}
```

**Destructure the variant to access its payload.** `Message::Move(x, y)` binds
its two fields. An enum value is not an ordinary tuple: `message.0` cannot
access a variant's data. Matching first proves which variant is present.

<div class="quiz" data-answer="c" data-explanation="Move was declared with positional fields, so its pattern uses parentheses and binds fields in order.">
<p>For Message::Move(3, 4), which pattern binds both coordinates?</p>
<label><input type="radio" name="enums-refactor-2" value="a"> Message::Move { x, y }</label><br>
<label><input type="radio" name="enums-refactor-2" value="b"> message.0</label><br>
<label><input type="radio" name="enums-refactor-2" value="c"> Message::Move(x, y)</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Struct-like variants

Use named fields when they make a variant's data easier to understand:

```rust
enum Event {
    Started,
    Progress { completed: u32, total: u32 },
}

let event = Event::Progress { completed: 2, total: 5 };
match event {
    Event::Started => println!("started"),
    Event::Progress { completed, total } => println!("progress: {completed}/{total}"),
}
```

`Progress { completed, total }` is a struct-like variant. In contrast,
`Progress(SomeStruct)` would be a tuple-like variant containing a struct.
Access an enum's named payload through a pattern, not `event.Progress.completed`.

- **`(...)`** — positional variant fields: `Message::Move(3, 4)` supplies ordered values.
- **`{ ... }`** — named variant fields: `Event::Progress { completed: 2, total: 5 }` names each value.

<div class="quiz" data-answer="a" data-explanation="Braces and named fields define a struct-like variant; parentheses are tuple-like, and no fields is unit-like.">
<p>Which declaration makes a struct-like variant?</p>
<label><input type="radio" name="enums-refactor-3" value="a"> Progress { completed: u32, total: u32 }</label><br>
<label><input type="radio" name="enums-refactor-3" value="b"> Progress(u32, u32)</label><br>
<label><input type="radio" name="enums-refactor-3" value="c"> Progress</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Matching and exhaustiveness

A `match` must cover every possible variant. Removing the Cancelled arm from
the first example causes **E0004: non-exhaustive patterns**; the program cannot
run until all cases are handled. A wildcard `_` can cover remaining cases,
but explicit arms make newly added variants harder to overlook.

- **`=>`** — arm separator: `TaskStatus::Pending => "pending"` maps a pattern to an expression.
- **`_`** — wildcard pattern: `_ => "other"` handles values not matched earlier.

Borrow when you only need to inspect a status:

```rust
fn status_label(status: &TaskStatus) -> &str {
    match status {
        TaskStatus::Pending => "pending",
        TaskStatus::Completed => "completed",
        TaskStatus::Cancelled => "cancelled",
    }
}
```

The whole `match` is the function's tail expression; every arm returns a
string literal. No integer status codes or manual casts are necessary.
[Topic 4](04-pattern-matching.md) develops borrowed versus owned patterns,
ignored fields, guards, and `if let` after this introduction.

<div class="quiz" data-answer="b" data-explanation="Exhaustiveness is checked before execution; each possible variant needs a matching arm.">
<p>If the Cancelled arm is removed with no wildcard fallback, what happens?</p>
<label><input type="radio" name="enums-refactor-4" value="a"> The program silently ignores Cancelled</label><br>
<label><input type="radio" name="enums-refactor-4" value="b"> Compilation fails because Cancelled is uncovered</label><br>
<label><input type="radio" name="enums-refactor-4" value="c"> The compiler inserts a default arm</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Add status to the Task model

Keep the identifier and owned title from earlier lessons, then add
`status: TaskStatus`. The constructor creates Pending tasks. Mutable methods
replace the status, while inspection borrows it.

The constructor in this demo accepts an owned `String`, unlike Topic 2's
borrowed `&str` parameter. Call it with `String::from(...)`; ownership of that
title passes into the task. This is an API choice, not a requirement of enums.

`#[derive(Debug)]` enables debug printing of the actual task; deriving
`PartialEq` on its status enables equality assertions. Module 4 explains traits.

The demo's `complete` and `cancel` methods set their statuses unconditionally.
[Topic 6](06-result.md) adds rejected transitions through `Result`; this lesson
does not yet enforce a lifecycle policy.

---

## Complete runnable example

```rust
--8<-- "examples/03-structs-enums-and-data-modeling/03-enums/main.rs"
```

!!! info "New keywords: derives and simple tests"
    Tests check that code behaves as expected. They are separate from the
    program's normal execution; the demo README contains the test commands.

    - **`#[derive(Debug, PartialEq)]`** — asks Rust to generate implementations of these traits for the type.
    - **`Debug`** — enables debug formatting such as `println!("{task:?}")`.
    - **`PartialEq`** — enables equality comparisons, including the comparisons used by `assert_eq!`.
    - **`#[cfg(test)]`** — includes the following item only when compiling tests.
    - **`mod tests { ... }`** — groups test code in a module named `tests`.
    - **`use super::*;`** — brings accessible names from the parent module into scope, such as `Task` and `TaskStatus`.
    - **`#[test]`** — marks a function for the test runner to execute.
    - **`assert_eq!(actual, expected)`** — checks equality and fails the test if the values differ.

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn new_task_is_pending() {
            let task = Task::new(1, String::from("Read"));
            assert_eq!(task.status, TaskStatus::Pending);
        }
    }
    ```

    This example checks that a newly constructed task starts **Pending**.
    A passing test normally prints no application output; the test runner
    reports success or failure. Module 5 develops testing in more detail.

!!! info "Build, run, list, and select tests"
    From the repository root, **build the test executable** with `--test`:

    ```bash
    rustc --test examples/03-structs-enums-and-data-modeling/03-enums/main.rs -o /tmp/rust-task-enums-tests
    ```

    This includes `#[cfg(test)]` code and builds a test runner instead of
    starting the program through its normal `main()`.

    **Run all tests:**

    ```bash
    /tmp/rust-task-enums-tests
    ```

    **List tests without running them:**

    ```bash
    /tmp/rust-task-enums-tests --list
    ```

    The example lists `tests::constructs_and_sets_each_status: test`.

    **Run exactly one named test:**

    ```bash
    /tmp/rust-task-enums-tests tests::constructs_and_sets_each_status --exact
    ```

    Use the full name from `--list` with `--exact`. Without `--exact`, the
    supplied name is a substring filter and may select several tests. Check
    the reported test count: a misspelled filter may run zero tests.
    Add `--nocapture` to see output printed by passing tests.

    For a Cargo project, the corresponding commands are:

    ```bash
    cargo test --no-run
    cargo test
    cargo test -- --list
    cargo test tests::constructs_and_sets_each_status -- --exact
    ```

    Cargo commands require a Cargo project; this lesson's standalone demo
    uses the `rustc` commands above. Arguments after Cargo's `--` go to the
    test runner. To run the normal demo, build without `--test`:

    ```bash
    rustc examples/03-structs-enums-and-data-modeling/03-enums/main.rs -o /tmp/rust-task-enums
    /tmp/rust-task-enums
    ```

Expected program output:

```text
Created task: Task { id: 1, title: "Read the Rust book", status: Pending }
Status: Pending
Completed!
Status: Completed
```

The demo includes one test covering construction and both status-setting methods.

---

## Custom enums, Option, and Result

**They are all enums.** Each answers a different question. You define custom
enums for your application's alternatives; Rust already provides `Option` and
`Result` for two common situations.

### Custom enum: which state is the task in?

```rust
enum TaskStatus {
    Pending,
    Completed,
    Cancelled,
}

let status = TaskStatus::Pending;
```

The task exists and has one of these statuses. You choose the variant names
and any data they carry to describe your application's domain.

---

### Option: is there a value?

`Option<T>` represents a value that might be absent. `T` names the contained
type, such as an integer or a task reference.

- **`Some(value)`** — a value exists.
- **`None`** — no value exists.

```rust
let numbers = [10, 20];
let found = numbers.get(0);
let missing = numbers.get(9);
println!("found: {found:?}");     // found: Some(10)
println!("missing: {missing:?}"); // missing: None
```

`get` returns an optional reference: `Some(&10)` in the first case, or `None`
in the second. Debug formatting prints the referenced integer without an `&`.

For tasks, `Option<&Task>` means **a borrowed task was found, or no task was
found**. A missing task is different from a Pending task: Pending describes
an existing task. A missing lookup is an expected possibility, not necessarily
an operation failure.

---

### Result: did the operation succeed?

`Result<T, E>` represents success or failure. `T` names the success value's
type; `E` names the error's type.

- **`Ok(value)`** — success, with its returned value.
- **`Err(error)`** — failure, with a reason.

```rust
let valid = "42".parse::<u32>();
let invalid = "hello".parse::<u32>();
assert_eq!(valid, Ok(42));
assert!(invalid.is_err());
println!("valid: {valid:?}"); // valid: Ok(42)
```

`parse::<u32>()` attempts to convert text into an unsigned integer. `"42"`
succeeds; `"hello"` returns an error value rather than an integer. The caller
can handle that error with `match`; receiving `Err` does not itself crash the
program.

For task completion, `Result<(), TaskError>` means **completion succeeded, or
it failed for a specific reason**. `Ok(())` reports success with no extra
payload. `Err(TaskError::AlreadyCompleted)` reports a rejected operation.
`TaskError` is another custom enum, introduced in Topic 6.

---

### How the three work together

| Question | Type | Example answer |
| --- | --- | --- |
| Which status does this task have? | `TaskStatus` | `Pending` |
| Did we find the requested task? | `Option<&Task>` | `Some(task)` or `None` |
| Could we complete it? | `Result<(), TaskError>` | `Ok(())` or `Err(TaskError::AlreadyCompleted)` |

A useful sequence is **find the task → inspect its status → attempt completion**.
Each step answers a different question. A found task may be Pending, Completed,
or Cancelled. A found Completed task still exists, but trying to complete it
again can produce an error.

Use `match` to handle the relevant alternatives rather than assuming a value
or a successful outcome. Topics [5](05-option.md) and [6](06-result.md) develop
these standard enums and their use in the task tracker.

---

## Exercises

Try each task before expanding its solution. Solutions are standalone programs;
save one in a temporary `.rs` file, compile with `rustc`, and run it.

### 1. Give completion a description

Create `Completed { description: String }` alongside Pending and Cancelled.
Construct a completed value and match all variants to print its description.

??? success "Solution: store named completion data"
    ```rust
    enum TaskStatus {
        Pending,
        Completed { description: String },
        Cancelled,
    }

    fn main() {
        let statuses = [
            TaskStatus::Pending,
            TaskStatus::Completed { description: String::from("Read chapter 1") },
            TaskStatus::Cancelled,
        ];
        for status in &statuses {
            match status {
                TaskStatus::Pending => println!("pending"),
                TaskStatus::Completed { description } => println!("done: {description}"),
                TaskStatus::Cancelled => println!("cancelled"),
            }
        }
    }
    ```

    The Completed construction and pattern both use braces and the field name.
    Expected output:

    ```text
    pending
    done: Read chapter 1
    cancelled
    ```

---

### 2. Construct payment alternatives

Define `PaymentMethod` with a card label, a bank label, or a crypto currency
label and integer reference ID. Write a function returning the chosen crypto
variant and match all alternatives. Use fictional labels, not payment processing.

??? success "Solution: construct and match positional payloads"
    ```rust
    enum PaymentMethod {
        Card(String),
        Bank(String),
        Crypto(String, u32),
    }

    fn example_payment() -> PaymentMethod {
        PaymentMethod::Crypto(String::from("BTC"), 7)
    }

    fn main() {
        let methods = [
            PaymentMethod::Card(String::from("demo card")),
            PaymentMethod::Bank(String::from("demo bank")),
            example_payment(),
        ];
        for method in &methods {
            match method {
                PaymentMethod::Card(label) => println!("card: {label}"),
                PaymentMethod::Bank(label) => println!("bank: {label}"),
                PaymentMethod::Crypto(currency, id) => println!("crypto: {currency}, reference: {id}"),
            }
        }
    }
    ```

    Each variant carries its own payload shape. This only models fictional
    alternatives; it does not handle money or perform transactions.
    Expected output:

    ```text
    card: demo card
    bank: demo bank
    crypto: BTC, reference: 7
    ```

---

### 3. Return a label from every variant

Write `status_label(&TaskStatus) -> &str` for the original unit-like status
variants. Test all three branches without consuming a status.

??? success "Solution: borrow the enum and return each arm's label"
    ```rust
    enum TaskStatus { Pending, Completed, Cancelled }

    fn status_label(status: &TaskStatus) -> &str {
        match status {
            TaskStatus::Pending => "pending",
            TaskStatus::Completed => "completed",
            TaskStatus::Cancelled => "cancelled",
        }
    }

    fn main() {
        assert_eq!(status_label(&TaskStatus::Pending), "pending");
        assert_eq!(status_label(&TaskStatus::Completed), "completed");
        assert_eq!(status_label(&TaskStatus::Cancelled), "cancelled");
        println!("{}", status_label(&TaskStatus::Completed));
    }
    ```

    Every arm supplies a string literal of the expected type.
    Expected output: `completed`.

---

### 4. Separate missing status from a real status

As a preview of Topic 5, use `Option<TaskStatus>` rather than adding a confusing
`None` variant to TaskStatus. Handle `Some(Pending)`, `Some(Completed)`,
`Some(Cancelled)`, and `None` separately.

??? success "Solution: wrap domain status in Option"
    ```rust
    enum TaskStatus { Pending, Completed, Cancelled }

    fn main() {
        let statuses = [
            Some(TaskStatus::Pending),
            Some(TaskStatus::Completed),
            Some(TaskStatus::Cancelled),
            None,
        ];
        for status in statuses {
            match status {
                Some(TaskStatus::Pending) => println!("pending"),
                Some(TaskStatus::Completed) => println!("completed"),
                Some(TaskStatus::Cancelled) => println!("cancelled"),
                None => println!("missing"),
            }
        }
    }
    ```

    Some contains a real status; None contains no status at all.
    Expected output:

    ```text
    pending
    completed
    cancelled
    missing
    ```

---

## Check your understanding
<div class="quiz" data-answer="c" data-explanation="A domain enum models valid alternatives; wrapping it in Option adds absence without confusing it with a real status.">
<p>Which modeling choice best separates domain status from absence?</p>
<label><input type="radio" name="enums-refactor-5" value="a"> Use Pending for both an existing pending task and a missing task</label><br>
<label><input type="radio" name="enums-refactor-5" value="b"> Use one integer code for everything</label><br>
<label><input type="radio" name="enums-refactor-5" value="c"> Use TaskStatus for alternatives and Option<TaskStatus> when a status may be absent</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Topic 4: Pattern matching and destructuring](04-pattern-matching.md).
Return to the [Module 3 index](index.md).
