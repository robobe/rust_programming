# Topic 1: Generic functions

By the end, you will be able to

- Define **[generic functions and type parameters](#one-function-several-types)**.
- Use **[type inference](#inference-and-explicit-type-arguments)** and **[explicit type arguments](#inference-and-explicit-type-arguments)**.
- Distinguish **[owned and borrowed generic inputs](#borrow-without-copying)**.
- Explain why operations on a type parameter may require **[trait bounds](#what-an-unconstrained-type-cannot-do)**.

Prerequisites: [Module 3 task model](../03-structs-enums-and-data-modeling/07-task-model.md)
and [Module 2 slices](../02-ownership-and-borrowing/07-slices-and-api-boundaries.md).
Helper functions below belong outside `main`; calls and local bindings belong
inside it. The complete runnable demo includes both.

---

## One function, several types

Sometimes behavior is identical even though the data type changes. Instead of
writing separate integer and string versions, give the function a type parameter.
Here the behavior is simply to return the input unchanged:

```rust
fn identity<T>(value: T) -> T {
    value
}
```

`T` is a placeholder chosen for each concrete use of the function. It is not
a value passed at runtime and does not mean that values change type while the
function runs. The parameter and result use the same `T`, so they have the same
type. The tail expression `value` returns the input without a clone.

- **`<T>`** — type-parameter declaration: `fn identity<T>` introduces the type name.
- **`:`** — parameter type marker: `value: T` gives the input that type.
- **`->`** — return-type marker: `-> T` gives the result the same type.

```rust
let number = identity(7_i32);
let text = identity(String::from("Rust"));
println!("{number}, {text}"); // 7, Rust
```

Generics preserve ownership rules. Passing an owned `String` moves it into
`identity`, which returns ownership to the caller. Passing an `i32` copies its
value because that concrete type implements `Copy`; `identity` itself does not
require every possible `T` to be `Copy`.
<div class="quiz" data-answer="a" data-explanation="The same type parameter appears in the input and output, so a String input yields a String result.">
<p>If identity receives a String, what type does it return?</p>
<label><input type="radio" name="m4-t1-q1" value="a"> A String</label><br>
<label><input type="radio" name="m4-t1-q1" value="b"> Always i32</label><br>
<label><input type="radio" name="m4-t1-q1" value="c"> A runtime placeholder named T</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Inference and explicit type arguments

The compiler can infer `T` from an argument, or you can choose it explicitly:

```rust
let inferred = identity(7_i32);
let explicit = identity::<u8>(9);
assert_eq!(inferred, 7);
assert_eq!(explicit, 9);
```

- **`::<u8>`** — explicit type-argument syntax, often called the turbofish: `identity::<u8>(9)` selects `T = u8`.
- **`(9)`** — value-argument list: the number 9 is the input, not a type.

The explicit choice must agree with the value: `identity::<u8>(300)` cannot
compile because 300 does not fit `u8`. Without an annotation or a constraining
use, an ordinary integer literal commonly defaults to `i32`. If no type can
be determined, provide a type argument or a suitable annotation.
<div class="quiz" data-answer="b" data-explanation="The turbofish supplies a type argument; the parentheses contain the value argument.">
<p>In <code>identity::&lt;u8&gt;(9)</code>, which part chooses the type?</p>
<label><input type="radio" name="m4-t1-q2" value="a"> (9)</label><br>
<label><input type="radio" name="m4-t1-q2" value="b"> ::<u8></label><br>
<label><input type="radio" name="m4-t1-q2" value="c"> The variable name receiving the result</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Borrow without copying

Generic functions can borrow instead of taking ownership. Return a reference
to the first element of a slice, or `None` for empty input:

```rust
fn first<T>(values: &[T]) -> Option<&T> {
    values.first()
}
```

This works with integers and owned strings without a `Clone` or `Copy` bound.
It returns a borrowed view of an existing element, not a newly owned element.
The compiler ties that reference to the input; it cannot outlive the data.

```rust
let words = [String::from("Read"), String::from("Practice")];
assert_eq!(first(&words), Some(&words[0]));
assert_eq!(first::<String>(&[]), None);
println!("still owned: {}", words[0]); // still owned: Read
```

`&[]` has no elements to suggest an element type here, so the explicit
`::<String>` resolves it. These functions are constant-time examples: returning
one value or inspecting the first slice element does not scan the collection.
<div class="quiz" data-answer="c" data-explanation="Borrowing an element does not duplicate its text or take ownership away from the collection.">
<p>Why can first read a slice of String values without requiring Clone?</p>
<label><input type="radio" name="m4-t1-q3" value="a"> It converts every String into an integer</label><br>
<label><input type="radio" name="m4-t1-q3" value="b"> All String values implement Copy</label><br>
<label><input type="radio" name="m4-t1-q3" value="c"> It returns a reference to an existing element</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## What an unconstrained type cannot do

A type parameter does not promise that every operation exists. This deliberate
counterexample **does not compile**:

```rust
fn add<T>(left: T, right: T) -> T {
    left + right
}
```

**`+`** is addition, but an arbitrary `T` need not support addition. Rust checks
the generic function using its declared guarantees; calling it only with
integers does not repair its missing requirement. The compiler reports E0369.
A **trait bound** states the required capability. Topic 4 will teach how to
write those bounds; for now, choose operations that are valid for unconstrained
inputs, such as moving them or borrowing them.

Likewise, printing an arbitrary `T` with `{:?}` requires a `Debug` bound. The
identity demo prints its concrete results in `main`, where their types are known.

---

## Complete runnable example

```rust
--8<-- "examples/04-traits-and-generics/01-generic-functions/main.rs"
```

Run from the repository root:

```bash
rustc examples/04-traits-and-generics/01-generic-functions/main.rs -o /tmp/rust-generic-functions
/tmp/rust-generic-functions
rustc --test examples/04-traits-and-generics/01-generic-functions/main.rs -o /tmp/rust-generic-functions-tests
/tmp/rust-generic-functions-tests
rustfmt --check examples/04-traits-and-generics/01-generic-functions/main.rs
```

Expected output, followed by two passing tests from the test command:

```text
identity: 7, Rust, 9
first word: Some("Read")
still owned: Read
```

---

## Exercises

Attempt each task before opening its solution. Solutions are standalone programs.
Save one in a temporary `.rs` file, compile with `rustc`, and run it.

### 1. Return two owned inputs

Write `pair<T>(left: T, right: T) -> (T, T)`. Return both inputs without cloning.
Test integer inputs and two owned strings. Must the two inputs have the same type?

??? success "Solution: preserve both values in a tuple"
    ```rust
    fn pair<T>(left: T, right: T) -> (T, T) {
        (left, right)
    }

    fn main() {
        assert_eq!(pair(1, 2), (1, 2));
        let (left, right) = pair(String::from("Read"), String::from("Practice"));
        assert_eq!(left, "Read");
        assert_eq!(right, "Practice");
        println!("{left}, {right}");
    }
    ```

    Both arguments use the same `T`, so they must have the same type. Each
    string moves into the function and then into its returned tuple.
    Expected output: `Read, Practice`.

---

### 2. Borrow the last element

Write `last<T>(values: &[T]) -> Option<&T>` using the slice's `last()` method.
Test integers, owned strings, and empty input without cloning.

??? success "Solution: delegate to the slice method"
    ```rust
    fn last<T>(values: &[T]) -> Option<&T> {
        values.last()
    }

    fn main() {
        assert_eq!(last(&[1, 2, 3]), Some(&3));
        let words = [String::from("Read"), String::from("Practice")];
        assert_eq!(last(&words), Some(&words[1]));
        assert_eq!(last::<String>(&[]), None);
        println!("last: {:?}", last(&words));
        assert_eq!(words.len(), 2);
    }
    ```

    The result borrows the last element. Empty input returns `None`, and the
    caller keeps the collection. Expected output: `last: Some("Practice")`.

---

## Check your understanding
<div class="quiz" data-answer="a" data-explanation="Generics reuse typed behavior; they do not grant operations or bypass moves and borrows.">
<p>What does a generic function promise?</p>
<label><input type="radio" name="m4-t1-q4" value="a"> It works for types satisfying its declared requirements and preserves ownership rules</label><br>
<label><input type="radio" name="m4-t1-q4" value="b"> Every T supports arithmetic and printing</label><br>
<label><input type="radio" name="m4-t1-q4" value="c"> All arguments are cloned automatically</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Topic 2: Generic structs, enums, and impl blocks](index.md#2-generic-structs-enums-and-impl-blocks)
(planned). Return to the [Module 4 index](index.md).
