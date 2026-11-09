# Methods and associated functions

## By the end, you will be able to

- Attach methods to `Task` with an `impl` block.
- Choose `&self`, `&mut self`, or `self` based on ownership needs.
- Call methods with dot syntax.
- Use an associated function such as `Task::new` to construct a value.
- Keep a method focused on one job and return the right kind of value.

Prerequisite: [Topic 1: Structs](01-structs.md).

!!! info "Refresher: a struct owns its fields"
    `Task` has an owned `String` field. A method can borrow that title, replace
    it through a mutable borrow, or move it out by consuming the whole task.

!!! tip "Refresher: mutability belongs to the binding"
    Calling a method through `&mut self` requires a mutable binding such as
    `let mut task = ...`. The method does not make an immutable binding mutable.

---

## Put behavior beside the data

An `impl` block attaches functions to a type. Inside the block, the first
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

## Choose the receiver deliberately

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

### Checkpoint 2

<div class="quiz" data-answer="c" data-explanation="rename changes the title, so it needs a mutable borrow of the Task and a mutable binding at the call site.">
<p>Which declaration fits a method that replaces <code>Task.title</code>?</p>
<label><input type="radio" name="methods-2-mutable" value="a"> <code>fn rename(self, title: &amp;str)</code></label><br>
<label><input type="radio" name="methods-2-mutable" value="b"> <code>fn rename(&amp;self, title: &amp;str)</code></label><br>
<label><input type="radio" name="methods-2-mutable" value="c"> <code>fn rename(&amp;mut self, title: &amp;str)</code></label><br>
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
println!("{task:?}");
```

`Self` means the type currently being implemented, here `Task`. The constructor
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

## Exercise and final checkpoint

Add a `description: String` field and a `describe(&self) -> &str` method. Update
`new` to accept the description, print it in `main`, and extend the test to
check it. Then change `rename` to accept a string literal and an owned
`String` by passing each through a borrowed `&str` view.

### Checkpoint 4

<div class="quiz" data-answer="b" data-explanation="A method should borrow with &self when it only reads, use &mut self when it changes fields, and consume with self when it takes ownership.">
<p>Which receiver choice matches the operation?</p>
<label><input type="radio" name="methods-2-final" value="a"> Use <code>self</code> for every method so ownership is always clear</label><br>
<label><input type="radio" name="methods-2-final" value="b"> Use <code>&amp;self</code> to read, <code>&amp;mut self</code> to change, and <code>self</code> to consume</label><br>
<label><input type="radio" name="methods-2-final" value="c"> Use <code>&amp;mut self</code> even for methods that only print</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

Next: [Topic 3, Enums: representing alternatives](03-enum.md).
