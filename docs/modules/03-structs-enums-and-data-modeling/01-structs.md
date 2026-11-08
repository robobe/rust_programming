# Structs: grouping related data

## By the end, you will be able to

- Define and construct a named-field `Task` struct.
- Read and update fields while respecting ownership.
- Use field shorthand and `Debug` printing.
- Recognize when struct update syntax, tuple structs, and unit-like structs fit.

Prerequisite: [Module 2: Ownership and borrowing](../02-ownership-and-borrowing/index.md).
This lesson starts the `Task` example used throughout Module 3.

---

## A task is one value with related fields

A task needs an identifier and a title. Passing these as unrelated values makes
it easy to mix up which title belongs to which ID. A struct gives the pair one
name while keeping each field's type clear.

```rust
struct Task {
    id: u32,
    title: String,
}

let task = Task {
    id: 1,
    title: String::from("Read the Rust book"),
};
println!("{}: {}", task.id, task.title);
```

The field names are part of the type. Construction must provide both fields
with the declared types. `Task` owns its `String` title. A binding must be
mutable to replace a field:

```rust
let mut task = Task {
    id: 1,
    title: String::from("Read the Rust book"),
};
task.title = String::from("Practice structs");
```

### Checkpoint 1

<div class="quiz" data-answer="b" data-explanation="The struct declares both fields, and construction must supply each with the declared type.">
<p>Which value can construct this <code>Task</code>?</p>
<label><input type="radio" name="structs-1-fields" value="a"> <code>Task { id: 1 }</code></label><br>
<label><input type="radio" name="structs-1-fields" value="b"> <code>Task { id: 1, title: String::from("Read") }</code></label><br>
<label><input type="radio" name="structs-1-fields" value="c"> <code>Task { id: "1", title: "Read" }</code></label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Field shorthand and ownership

When a variable has the same name as a field, write the name once. This moves
the `String` into the struct, just as writing `title: title` would.

```rust
let title = String::from("Read the Rust book");
let task = Task { id: 1, title };
// println!("{title}"); // Error: title was moved into task.
```

!!! tip "Ownership reminder"
    `String::from("Read the Rust book")` creates an owned `String`. Assigning
    that value to `title` gives the binding ownership, and using `title` in
    `Task { id: 1, title }` moves that ownership into the struct. The original
    `title` binding cannot be used afterward unless you borrow or clone the
    string first.

Reading `task.title` for display borrows it for the print call. An explicit
borrow can be useful when passing the field to an API that accepts `&str`:

```rust
println!("{}", task.title.as_str());
println!("{}", task.id); // task is still usable.
```

Moving the owned field into another variable is different. After a partial
move, Rust still allows access to the untouched `Copy` field `id`, but the
whole `Task` can no longer be used as a value.

```rust
let title = task.title;
println!("{title}");
println!("{}", task.id);
// println!("{}", task.title); // Error: title was moved out.
```

### Checkpoint 2

<div class="quiz" data-answer="c" data-explanation="Shorthand moves the String into the title field, so the old title binding no longer owns it.">
<p>After <code>let task = Task { id: 1, title };</code>, what happens to the original <code>title: String</code> binding?</p>
<label><input type="radio" name="structs-1-move" value="a"> It still owns the same String</label><br>
<label><input type="radio" name="structs-1-move" value="b"> It owns an automatic clone</label><br>
<label><input type="radio" name="structs-1-move" value="c"> It cannot be used because its String moved into the task</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Inspect the value with `Debug`

Add `#[derive(Debug)]` above the definition to print all fields while learning
or diagnosing a program. `{:?}` uses that derived formatting behavior. Module 4
explains the trait behind it.

```rust
#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
}

let task = Task { id: 1, title: String::from("Read") };
println!("{task:?}"); // Task { id: 1, title: "Read" }
```

### Checkpoint 3

<div class="quiz" data-answer="a" data-explanation="The Debug derive supplies the formatting used by the {:?} placeholder.">
<p>What enables <code>println!("{task:?}")</code> for this struct?</p>
<label><input type="radio" name="structs-1-debug" value="a"> <code>#[derive(Debug)]</code> on <code>Task</code></label><br>
<label><input type="radio" name="structs-1-debug" value="b"> Making every field mutable</label><br>
<label><input type="radio" name="structs-1-debug" value="c"> Converting <code>id</code> to a String</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Complete runnable example

This file combines construction, shorthand, mutation, borrowing, debug
printing, and a partial move:

```rust
--8<-- "examples/03-structs-enums-and-data-modeling/01-structs/main.rs"
```

From the repository root, with the stable Rust toolchain installed:

```bash
rustc examples/03-structs-enums-and-data-modeling/01-structs/main.rs -o /tmp/rust-task-structs
/tmp/rust-task-structs
```

Expected output:

```text
task: Task { id: 1, title: "Read the Rust book" }
title: Read the Rust book
updated: Task { id: 1, title: "Practice structs" }
moved title: Practice structs
remaining id: 1
```

The demo's README in `examples/03-structs-enums-and-data-modeling/01-structs/`
records its prerequisites and limitations.

---

## Small extensions

Struct update syntax copies or moves fields from an existing value. Here the
new title is supplied and `id`, a `Copy` value, comes from `first`:

```rust
let first = Task { id: 1, title: String::from("Read") };
let second = Task { title: String::from("Practice"), ..first };
println!("{} {}", first.id, second.title);
```

### Checkpoint 4

<div class="quiz" data-answer="b" data-explanation="The update copies the u32 id from first and uses a new String for second's title.">
<p>After the struct update above, which field comes from <code>first</code>?</p>
<label><input type="radio" name="structs-1-update" value="a"> Both fields</label><br>
<label><input type="radio" name="structs-1-update" value="b"> Only <code>id</code></label><br>
<label><input type="radio" name="structs-1-update" value="c"> Only <code>title</code></label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

A tuple struct gives a value its own type without naming its fields. In
`TaskId(u32)`, the parentheses declare one field of type `u32`. Construct it
with `TaskId(1)` and read its first field with `.0`: the zero is the field's
**position**, while `1` is the task ID stored there. A tuple struct with two
fields would use `.0` and `.1`. Use named fields for `Task`, where `id` and
`title` have different meanings.

```rust
struct TaskId(u32);

let id = TaskId(1);
println!("task ID: {}", id.0); // task ID: 1
```

A unit-like struct has no fields or stored values. For example,
`struct Marker;` declares a type whose value can be written as `Marker`.

---

## Exercise

1. Add a `priority: u8` field to `Task` and update every construction site.
2. Print the priority, then change it through a mutable `Task` binding.
3. Change the title through the same binding and check the full value with `Debug`.
4. Comment out `let title = task.title;` in the demo and compare which later
   uses of `task` become possible. Restore it afterward.

Expected result: the program compiles, prints the new priority, and still
shows the updated title. A construction missing `priority` should produce a
compiler diagnostic that points to the missing field.

### Final checkpoint

<div class="quiz" data-answer="c" data-explanation="A named-field struct keeps a task's related, differently typed data together under clear field names.">
<p>Why is a named-field struct a good starting model for a task?</p>
<label><input type="radio" name="structs-1-final" value="a"> It makes all fields globally mutable</label><br>
<label><input type="radio" name="structs-1-final" value="b"> It automatically stores every task in a file</label><br>
<label><input type="radio" name="structs-1-final" value="c"> It groups the identifier and title with explicit names and types</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

Next: [Topic 2, Methods and associated functions](index.md#2-methods-and-associated-functions)
(planned).
