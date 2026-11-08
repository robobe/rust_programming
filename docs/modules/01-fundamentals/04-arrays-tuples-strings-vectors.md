# Topic 4: Arrays, tuples, strings, and vectors

By the end, you will be able to

- Choose arrays, tuples, strings, and vectors.
- Use indexing, methods, and tuple destructuring.
- Distinguish byte lengths from Unicode scalar counts.

Prerequisite: [Topic 3: Scalar types and operators](03-scalar-types-and-operators.md).

---

## Store related values

An array has a fixed number of elements of one type. A tuple groups a fixed
set of values that may have different types. A vector is a growable sequence
of one element type.

```rust
let fixed: [i32; 3] = [2, 4, 6];
let summary: (usize, i64) = (fixed.len(), 12);
let (count, total) = summary; // Destructure a tuple.
println!("first: {}, count: {count}, total: {total}", fixed[0]);

let mut numbers: Vec<i32> = vec![3, -1, 8];
numbers.push(2);
println!("length: {}, values: {numbers:?}", numbers.len());
```

Indexes start at zero. `fixed[3]` is out of bounds for a three-element array.
Indexing outside a sequence's bounds panics; `numbers.get(3)` instead returns
an `Option`, representing either a present or missing element. Module 3
explains `Option` in detail.

```rust
let greeting: &str = "Hello"; // Borrowed text, here a string literal.
let mut title = String::from(greeting); // Owned, growable UTF-8 text.
title.push_str(", Rust");
println!("{title}");
```

`String` and `Vec` own their stored data; `&str` and `&[T]` provide
borrowed views. For now, read `&numbers` as “let this function read my
numbers.” Module 2 develops the ownership rules.

Text is UTF-8: `"é".len()` is 2 bytes, while `"é".chars().count()` is 1
Unicode scalar value. Rust does not allow `text[0]` on a string.
Use `.chars()` when iterating over Unicode scalar values.

### Checkpoint 2

<div class="quiz" data-answer="c" data-explanation="Vec supports adding elements, while an array has a fixed length.">
<p>Which choice fits a list of numbers that grows as values arrive?</p>
<label><input type="radio" name="fundamentals-2" value="a"> A tuple with a fixed set of fields</label><br>
<label><input type="radio" name="fundamentals-2" value="b"> A fixed-length array</label><br>
<label><input type="radio" name="fundamentals-2" value="c"> Vec&lt;i32&gt; with push</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

<div class="quiz" data-answer="a" data-explanation="String length is measured in bytes; é uses two UTF-8 bytes.">
<p>Why does <code>"é".len()</code> return 2?</p>
<label><input type="radio" name="fundamentals-t4-q2" value="a"> It counts UTF-8 bytes</label><br>
<label><input type="radio" name="fundamentals-t4-q2" value="b"> It counts two words</label><br>
<label><input type="radio" name="fundamentals-t4-q2" value="c"> All characters occupy two bytes</label><br>
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

## Exercise

Add a number to the vector and update the length assertion. Create a tuple
containing your name and age, then destructure and print it. Compare the byte
length and character count of `"café"`. Do not use integer indexing on text.

## Check your understanding
<div class="quiz" data-answer="b" data-explanation="Tuples can group different types; arrays and vectors have one element type.">
<p>Which structure groups a fixed set of potentially different types?</p>
<label><input type="radio" name="fundamentals-t4-q3" value="a"> An array</label><br>
<label><input type="radio" name="fundamentals-t4-q3" value="b"> A tuple</label><br>
<label><input type="radio" name="fundamentals-t4-q3" value="c"> A vector</label><br>
<button class="quiz-check">Check answer</button>
<p class="quiz-result" aria-live="polite"></p>
</div>

---

## Next

Continue with [Topic 5: Functions, statements, and expressions](05-functions-statements-expressions.md). Return to the [topic index](index.md).
