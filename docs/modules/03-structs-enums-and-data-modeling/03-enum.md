# Enums: representing alternatives

## By the end, you will be able to

- Define enum types with variants and construct values of each variant.
- Choose between unit-like, tuple-like, and struct-like variants.
- Store different data in different variants.
- Distinguish between "no value" and "a value of another type" using enums.
- Use pattern matching to handle enum cases exhaustively.

Prerequisite: [Topic 2: Methods and associated functions](02-methods.md).
This lesson introduces enums and extends the `Task` example from earlier lessons.

---

## What is an enum?

An `enum` (enumeration) is a type that lists its possible values. Unlike C++ where enums were just integer constants, Rust enums can hold actual data and are strongly typed.

### A simple boolean alternative

First, consider the task status: pending, completed, or cancelled. You could use integers, but that loses clarity:

```rust
const PENDING: u8 = 1;
const COMPLETED: u8 = 2;
const CANCELLED: u8 = 3;

fn get_task_status() -> u8 { 1 }

let status = get_task_status();
// Is it PENDING? Is it COMPLETED?
if status == PENDING {
    println!("Pending");
} else if status == COMPLETED {
    println!("Completed");
} else {
    // What if it's 99?
    println!("Unknown");
}
```

An enum makes the alternatives explicit and forces you to handle each case:

```rust
enum TaskStatus {
    Pending,
    Completed,
    Cancelled,
}

fn get_task_status() -> TaskStatus { TaskStatus::Pending }

match get_task_status() {
    TaskStatus::Pending => println!("Pending"),
    TaskStatus::Completed => println!("Completed"),
    TaskStatus::Cancelled => println!("Cancelled"),
}
```

Rust will compile an error if you miss a case. This is called **exhaustive matching**.

### Defining an enum

```rust
enum TaskStatus {
    Pending,
    Completed,
    Cancelled,
}
```

Each possible value is a **variant**. The variants above are **unit-like**: they carry no data.

---

## Unit-like, tuple-like, and struct-like variants

### Unit-like variants

Unit-like variants carry no data. They're like `bool`'s `true` and `false`:

```rust
enum Status {
    Active,
    Inactive,
}

let s = Status::Active;
```

### Tuple-like variants

Tuple-like variants store data as a tuple:

```rust
enum Point {
    X(f64),
    Y(f64),
    Z(f64, f64),
    XY(f64, f64),
}

let x = Point::X(3.14);
let y = Point::Y(2.71);
let z = Point::Z(1.0, 2.0);
let xy = Point::XY(0.0, 1.0);
```

Access data with `.0`, `.1`, etc. (field positions, not names):

```rust
println!("x = {}", x.0); // x = 3.14
println!("z = {:?}", z.0, z.1);
```

### Struct-like variants

Struct-like variants store data in named fields:

```rust
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

enum Point {
    X(Point3D),
    Y(Point3D),
    Z(Point3D),
    XY(Point3D),
}

let x = Point::X(Point3D { x: 3.14, y: 0.0, z: 0.0 });
let xy = Point::XY(Point3D { x: 0.0, y: 1.0, z: 0.0 });
```

Access with field names:

```rust
println!("x = {}", x.X.x); // x = 3.14
println!("xy = {:?}", xy.XY.x, xy.XY.y);
```

**Choosing:** Use unit-like for simple flags, tuple-like for ordered data without names, struct-like for named, related data.

---

## Representing "no value" with enums

Consider a function that returns the first task in a list. If the list is empty, what should it return?

### Option: missing values

Rust provides `Option<T>` for "some value of type `T` or nothing":

```rust
let tasks: Vec<Task> = vec![];
let first = tasks.get(0); // Option<&Task>

if let Some(task) = first {
    println!("First: {}", task.title);
} else {
    println!("No first task");
}
```

But `Option` is a built-in, and sometimes you need your own enum.

### Result: success or failure

Consider a function that opens a file. It succeeds with the file handle, or fails with an error:

```rust
fn open_log(log_path: &str) -> Result<String, std::io::Error> {
    // Returns Ok(content) on success, Err(e) on failure
    std::fs::read_to_string(log_path)
}

let content = open_log("app.log")?; // Propagate errors with ?
```

`Result<T, E>` has two variants: `Ok(T)` for success and `Err(E)` for failure.

### Defining your own enums

