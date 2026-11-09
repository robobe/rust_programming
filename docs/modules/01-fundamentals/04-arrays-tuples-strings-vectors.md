# Topic 4: Arrays, tuples, strings, and vectors

By the end, you will be able to

- Choose **[arrays](#arrays-fixed-length-sequences)**, **[tuples](#tuples-group-different-types)**, **[strings](#strings-owned-text-and-borrowed-text)**, and **[vectors](#vectors-growable-sequences)**.
- Use **[indexing](#arrays-fixed-length-sequences)**, **[tuple destructuring](#tuples-group-different-types)**, and **[collection methods](#vectors-growable-sequences)**.
- Distinguish **[UTF-8 byte lengths and Unicode scalar counts](#strings-owned-text-and-borrowed-text)**.

Prerequisite: [Topic 3: Scalar types and operators](03-scalar-types-and-operators.md).
The short snippets belong inside `main`; the complete runnable example follows.

---

## Arrays: fixed-length sequences

An array stores a fixed number of values of one type. Its length is part of
its type, so `[i32; 3]` and `[i32; 4]` are different types. Use an array when
the number of elements is known and does not need to grow.

```rust
let fixed: [i32; 3] = [2, 4, 6];
println!("first: {}, length: {}", fixed[0], fixed.len());
// first: 2, length: 3
```

- **`[T; N]`** — array type: `[i32; 3]` means three `i32` values.
- **`[value; N]`** — repetition expression: `[0; 3]` creates `[0, 0, 0]`.
- **`[index]`** — indexing expression: `fixed[0]` reads the first element.

Indexes start at zero: this array has indices 0, 1, and 2. Out-of-bounds
indexing panics if executed; a statically obvious invalid access may be
rejected during compilation. `fixed.get(3)` instead returns `None`, an
`Option` value representing absence. Module 3 explains `Option` in detail.

A mutable array can change its elements, but it cannot grow: `mut` does not
change its fixed length.

<div class="quiz" data-answer="b" data-explanation="Indexing starts at zero; an array of length 3 has indices 0 through 2, regardless of mutability.">
<p>For <code>[2, 4, 6]</code>, which statement about indexing is correct?</p>
<label><input type="radio" name="fundamentals-t4-review-1" value="a"> Index 3 contains 6</label><br>
<label><input type="radio" name="fundamentals-t4-review-1" value="b"> Valid indices are 0, 1, and 2</label><br>
<label><input type="radio" name="fundamentals-t4-review-1" value="c"> Making the array mutable makes index 3 valid</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

### Array methods: length, reverse, and sort

Arrays support methods for inspecting and rearranging their elements. Most
of the methods below come from slices, which arrays can use automatically.

```rust
let mut numbers = [3, 1, 2];
println!("length: {}", numbers.len()); // length: 3

numbers.reverse();
println!("reversed: {numbers:?}"); // reversed: [2, 1, 3]

numbers.sort();
println!("sorted: {numbers:?}"); // sorted: [1, 2, 3]
```

!!! info "Reading println! and debug formatting"
    In `println!("sorted: {numbers:?}")`:

    - **`println!`** — macro that prints a line to the terminal.
    - **`{numbers:?}`** — placeholder that inserts `numbers` using debug formatting.
    - **`:?`** — debug-format specifier, useful for arrays and other compound values.
    - **`:#?`** — pretty debug-format specifier: `{numbers:#?}` spreads the output over multiple lines.

    ```rust
    let numbers = [1, 2, 3];
    println!("sorted: {numbers:?}");
    ```

    Output: `sorted: [1, 2, 3]`.
    Arrays support debug formatting, so use `{numbers:?}` rather than the
    ordinary display placeholder `{numbers}`.

`reverse()` and `sort()` change the original array, so the binding needs
`mut`. Reversing flips the current order; sorting puts values in ascending
order. Neither operation changes the array's length.

The following examples assume `numbers` is now `[1, 2, 3]`:

| Method | Purpose | Example or result |
| --- | --- | --- |
| `len()` | Count elements | `numbers.len()` returns `3` |
| `is_empty()` | Check whether length is zero | `numbers.is_empty()` returns `false` |
| `first()` | Borrow the first element, if present | `numbers.first()` returns `Some(&1)` |
| `last()` | Borrow the last element, if present | `numbers.last()` returns `Some(&3)` |
| `get(index)` | Borrow an element when the index is valid | `numbers.get(9)` returns `None` |
| `contains(&value)` | Check whether a value exists | `numbers.contains(&2)` returns `true` |
| `reverse()` | Reverse elements in place | `[1, 2, 3]` becomes `[3, 2, 1]` |
| `sort()` | Sort in place, preserving the original order of equal elements | `[3, 1, 2]` becomes `[1, 2, 3]` |
| `sort_unstable()` | Sort in place without guaranteeing equal elements keep their original order | Produces ascending order too |
| `iter()` | Visit elements through shared references | `numbers.iter()` yields `&i32` values |
| `iter_mut()` | Visit elements through mutable references | `numbers.iter_mut()` yields `&mut i32` values |

The sorting methods shown require elements with a total ordering, such as
integers. An array cannot grow with `push()`; use a vector for that.
[Module 2, Topic 6](../02-ownership-and-borrowing/06-borrowing-collections-and-iterators.md)
explains borrowed iteration, and [Module 3, Topic 5](../03-structs-enums-and-data-modeling/05-option.md)
explains the `Some` and `None` results used by safe access methods.

---

## Tuples: group different types

A tuple groups a fixed set of values whose types may differ. Positions have
meaning: a `(usize, i64)` summary can hold a count followed by a total.
Destructuring gives those positions readable local names.

```rust
let summary: (usize, i64) = (3, 12);
let (count, total) = summary;
println!("count: {count}, total: {total}"); // count: 3, total: 12
println!("count by position: {}", summary.0); // count by position: 3
```

- **`(T, U)`** — tuple type: `(usize, i64)` fixes the types of two positions.
- **`(a, b)`** — destructuring pattern: `let (count, total) = summary;` binds both fields.
- **`.0`** — tuple-field access: `summary.0` reads the first field; `.1` reads the second.

The integer fields in this example implement `Copy`, so destructuring them
leaves `summary` usable. Tuples containing owned strings can move those fields;
Module 2 develops that ownership distinction.

<div class="quiz" data-answer="c" data-explanation="The first pattern name receives the first field, and the second receives the second field.">
<p>After <code>let (count, total) = (3_usize, 12_i64);</code>, what does total hold?</p>
<label><input type="radio" name="fundamentals-t4-review-2" value="a"> 3, because names are sorted alphabetically</label><br>
<label><input type="radio" name="fundamentals-t4-review-2" value="b"> The whole tuple</label><br>
<label><input type="radio" name="fundamentals-t4-review-2" value="c"> 12, because destructuring follows field positions</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Strings: owned text and borrowed text

`String` owns growable UTF-8 text. `&str` is a borrowed text view; a string
literal such as `"Hello"` can provide one. You can create owned text from a
literal and append more text with `push_str`:

```rust
let greeting: &str = "Hello";
let mut title = String::from(greeting);
title.push_str(", Rust");
println!("{title}"); // Hello, Rust
```

`String::from` creates the owned string. `push_str` modifies it, so `title`
needs `mut`. A borrowed `&str` is not itself a growable string. Module 2 explains
why the owner and its borrowed views have different permissions.

UTF-8 uses a variable number of bytes per Unicode scalar value:

```rust
let text = "café";
println!("bytes: {}, scalars: {}", text.len(), text.chars().count());
// bytes: 5, scalars: 4
```

`len()` counts bytes. `chars().count()` counts Unicode scalar values, which
are not always the same as user-perceived characters: a displayed character
can contain multiple scalars. Rust rejects `text[0]` on strings; use `chars()`
to visit scalar values rather than guessing a byte boundary.

<div class="quiz" data-answer="a" data-explanation="The three ASCII letters use one byte each and é uses two bytes, giving five bytes but four Unicode scalar values.">
<p>Why does <code>"café".len()</code> return 5 while <code>.chars().count()</code> returns 4?</p>
<label><input type="radio" name="fundamentals-t4-review-3" value="a"> len counts UTF-8 bytes, and é occupies two bytes</label><br>
<label><input type="radio" name="fundamentals-t4-review-3" value="b"> Every letter occupies two bytes</label><br>
<label><input type="radio" name="fundamentals-t4-review-3" value="c"> chars removes the final letter</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Vectors: growable sequences

A `Vec<T>` stores a growable sequence of one element type. Unlike an array,
its length can change during execution. Use a vector when values arrive or
are removed while the program runs.

```rust
let mut numbers: Vec<i32> = vec![3, -1, 8];
numbers.push(2);
println!("length: {}, values: {numbers:?}", numbers.len());
// length: 4, values: [3, -1, 8, 2]
```

- **`<T>`** — type argument: `Vec<i32>` stores `i32` elements.
- **`vec![]`** — vector-construction macro: `vec![3, -1, 8]` creates initial elements.
- **`{:?}`** — debug formatting: `println!("{:?}", numbers)` shows the collection's contents.

`push(value)` appends one element and `len()` reports the current number of
elements. Both arrays and vectors support indexing and `get`; out-of-bounds
vector indexing can panic too. `mut` allows changes, but never permits mixing
unrelated element types in the same `Vec<i32>`.

---

### Vector methods: add, remove, and reorder

These methods change the original vector, so its binding needs `mut`:

```rust
let mut numbers = vec![3, 1, 2];
numbers.push(4);
println!("after push: {numbers:?}"); // [3, 1, 2, 4]

let last = numbers.pop();
assert_eq!(last, Some(4));
println!("popped: {last:?}, remaining: {numbers:?}"); // Some(4), [3, 1, 2]

numbers.insert(1, 9);
println!("after insert: {numbers:?}"); // [3, 9, 1, 2]

let removed = numbers.remove(0);
assert_eq!(removed, 3);
println!("removed: {removed}, remaining: {numbers:?}"); // 3, [9, 1, 2]

numbers.sort();
println!("sorted: {numbers:?}"); // [1, 2, 9]
numbers.reverse();
println!("reversed: {numbers:?}"); // [9, 2, 1]
```

`pop()` returns `Some(value)` when it removes the last element, or `None` if
the vector is empty. `remove(index)` returns the removed element directly,
but panics if the index is out of bounds. Both `insert` and `remove` shift
later elements, so their positions can change.

| Method | Purpose | Example |
| --- | --- | --- |
| `len()` | Count current elements | `numbers.len()` |
| `is_empty()` | Check whether there are no elements | `numbers.is_empty()` |
| `push(value)` | Append a value | `numbers.push(5)` |
| `pop()` | Remove the last value, if present | `numbers.pop()` |
| `insert(index, value)` | Insert before an index, or append at the length | `numbers.insert(0, 7)` |
| `remove(index)` | Remove an element and shift later elements left | `numbers.remove(0)` |
| `get(index)` | Borrow an element if the index is valid | `numbers.get(0)` |
| `contains(&value)` | Check whether a value exists | `numbers.contains(&2)` |
| `sort()` | Sort elements in ascending order | `numbers.sort()` |
| `reverse()` | Reverse the current order | `numbers.reverse()` |
| `clear()` | Remove all elements | `numbers.clear()` |

For `insert`, an index up to and including `len()` is valid; a larger index
panics. For `remove`, the index must be strictly less than `len()`.
Read-only methods such as `len`, `get`, and `contains` do not need `mut`.

---

`String` and `Vec` own their stored data; `&str` and `&[T]` provide borrowed
views. In the next module, read `&numbers` as “let this function read my
numbers” while the caller retains ownership.

<div class="quiz" data-answer="c" data-explanation="Vec can append elements with push; an array keeps the same length and a tuple keeps the same fields.">
<p>Which choice fits a list of numbers that grows as values arrive?</p>
<label><input type="radio" name="fundamentals-t4-review-4" value="a"> A tuple whose fields have fixed positions</label><br>
<label><input type="radio" name="fundamentals-t4-review-4" value="b"> A fixed-length array</label><br>
<label><input type="radio" name="fundamentals-t4-review-4" value="c"> Vec<i32> with push</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Runnable example

```rust
--8<-- "examples/01-fundamentals/04-arrays-tuples-strings-vectors/main.rs"
```

Run from the repository root:

```bash
rustc examples/01-fundamentals/04-arrays-tuples-strings-vectors/main.rs -o /tmp/rust-fundamentals-4
/tmp/rust-fundamentals-4
```

Expected output:

```text
count: 3, total: 12, values: [3, -1, 8, 2]
Hello, Rust
```


---

## Exercises

Try each task before opening its solution. Each solution is a standalone
program; save it to a temporary `.rs` file, compile with `rustc`, and run it.

### 1. Update an array element

Create `[2, 4, 6]`, replace the middle value with 10, and assert that the array
still has three elements.

??? success "Solution: change an element without changing the length"
    ```rust
    fn main() {
        let mut fixed = [2, 4, 6];
        fixed[1] = 10;
        assert_eq!(fixed, [2, 10, 6]);
        assert_eq!(fixed.len(), 3);
        println!("{fixed:?}");
    }
    ```

    Index 1 is the middle element. Mutation replaces its value while keeping
    the array length unchanged. Expected output: `[2, 10, 6]`.

---

### 2. Destructure a tuple

Create a tuple containing a name and age. Destructure it into two bindings,
assert their values, then print them.

??? success "Solution: name each tuple field"
    ```rust
    fn main() {
        let person: (&str, u8) = ("Alex", 25);
        let (name, age) = person;
        assert_eq!(name, "Alex");
        assert_eq!(age, 25);
        println!("name: {name}, age: {age}");
    }
    ```

    The tuple's fields have different types. Destructuring follows their
    order. Expected output: `name: Alex, age: 25`.

---

### 3. Inspect UTF-8 text

Create an owned string containing `"café"`. Assert its byte length and Unicode
scalar count, then append `"!"` and check the updated text.

??? success "Solution: count bytes and scalars separately"
    ```rust
    fn main() {
        let mut text = String::from("café");
        assert_eq!(text.len(), 5);
        assert_eq!(text.chars().count(), 4);
        text.push_str("!");
        assert_eq!(text, "café!");
        println!("{text}: {} bytes, {} scalars", text.len(), text.chars().count());
    }
    ```

    `é` uses two UTF-8 bytes. Appending ASCII `!` adds one byte and one scalar.
    Expected output: `café!: 6 bytes, 5 scalars`.

---

### 4. Grow the vector

Add 5 after the runnable example's final vector element. Update the length
assertion and check that the earlier values stay unchanged.

??? success "Solution: append one more element"
    ```rust
    fn main() {
        let mut numbers = vec![3, -1, 8];
        numbers.push(2);
        numbers.push(5);
        assert_eq!(numbers.len(), 5);
        assert_eq!(numbers, vec![3, -1, 8, 2, 5]);
        println!("{numbers:?}");
    }
    ```

    Each `push` appends one value without replacing earlier elements.
    Expected output: `[3, -1, 8, 2, 5]`.

---

## Check your understanding
<div class="quiz" data-answer="b" data-explanation="Arrays fix a homogeneous sequence length, tuples group fixed fields, String owns growable text, and Vec grows a homogeneous list.">
<p>Which choice matches each requirement?</p>
<label><input type="radio" name="fundamentals-t4-review-5" value="a"> Use arrays for growing lists and integer indexing for text</label><br>
<label><input type="radio" name="fundamentals-t4-review-5" value="b"> Use an array for fixed-length values, a tuple for mixed fields, String for growable text, and Vec for a growing list</label><br>
<label><input type="radio" name="fundamentals-t4-review-5" value="c"> Use tuples for every collection because their length can grow</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Topic 5: Functions, statements, and expressions](05-functions-statements-expressions.md). Return to the [topic index](index.md).
