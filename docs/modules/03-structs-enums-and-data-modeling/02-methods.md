# Methods and associated functions

By the end, you will be able to

- Attach **[methods](#put-behavior-beside-the-data)** to `Task` with an **[impl block](#put-behavior-beside-the-data)**.
- Choose **[&self](#put-behavior-beside-the-data)**, **[&mut self](#mutable-receivers-change-fields)**, or **[self](#consuming-receivers-transfer-ownership)** based on ownership needs.
- Call methods with **[dot syntax](#put-behavior-beside-the-data)**.
- Construct values with **[associated functions and Self](#associated-functions-construct-values)**.
- Keep methods **[focused on one responsibility](#focused-methods-and-return-values)** and return appropriate values.

Prerequisite: [Topic 1: Structs](01-structs.md).

!!! info "Refresher: a struct owns its fields"
    `Task` has an owned `String` field. A method can borrow that title, replace
    it through a mutable borrow, or move it out by consuming the whole task.

!!! tip "Refresher: mutability belongs to the binding"
    Calling a method through `&mut self` requires a mutable binding such as
    `let mut task = ...`. The method does not make an immutable binding mutable.

---

## Put behavior beside the data

An `impl` block attaches functions to a type. Define the struct and impl blocks
at module scope; put calls and local bindings inside `main`. The snippets build
on the same Task definition, and the complete demo supplies all definitions.

An impl block does not require every function to take a receiver. Inside the block, the first
parameter is often a receiver that describes how the method uses the value.

```rust
struct Task {
    id: u32,
    title: String,
}

impl Task {
    fn title(&self) -> &str {
        &self.title
    }
}

let task = Task { id: 1, title: String::from("Read") };
println!("{}", task.title());
```

Read the syntax as follows:

- **`.`** — method-call syntax: `task.title()` calls a method on an existing task.
- **`&self`** — shared receiver: `fn title(&self)` borrows the task for reading.
- **`->`** — return-type marker: `-> &str` returns borrowed text.

The call `task.title()` is method syntax. Rust passes a reference to `task` as
the receiver, so the method can read the field without taking ownership.

### Checkpoint 1

<div class="quiz" data-answer="b" data-explanation="&self borrows the receiver for the call, so the Task remains usable afterward.">
<p>What does <code>&amp;self</code> mean in <code>fn title(&amp;self) -&gt; &amp;str</code>?</p>
<label><input type="radio" name="methods-2-self" value="a"> The method consumes the task</label><br>
<label><input type="radio" name="methods-2-self" value="b"> The method borrows the task for the call</label><br>
<label><input type="radio" name="methods-2-self" value="c"> The method clones every field</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Mutable receivers change fields

Use `&self` for read-only behavior. Use `&mut self` when the method changes a
field. The caller must have a mutable binding for a mutable receiver:

```rust
impl Task {
    fn rename(&mut self, title: &str) {
        self.title = String::from(title);
    }
}

let mut task = Task { id: 1, title: String::from("Read") };
task.rename("Practice");
println!("{}", task.title);
```

The parameter `title: &str` is borrowed input. `String::from(title)` creates a
new owned string for the field, so the task owns its title after the call.

!!! tip "Borrowing reminder"
    A borrowed `&str` is a temporary view. The task cannot store that view in
    this example, because its field is an owned `String`; converting with
    `String::from` gives the task its own data.

### Checkpoint 2

<div class="quiz" data-answer="c" data-explanation="rename changes the title, so it needs a mutable borrow of the Task and a mutable binding at the call site.">
<p>Which receiver replaces <code>Task.title</code> while keeping the task available to its caller?</p>
<label><input type="radio" name="methods-2-mutable" value="a"> <code>fn rename(self, title: &amp;str)</code></label><br>
<label><input type="radio" name="methods-2-mutable" value="b"> <code>fn rename(&amp;self, title: &amp;str)</code></label><br>
<label><input type="radio" name="methods-2-mutable" value="c"> <code>fn rename(&amp;mut self, title: &amp;str)</code></label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>


---

## Consuming receivers transfer ownership

Use `self` when the method must take ownership. This method returns the title
and consumes the task while moving the field out:

```rust
impl Task {
    fn into_title(self) -> String {
        self.title
    }
}

let task = Task { id: 1, title: String::from("Practice") };
let title = task.into_title();
println!("{title}");
// task is no longer available here: the method consumed it.
```

The tail expression `self.title` moves the owned field into the result without
cloning it. The caller owns the returned string; the original task is consumed.

<div class="quiz" data-answer="c" data-explanation="The self receiver takes ownership of task, and the returned String becomes owned by title.">
<p>After <code>let title = task.into_title();</code>, which value remains usable?</p>
<label><input type="radio" name="methods-2-consume" value="a"> Both task and title, because a move creates a copy</label><br>
<label><input type="radio" name="methods-2-consume" value="b"> Only task, because the result borrows it</label><br>
<label><input type="radio" name="methods-2-consume" value="c"> title, which owns the String returned by the consumed task</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>



---

## Associated functions construct values

An associated function belongs to a type but has no receiver. `Task::new` uses
the type name and `::` because it creates a task before a task exists to call a
method on.

```rust
impl Task {
    fn new(id: u32, title: &str) -> Self {
        Self {
            id,
            title: String::from(title),
        }
    }
}

let task = Task::new(1, "Read the Rust book");
println!("{}", task.title()); // Read the Rust book
```

- **`::`** — associated-item path separator: `Task::new(...)` calls through the type name.
- **`Self`** — implementing type: `-> Self` means the result is a `Task` here.
- **`self`** — receiver value: `self.title` accesses the current task's field.

`new` is a naming convention, not a constructor keyword or a function Rust
automatically supplies. Associated functions need not construct values; this
one does. `Self` means the type currently being implemented, here `Task`. The constructor
accepts borrowed text and creates the owned `String` required by the struct.

### Checkpoint 3

<div class="quiz" data-answer="a" data-explanation="new has no receiver, so it is called with the type name and :: before an instance exists.">
<p>Why is the constructor called as <code>Task::new(...)</code>?</p>
<label><input type="radio" name="methods-2-associated" value="a"> It is an associated function with no existing Task receiver</label><br>
<label><input type="radio" name="methods-2-associated" value="b"> It borrows every Task in the program</label><br>
<label><input type="radio" name="methods-2-associated" value="c"> <code>::</code> always means a mutable method call</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Focused methods and return values

`title` reads, `rename` changes one field, and `into_title` transfers ownership.
They leave printing to the caller, which keeps their behavior easy to test.
`title` returns a borrowed `&str`; `into_title` returns an owned `String`.
`rename` has no declared return type and returns unit `()`: its result is the
changed task rather than a separate title value.

---

## Complete runnable example

The demo combines an associated constructor, read-only method, mutable method,
consuming method, and a focused test:

```rust
--8<-- "examples/03-structs-enums-and-data-modeling/02-methods/main.rs"
```

From the repository root:

```bash
rustc examples/03-structs-enums-and-data-modeling/02-methods/main.rs -o /tmp/rust-task-methods
/tmp/rust-task-methods
rustc --test examples/03-structs-enums-and-data-modeling/02-methods/main.rs -o /tmp/rust-task-methods-tests
/tmp/rust-task-methods-tests
```

Expected program output:

```text
created: Task { id: 1, title: "Read the Rust book" } (id: 1)
title: Read the Rust book
renamed: Task { id: 1, title: "Practice methods" }
owned title: Practice methods
```

---

## Exercises

Attempt each task before expanding its solution. Each solution is a standalone
program. Save it in a temporary `.rs` file and compile with `rustc`; compile the
first solution with `rustc --test` too to run its test.

### 1. Add a borrowed description method

Add a `description: String` field and `describe(&self) -> &str` method. Update
`new` to accept the description, print it in `main`, and extend a test to check it.

??? success "Solution: let the task own its description"
    ```rust
    struct Task {
        id: u32,
        title: String,
        description: String,
    }

    impl Task {
        fn new(id: u32, title: &str, description: &str) -> Self {
            Self {
                id,
                title: String::from(title),
                description: String::from(description),
            }
        }

        fn describe(&self) -> &str {
            &self.description
        }
    }

    fn main() {
        let task = Task::new(1, "Read", "Study methods");
        assert_eq!(task.describe(), "Study methods");
        println!("{}: {} - {}", task.id, task.title, task.describe());
    }

    #[cfg(test)]
    mod tests {
        use super::Task;

        #[test]
        fn description_is_owned_and_borrowed_for_reading() {
            let description = String::from("Study methods");
            let task = Task::new(1, "Read", &description);
            drop(description);
            assert_eq!(task.id, 1);
            assert_eq!(task.title, "Read");
            assert_eq!(task.describe(), "Study methods");
            assert_eq!(Task::new(2, "Empty", "").describe(), "");
        }
    }
    ```

    The constructor creates owned text; `describe` borrows it instead of
    cloning. Dropping the constructor's input does not remove the stored
    description. This focused solution shows the added field and method;
    retain the original methods when extending the full demo.
    Expected output: `1: Read - Study methods`. The test command passes one test.

---

### 2. Rename from literal and owned text

Keep `rename(&mut self, title: &str)` and call it once with a string literal and
once with a borrowed view of an owned `String`. Verify that the input string
remains usable after the call; no new generic API is needed.

??? success "Solution: borrow both kinds of text as str"
    ```rust
    struct Task {
        title: String,
    }

    impl Task {
        fn rename(&mut self, title: &str) {
            self.title = String::from(title);
        }
    }

    fn main() {
        let mut task = Task { title: String::from("Read") };
        task.rename("Practice");
        assert_eq!(task.title, "Practice");
        let owned = String::from("Review methods");
        task.rename(&owned);
        assert_eq!(task.title, "Review methods");
        assert_eq!(owned, "Review methods");
        println!("task: {}, input: {owned}", task.title);
    }
    ```

    A literal provides `&str` directly. `&owned` is converted to a borrowed
    `&str` view at the call. `String::from` creates the task's replacement
    title, while the caller keeps ownership of `owned`.
    Expected output: `task: Review methods, input: Review methods`.

---

## Check your understanding

<div class="quiz" data-answer="b" data-explanation="A method should borrow with &self when it only reads, use &mut self when it changes fields, and consume with self when it takes ownership.">
<p>Which receiver choice matches the operation?</p>
<label><input type="radio" name="methods-2-final" value="a"> Use <code>self</code> for every method so ownership is always clear</label><br>
<label><input type="radio" name="methods-2-final" value="b"> Use <code>&amp;self</code> to read, <code>&amp;mut self</code> to change, and <code>self</code> to consume</label><br>
<label><input type="radio" name="methods-2-final" value="c"> Use <code>&amp;mut self</code> even for methods that only print</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Next: [Topic 3, Enums: representing alternatives](03-enum.md).
