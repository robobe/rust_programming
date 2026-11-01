# Slices and API boundaries

## By the end, you will be able to

- Explain a slice as a borrowed view of part or all of a collection.
- Use `&str` for read-only text APIs.
- Use `&[T]` for functions that read sequences of values.
- Return a slice tied to an input instead of returning a dangling reference.
- Recognize when `AsRef` makes a generic API useful.

---

## What is a slice?

A slice is a borrowed view into existing data. It does not own the data and it
does not copy the elements. Conceptually, a slice contains a pointer to the
first item and a length:

```rust
let numbers = [10, 20, 30, 40];
let middle: &[i32] = &numbers[1..3];

println!("{middle:?}"); // [20, 30]
```

The range `1..3` includes index `1` and stops before index `3`. The original
array owns all four values; `middle` only views the two selected values.

![Conceptual memory layout of string and collection slices](images/slices-memory-layout.svg)

The diagram is conceptual. A slice is shown as a pointer and length, not as a
second allocation or an owner.

## Checkpoint 1

<div class="quiz" data-answer="b" data-explanation="A slice stores a view into existing data, so it needs a pointer and length but does not own or copy the elements.">
<p>What does a slice contain conceptually?</p>
<label><input type="radio" name="slice-view" value="a"> A new owned copy of every selected element</label><br>
<label><input type="radio" name="slice-view" value="b"> A pointer and length describing a borrowed range</label><br>
<label><input type="radio" name="slice-view" value="c"> Only the allocation capacity of a collection</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result"></p>
</div>

## `&str`: a text API boundary

`String` owns growable UTF-8 text. `&str` is a borrowed view of UTF-8 text. A
read-only function should usually accept `&str` instead of `&String` because
both a `String` and a string literal can provide a `&str`:

```rust
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

let owned = String::from("Rust makes views useful");
let literal = "Rust is fast";

println!("{}", first_word(&owned));
println!("{}", first_word(literal));
```

The returned `&str` points into the input. It is valid only while the input
text remains valid, so the function cannot outlive or out-own its argument.

## Checkpoint 2

<div class="quiz" data-answer="c" data-explanation="&str accepts a borrowed view from both owned String values and string literals, making the function less restrictive.">
<p>Why is <code>&amp;str</code> usually preferable to <code>&amp;String</code> for read-only text?</p>
<label><input type="radio" name="str-api" value="a"> It always creates a new String</label><br>
<label><input type="radio" name="str-api" value="b"> It transfers ownership of the caller's text</label><br>
<label><input type="radio" name="str-api" value="c"> It accepts text views from String values and literals</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result"></p>
</div>

## `&[T]`: a sequence API boundary

The same idea applies to collections. A function that only reads values should
usually accept `&[T]`, not `&Vec<T>`:

```rust
fn sum_values(values: &[i32]) -> i32 {
    values.iter().sum()
}

let array = [1, 2, 3, 4];
let vector = vec![5, 6, 7];

println!("{}", sum_values(&array));
println!("{}", sum_values(&vector));
println!("{}", sum_values(&array[1..3]));
```

One signature accepts an array, a `Vec`, or a sub-slice. The caller keeps
ownership, and the function does not need to know how the sequence was stored.

## Checkpoint 3

<div class="quiz" data-answer="a" data-explanation="&[T] is a borrowed view that works with arrays, Vec values, and ranges within either collection.">
<p>What can a function accepting <code>&amp;[i32]</code> read?</p>
<label><input type="radio" name="slice-api" value="a"> An array, a Vec, or a sub-slice of either</label><br>
<label><input type="radio" name="slice-api" value="b"> Only a Vec with exactly one capacity</label><br>
<label><input type="radio" name="slice-api" value="c"> Only an owned array copied into the function</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result"></p>
</div>

## Returning a slice safely

A function may return a slice when the slice comes from an input that stays
alive. This is a common zero-allocation pattern:

```rust
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

let sentence = String::from("borrowed result");
let word = first_word(&sentence);
println!("{word}");
```

The result is tied to `sentence`. Rust rejects a function that creates a local
`String` and returns a reference to it because the local would be dropped on
function return. If the function creates new data, return an owned `String`
instead.

## Checkpoint 4

<div class="quiz" data-answer="b" data-explanation="A returned slice is safe when it points into input data whose lifetime covers the returned reference.">
<p>When is returning a <code>&amp;str</code> safe?</p>
<label><input type="radio" name="slice-return" value="a"> When it points to a local String created inside the function</label><br>
<label><input type="radio" name="slice-return" value="b"> When it points into input data that remains alive</label><br>
<label><input type="radio" name="slice-return" value="c"> Only when the text is cloned first</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result"></p>
</div>

## `AsRef`: accept several view providers

Sometimes an API should accept several types that can cheaply provide the same
borrowed view. `AsRef<str>` is useful for a small helper like this:

```rust
fn print_text<T: AsRef<str>>(text: T) {
    println!("{}", text.as_ref());
}

print_text("a literal");
print_text(String::from("owned text"));
```

The generic parameter owns its input for the duration of the call, but the
function immediately borrows a `&str` view from it. Do not add a generic bound
just to make an API look flexible: use `&str` when that is all the function
needs. `Borrow` and more advanced generic bounds are useful when a collection
or lookup API needs stronger equivalence guarantees.

## Complete runnable example

The lesson displays the same source file used by the matching demo:

```rust
--8<-- "examples/02-ownership-and-borrowing/07-slices-and-api-boundaries/main.rs"
```

Run it from the repository root:

```bash
rustc examples/02-ownership-and-borrowing/07-slices-and-api-boundaries/main.rs \
  -o /tmp/rust-slices
/tmp/rust-slices
```

Expected output:

```text
first word: Rust
array sum: 10
middle sum: 5
Vec sum: 18
text: a literal
text: Rust makes views useful
```

## Exercise

1. Change `sum_values` to return the maximum value.
2. Add a function that returns the last word as a `&str`.
3. Try passing a `Vec<String>` to a function accepting `&[String]`.
4. Change `print_text` to accept only `&str`, then compare the call sites.

---

## Checkpoint 5

<div class="quiz" data-answer="c" data-explanation="Slices are borrowed views, so they make read-only APIs flexible without transferring ownership or cloning data.">
<p>Which statement best summarizes slices at an API boundary?</p>
<label><input type="radio" name="slice-summary" value="a"> They force every caller to use the same owning collection type</label><br>
<label><input type="radio" name="slice-summary" value="b"> They move the caller's data into the function and prevent reuse</label><br>
<label><input type="radio" name="slice-summary" value="c"> They provide flexible borrowed views without owning or copying the data</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result"></p>
</div>

## Next

Continue with [lifetimes: describing relationships](index.md#8-lifetimes-describing-relationships).