You can define custom enums for domain-specific alternatives:

```rust
enum PaymentMethod {
    CreditCard(String), // card number as String
    BankTransfer(f64),  // amount as f64
    Crypto(String, f64), // coin and amount
}

fn process_payment(method: &str, amount: f64) -> PaymentMethod {
    match method {
        "credit" => PaymentMethod::CreditCard(String::from("4242")),
        "bank" => PaymentMethod::BankTransfer(amount),
        "crypto" => PaymentMethod::Crypto(String::from("BTC"), amount),
        _ => panic!("Unsupported payment method"),
    }
}

let payment = process_payment("crypto", 100.0);
match payment {
    PaymentMethod::CreditCard(card) => println!("Card: {}", card),
    PaymentMethod::BankTransfer(amount) => println!("Transfer: {}", amount),
    PaymentMethod::Crypto(coin, amt) => println!("{}: {}", coin, amt),
}
```

Each variant can carry different data types, making the type system enforce correctness.

---

## Pattern matching with enums

Pattern matching (`match`) is the primary way to handle enums.

### Exhaustive matching

Rust requires all cases to be covered:

```rust
enum Status {
    Active,
    Inactive,
}

let s = Status::Active;

match s {
    Status::Active => println!("Active"),
    Status::Inactive => println!("Inactive"),
    // Missing case! Compiler error.
}
```

### Ignoring cases with `_`

Use `_` to handle "any other case":

```rust
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

let op = Operation::Add(1.0, 2.0);

match op {
    Operation::Add(a, b) => println!("{}", a + b),
    Operation::Subtract(a, b) => println!("{}", a - b),
    Operation::Multiply(a, b) => println!("{}", a * b),
    Operation::Divide(a, b) => println!("{}", a / b),
    Operation::Other => println!("Unknown operation"),
}
```

### Partial destructuring with `..`

Ignore all fields except some:

```rust
enum Status {
    Active,
    Inactive,
    Archived { created_at: u64, user_id: u64 },
}

let s = Status::Archived { created_at: 123, user_id: 456 };

match s {
    Status::Archived { created_at, .. } => println!("Created: {}", created_at),
    Status::Active | Status::Inactive => println!("Active or inactive"),
}
```

### Match guards

Add conditions with `if`:

```rust
enum TaskStatus {
    Completed,
    Pending,
    Cancelled,
}

let status = TaskStatus::Completed;

match status {
    TaskStatus::Completed if status == TaskStatus::Completed => println!("Completed"),
    TaskStatus::Pending => println!("Pending"),
    TaskStatus::Cancelled => println!("Cancelled"),
}
```

### If let for single cases

Use `if let` for "only this case, ignore others":

```rust
enum TaskStatus {
    Completed,
    Pending,
    Cancelled,
}

let status = TaskStatus::Completed;

if let TaskStatus::Completed = status {
    println!("Task completed!");
} // Pending and Cancelled are ignored
```

---

## Extending the Task model with TaskStatus

Now we extend the `Task` struct with a status enum.

```rust
enum TaskStatus {
    Pending,
    Completed,
    Cancelled,
}

struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}

let task = Task {
    id: 1,
    title: String::from("Read the Rust book"),
    status: TaskStatus::Pending,
};

match task.status {
    TaskStatus::Pending => println!("Pending"),
    TaskStatus::Completed => println!("Completed"),
    TaskStatus::Cancelled => println!("Cancelled"),
}
```

### Adding methods that change status

```rust
impl Task {
    fn complete(&mut self) {
        self.status = TaskStatus::Completed;
    }

    fn cancel(&mut self) {
        self.status = TaskStatus::Cancelled;
    }
}

let mut task = Task {
    id: 1,
    title: String::from("Read the Rust book"),
    status: TaskStatus::Pending,
};

task.complete();
match task.status {
    TaskStatus::Pending => println!("Pending"),
    TaskStatus::Completed => println!("Completed"),
    TaskStatus::Cancelled => println!("Cancelled"),
}
```

---

## Complete runnable example

```rust
--8<-- "examples/03-structs-enums-and-data-modeling/03-enums/main.rs"
```

From the repository root:

```bash
rustc examples/03-structs-enums-and-data-modeling/03-enums/main.rs -o /tmp/rust-task-enums
/tmp/rust-task-enums
```

Expected output:

```text
Created task: Task { id: 1, title: "Read the Rust book", status: Pending }
Status: Pending
Completed!
Status: Completed
```

The demo's README in `examples/03-structs-enums-and-data-modeling/03-enums/` records its prerequisites and limitations.

---

## Checkpoints

### Checkpoint 1: Variant choice

<div class="quiz" data-answer="c" data-explanation="Struct-like variants store data with meaningful field names, which is clearer than tuple-like variants.">
<p>Which variant choice best represents a `Point` with named coordinates?</p>
<label><input type="radio" name="enum-1-variant" value="a"> <code>struct Point { x: f64, y: f64 }</code></label><br>
<label><input type="radio" name="enum-1-variant" value="b"> <code>enum Point { X(f64), Y(f64) }</code></label><br>
<label><input type="radio" name="enum-1-variant" value="c"> <code>enum Point { Struct { x: f64, y: f64 } }</code></label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

### Checkpoint 2: Option vs custom enum

<div class="quiz" data-answer="c" data-explanation="Result<T, E> is for success/failure outcomes, while Option<T> is for optional values.">
<p>When should you use a custom enum instead of `Option<T>`?</p>
<label><input type="radio" name="enum-2-option" value="a"> When the value is missing</label><br>
<label><input type="radio" name="enum-2-option" value="b"> When you need to distinguish between missing and an error</label><br>
<label><input type="radio" name="enum-2-option" value="c"> When the value is present</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

### Checkpoint 3: Exhaustive matching

<div class="quiz" data-answer="c" data-explanation="Rust requires all enum variants to be handled in a match expression.">
<p>What happens if you miss a variant in a `match` on a custom enum?</p>
<label><input type="radio" name="enum-3-match" value="a"> The program compiles but may panic at runtime</label><br>
<label><input type="radio" name="enum-3-match" value="b"> The program compiles with a warning</label><br>
<label><input type="radio" name="enum-3-match" value="c"> The program fails to compile</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

### Checkpoint 4: Pattern destructuring

<div class="quiz" data-answer="b" data-explanation="Field names in pattern destructuring must match the struct variant field names.">
<p>Given <code>enum Status { Active, Inactive { id: u32 } }</code>, which pattern works?</p>
<label><input type="radio" name="enum-4-pattern" value="a"> <code>Status::Inactive(id) => println!("{}", id)</code></label><br>
<label><input type="radio" name="enum-4-pattern" value="b"> <code>Status::Inactive { id } => println!("{}", id)</code></label><br>
<label><input type="radio" name="enum-4-pattern" value="c"> <code>Status::Inactive => println!("Inactive")</code></label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

### Checkpoint 5: Final - when to use enums

<div class="quiz" data-answer="c" data-explanation="Enums are best when there are discrete, distinct alternatives that may carry different data.">
<p>When is an enum the right tool?</p>
<label><input type="radio" name="enum-5-final" value="a"> When you need a floating-point range</label><br>
<label><input type="radio" name="enum-5-final" value="b"> When alternatives can have arbitrary data of the same type</label><br>
<label><input type="radio" name="enum-5-final" value="c"> When there are discrete, distinct alternatives that may carry different data</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Exercises

1. **Add data to TaskStatus**: Create a variant `Completed { description: String }` that stores additional information about what was completed. Update all matching code and the runnable example.

2. **Add a payment method enum**: Define a `PaymentMethod` enum with variants for credit card, bank transfer, and crypto. Create a function that returns the appropriate variant based on input.

3. **Pattern matching practice**: Write a function that takes a custom enum with multiple variants and returns a descriptive string for each case, using pattern matching.

4. **Combine with Option**: Create an enum `TaskResult` that can be `Pending`, `Completed`, or `None` (using `None` or a custom variant). Write code that handles all three cases.

---

## Summary

- Enums represent discrete alternatives: `enum Status { Pending, Completed, Cancelled }`
- Variants can be unit-like, tuple-like, or struct-like, storing different data
- Pattern matching with `match` handles all enum cases; Rust enforces exhaustiveness
- Use `Option<T>` for "missing values" and `Result<T, E>` for "success/failure"
- Custom enums provide clear, type-safe alternatives for domain-specific needs
- Struct-like variants with named fields offer the clearest representation for complex data

Next: [Topic 4: Pattern matching and destructuring](04-pattern-matching.md).
