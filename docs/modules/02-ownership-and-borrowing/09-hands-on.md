# Topic 9: Ownership hands-on review

## By the end, you will be able to

- Trace owners and drops through nested scopes.
- Repair a move error and explain the ownership transfer.
- Choose copying, moving, or cloning for a stated requirement.
- Complete and run a small program without relying on answer choices.
- Read and update a sub-slice through borrowed iteration.
- Return a borrowed word without cloning its text.
- Combine borrowed processing with a final ownership-consuming step.

Prerequisites: [Values and scope](01-values-variables-scope.md),
[Moves](02-move-semantics.md), [Copy and Clone](03-copy-and-clone.md),
[Shared borrowing](ownership-and-borrowing.md),
[Mutable borrowing](05-mutable-borrowing.md),
[Collections and iterators](06-borrowing-collections-and-iterators.md), and
[Slices](07-slices-and-api-boundaries.md).
This final module topic practices selected skills from **Topics 2.1–2.7**.
Lifetime annotations are outside this practice set.
Attempt each task before consulting the solution. Allow about 90–120 minutes including the advanced exercise.

---

## 1. Explain the terms in your own words

Learn this in [Topic 1: Values and scope](01-values-variables-scope.md), [Topic 2: Moves](02-move-semantics.md), and [Topic 3: Copy and Clone](03-copy-and-clone.md).

For each term, give a definition and point to a concrete line in one of the
exercises: **value, binding, owner, scope, drop, stack, heap, move, Copy, Clone**.
Explain why a `String` binding leaving scope does not always drop its original
string data: ownership may already have moved elsewhere.

!!! tip "Reminder: bindings and data"
    A `String` owns heap-backed text. Moving it transfers ownership without
    duplicating that text. Scope determines when a remaining owner is dropped.

---

## 2. Trace ownership before compiling

Learn this in [Topic 1: Values and scope](01-values-variables-scope.md), [Topic 2: Moves](02-move-semantics.md), and [Topic 3: Copy and Clone](03-copy-and-clone.md).

```rust
fn main() {
    let count = 3_u32;
    let other_count = count;
    let first = String::from("Read");
    {
        let second = first;
        let third = second.clone();
        println!("{second} {third} {count} {other_count}");
    }
}
```

Write a table with one row after each `let`: usable bindings, current owners,
and whether an independent heap allocation was created. Then predict which
owned strings are dropped at the inner closing brace. Compile to check your
predicted output, and explain why output alone cannot prove your ownership table.

??? success "Solution: trace ownership"
    Each row describes the state immediately after that `let`. Here, A and B
    label two independent heap allocations holding the text `Read`.

    | After statement | Usable bindings | String owners | New independent heap allocation? |
    | --- | --- | --- | --- |
    | `let count = 3_u32;` | `count` | None | No, `u32` needs no heap allocation |
    | `let other_count = count;` | `count`, `other_count` | None | No, the integer is copied |
    | `let first = String::from("Read");` | `count`, `other_count`, `first` | `first` owns A | Yes, A |
    | `let second = first;` | `count`, `other_count`, `second` | `second` owns A | No, ownership moves from `first` |
    | `let third = second.clone();` | `count`, `other_count`, `second`, `third` | `second` owns A, `third` owns B | Yes, B contains a copy of A's text |

    `first` is unusable after the move into `second`. The two integers remain
    usable because `u32` implements `Copy`.

    At the inner closing brace, `third` is dropped first, then `second`, in
    reverse declaration order. Their drops release B and A respectively.
    Afterward, only `count` and `other_count` remain usable. At the outer
    closing brace, `first` does not drop A again because it has been moved from.

    Expected output:

    ```text
    Read Read 3 3
    ```

    Output confirms the printed values, but it does not show which binding
    owns an allocation or when that allocation is released. Independent
    strings can print identical text. The ownership table follows from Rust's
    move, `Copy`, `Clone`, and scope rules.

---

## 3. Diagnose and repair a move

Learn this in [Topic 2: Moves](02-move-semantics.md), with [Topic 5: Mutable borrowing](05-mutable-borrowing.md) for the alternative solution.

```rust
fn decorate(mut title: String) -> String {
    title.push('!');
    title
}

fn main() {
    let title = String::from("Read");
    let updated = decorate(title);
    println!("{title}");
}
```

Save this in a temporary Rust file and compile it. Identify the move site and
the invalid later use. Repair it **without cloning**: keep the returned value
and print `Read!`. Explain who owns the string inside `decorate` and after its
return. Do not replace the function with a borrowed API for this exercise.

??? success "Solution: repair the move"
    `decorate(title)` moves the string from `main` into the function's `title`
    parameter. The later `println!("{title}")` tries to use the moved binding,
    so the compiler reports **E0382: borrow of moved value**.

    Print `updated`, which owns the returned string:

    ```rust
    fn decorate(mut title: String) -> String {
        title.push('!');
        title
    }

    fn main() {
        let title = String::from("Read");
        let updated = decorate(title);
        println!("{updated}");
    }
    ```

    Expected output:

    ```text
    Read!
    ```

    Inside `decorate`, its `title` parameter owns the string. `mut` allows the
    function to append `!`. The final expression `title` moves ownership back
    to the caller, where `updated` becomes the owner. The string is dropped
    when `updated` leaves scope at the end of `main`.

    No clone is needed because the exercise requires only the updated string,
    with no independent copy of the original.

??? success "Alternative solution: mutable borrowing"
    This version explores a borrowed API. Use the first solution to complete
    Exercise 3's requirement to practice ownership transfer.

    ```rust
    fn decorate(title: &mut String) {
        title.push('!');
    }

    fn main() {
        let mut title = String::from("Read");
        decorate(&mut title);
        println!("{title}");
    }
    ```

    Expected output:

    ```text
    Read!
    ```

    `main` keeps ownership of the string. `&mut title` lends exclusive access
    to `decorate`, whose `&mut String` parameter lets it modify the caller's
    string. The binding in `main` needs `mut` to allow this mutable borrow.
    After the call, `main` can use `title` again. No returned string or clone
    is needed, and the string is dropped at the end of `main`.

    For more practice, see [Mutable borrowing](05-mutable-borrowing.md).

---

## 4. Complete the snapshot program

Learn this in [Topic 2: Moves](02-move-semantics.md) and [Topic 3: Copy and Clone](03-copy-and-clone.md).

The requirement is different here: preserve the original title while changing
a second independently owned title. Complete the starter in
`examples/02-ownership-and-borrowing/09-hands-on/starter.rs`.

```rust
--8<-- "examples/02-ownership-and-borrowing/09-hands-on/starter.rs"
```

From the repository root:

```bash
rustc examples/02-ownership-and-borrowing/09-hands-on/starter.rs -o /tmp/rust-ownership-practice
/tmp/rust-ownership-practice
```

The starter deliberately panics at `todo!()` until completed. Success means
both assertions pass and the output is:

```text
snapshot: Read Rust, updated: Read Rust!
```

Explain why a clone is justified by this requirement but unnecessary in
Exercise 3. Then add an empty-title case: its snapshot must remain empty and
its decorated result must be `!`.

??? success "Solution: preserve an independent snapshot"
    Clone the original before moving it into `decorate`. The snapshot keeps
    its own text while the function changes the string it receives.

    ```rust
    --8<-- "examples/02-ownership-and-borrowing/09-hands-on/solution.rs"
    ```

    `decorate` owns its `title`, appends `!`, and returns ownership to
    `updated`. The assertions check that the snapshot stays unchanged and
    the updated string contains the added punctuation, including for an empty
    title. The empty-title checks produce no additional output.

    Unlike Exercise 3, this requirement needs two independently owned strings,
    so the clone is justified.

---

## 5. Iterate over a selected slice

Learn this in [Topic 5: Mutable borrowing](05-mutable-borrowing.md), [Topic 6: Collections and iterators](06-borrowing-collections-and-iterators.md), and [Topic 7: Slices](07-slices-and-api-boundaries.md).

A slice lets a function work on just the selected elements while the caller
keeps the whole collection. The range `1..3` includes indices 1 and 2.

Complete these functions without cloning or consuming the vector:

```rust
fn sum_values(values: &[i32]) -> i32 {
    todo!("Use a for loop with iter() to sum the values")
}

fn add_one_to_each(values: &mut [i32]) {
    todo!("Use iter_mut() to increase each selected value by one")
}

fn main() {
    let mut numbers = vec![10, 20, 30, 40];
    let total = sum_values(&numbers[1..3]);
    assert_eq!(total, 50);
    add_one_to_each(&mut numbers[1..3]);
    assert_eq!(numbers, vec![10, 21, 31, 40]);
    println!("middle sum: {total}");
    println!("updated: {numbers:?}");
}
```

Save your attempt in a temporary `.rs` file and run it with `rustc`, as in
Exercise 4. Then check both functions with a whole array and an empty slice:
an empty sum must be `0`, and updating an empty slice must do nothing.
Use small numbers so addition stays within `i32` limits.

Explain why `*value` is needed to update an element, why indices 0 and 3
remain unchanged, and why `numbers` remains usable after both calls.
What would happen if you tried `&numbers[1..5]` with this four-element vector?

??? success "Solution: shared and mutable iteration"
    ```rust
    --8<-- "examples/02-ownership-and-borrowing/09-hands-on/iteration-and-slicing/numbers.rs"
    ```

    `iter()` yields `&i32`; dereferencing reads the integer for the sum.
    `iter_mut()` yields `&mut i32`; `*value += 1` changes the original element.
    Both functions borrow, so `numbers` keeps ownership. Only indices 1 and 2
    are included in the mutable slice. Its borrow ends before the vector is
    read again. An out-of-bounds range such as `1..5` would panic.

    Expected output:

    ```text
    middle sum: 50
    updated: [10, 21, 31, 40]
    ```

<div class="quiz" data-answer="b" data-explanation="The exclusive end index excludes index 3, and iter_mut changes only the elements within the borrowed slice.">
<p>After updating <code>&amp;mut numbers[1..3]</code>, which elements changed?</p>
<label><input type="radio" name="review9-mutable-slice" value="a"> Every element, because the vector owns the slice</label><br>
<label><input type="radio" name="review9-mutable-slice" value="b"> Only elements at indices 1 and 2</label><br>
<label><input type="radio" name="review9-mutable-slice" value="c"> Only copies, leaving the vector unchanged</label><br>
<button class="quiz-check">Check answer</button><p class="quiz-result" aria-live="polite"></p>
</div>

---

## 6. Return a borrowed word

Learn this in [Topic 4: Shared borrowing](ownership-and-borrowing.md) and [Topic 7: Slices and text APIs](07-slices-and-api-boundaries.md).

A text slice views existing UTF-8 text. Return the first whitespace-separated
word without allocating a new `String`. Use the `split_whitespace()` iterator
from Topic 7: `next()` selects its first item, and `unwrap_or("")` supplies an
empty result when there is no word.

```rust
fn first_word(text: &str) -> &str {
    todo!("Return the first word, or an empty slice when there is none")
}

fn main() {
    let sentence = String::from("Rust makes borrowing useful");
    let word = first_word(&sentence);
    assert_eq!(word, "Rust");
    println!("first word: {word}");
    println!("still owned: {sentence}");
}
```

Run your completed program, then add assertions for `"hello"`, `""`,
whitespace-only text, and `"  café Rust"`. Expected results are `"hello"`,
`""`, `""`, and `"café"`. Explain who owns the returned word's text.

Text range indices count bytes. An arbitrary byte boundary can split a UTF-8
character and cause a panic; splitting on whitespace returns valid text slices.
Try inserting `sentence.clear()` before printing `word`, read the compiler
error, then move the mutation after the last use of `word` and run again.

??? success "Solution: borrow from the input text"
    ```rust
    --8<-- "examples/02-ownership-and-borrowing/09-hands-on/iteration-and-slicing/text.rs"
    ```

    The result for a nonempty word points into the input text; no word is
    cloned. `sentence` continues to own its text. The returned reference must
    stay valid while used, so clearing `sentence` before the last use of
    `word` is rejected. After that last use, the borrow can end and clearing
    the string is allowed.

    Expected output:

    ```text
    first word: Rust
    still owned: Rust makes borrowing useful
    ```

<div class="quiz" data-answer="c" data-explanation="word borrows sentence's text. Mutating the String while that reference is still needed could invalidate the view, so Rust rejects it.">
<p>Why is clearing <code>sentence</code> before a later use of <code>word</code> rejected?</p>
<label><input type="radio" name="review9-text-slice" value="a"> Returning a slice permanently prevents all mutation</label><br>
<label><input type="radio" name="review9-text-slice" value="b"> first_word moved ownership out of sentence</label><br>
<label><input type="radio" name="review9-text-slice" value="c"> The borrowed text must remain valid until word's last use</label><br>
<button class="quiz-check">Check answer</button><p class="quiz-result" aria-live="polite"></p>
</div>

Run the reference programs from the repository root:

```bash
rustc examples/02-ownership-and-borrowing/09-hands-on/iteration-and-slicing/numbers.rs -o /tmp/rust-review-numbers
/tmp/rust-review-numbers
rustc examples/02-ownership-and-borrowing/09-hands-on/iteration-and-slicing/text.rs -o /tmp/rust-review-text
/tmp/rust-review-text
```

---

## 7. Advanced: borrow, update, then consume titles

Learn this in [Topic 2: Moves](02-move-semantics.md),
[Topic 5: Mutable borrowing](05-mutable-borrowing.md),
[Topic 6: Collections and iterators](06-borrowing-collections-and-iterators.md),
and [Topic 7: Slice APIs](07-slices-and-api-boundaries.md).

Combine three access patterns in one pipeline: borrow to read, borrow mutably
to edit, then take ownership to select strings for a new collection.

- Implement `mark_titles` with `iter_mut()` to append `!` to each selected title.
- Implement `total_bytes` with `iter()` to sum the selected strings' byte lengths.
- Implement `keep_long_titles` with `into_iter()` to consume the input vector
  and move titles of at least `minimum_bytes` into a new vector, in input order.
- Use no `clone()` or `to_string()`. Count bytes with `len()`, not characters:
  `"café"` has five UTF-8 bytes and four characters.

Complete this starter:

```rust
fn mark_titles(titles: &mut [String]) {
    todo!("Append ! through mutable references")
}

fn total_bytes(titles: &[String]) -> usize {
    todo!("Sum byte lengths through shared references")
}

fn keep_long_titles(titles: Vec<String>, minimum_bytes: usize) -> Vec<String> {
    todo!("Move matching strings into a new Vec")
}

fn main() {
    let mut titles = vec![
        String::from("Read"),
        String::from("Code"),
        String::from("Test"),
        String::from("Go"),
    ];
    mark_titles(&mut titles[1..3]);
    assert_eq!(titles, ["Read", "Code!", "Test!", "Go"]);
    let bytes = total_bytes(&titles[1..3]);
    assert_eq!(bytes, 10);
    println!("selected bytes: {bytes}");
    println!("before move: {titles:?}");

    let kept = keep_long_titles(titles, 5);
    assert_eq!(kept, ["Code!", "Test!"]);
    println!("kept: {kept:?}");
}
```

Add assertions for empty input, no matches, a zero threshold, and a title
containing `"café"`. Predict which strings are dropped during selection and
which survive in `kept`. Then try printing `titles` after the final call,
explain the compiler error, and remove that invalid print.

??? success "Solution: combine all three iterator modes"
    ```rust
    --8<-- "examples/02-ownership-and-borrowing/09-hands-on/owned-title-pipeline/main.rs"
    ```

    The first two functions borrow slices, so the caller retains its vector.
    `keep_long_titles` takes ownership of the vector and `into_iter()` yields
    owned strings. `push(title)` moves each matching string into `kept`.
    Rejected strings are dropped at the end of their loop iteration. Kept
    strings are dropped when the returned vector is dropped.

    The original `titles` binding is unusable after the consuming call. A new
    vector may allocate storage for its elements, but no string text is cloned.

    Expected output:

    ```text
    selected bytes: 10
    before move: ["Read", "Code!", "Test!", "Go"]
    kept: ["Code!", "Test!"]
    ```

Run the reference solution from the repository root:

```bash
rustc examples/02-ownership-and-borrowing/09-hands-on/owned-title-pipeline/main.rs -o /tmp/rust-title-pipeline
/tmp/rust-title-pipeline
```

---

## Check your reasoning

These checkpoints supplement the coding tasks. Complete the tasks first.

<div class="quiz" data-answer="b" data-explanation="Copy duplicates the integer value, so both bindings remain usable.">
<p>Why can Exercise 2 print both count variables?</p>
<label><input type="radio" name="review9-copy" value="a"> Every assignment borrows</label><br>
<label><input type="radio" name="review9-copy" value="b"> u32 implements Copy</label><br>
<label><input type="radio" name="review9-copy" value="c"> Printing restores moved bindings</label><br>
<button class="quiz-check">Check answer</button><p class="quiz-result" aria-live="polite"></p>
</div>

<div class="quiz" data-answer="a" data-explanation="second and third own independent strings and are dropped at the end of the inner scope.">
<p>Which strings are dropped at the inner closing brace in Exercise 2?</p>
<label><input type="radio" name="review9-scope" value="a"> The strings owned by second and third</label><br>
<label><input type="radio" name="review9-scope" value="b"> Only third, because first still owns the original</label><br>
<label><input type="radio" name="review9-scope" value="c"> None, because all strings live until program exit</label><br>
<button class="quiz-check">Check answer</button><p class="quiz-result" aria-live="polite"></p>
</div>

<div class="quiz" data-answer="c" data-explanation="Clone is appropriate when the requirement calls for independently owned copies; it should not be an automatic response to a move error.">
<p>When is cloning justified?</p>
<label><input type="radio" name="review9-final" value="a"> Whenever the compiler reports an ownership error</label><br>
<label><input type="radio" name="review9-final" value="b"> Whenever a function returns a String</label><br>
<label><input type="radio" name="review9-final" value="c"> When the program needs independent owned copies</label><br>
<button class="quiz-check">Check answer</button><p class="quiz-result" aria-live="polite"></p>
</div>

---

## Reference and completion criteria

After your attempt, compare with `solution.rs` in the same example folder.
For Exercise 2, `first` becomes unusable when moved into `second`; cloning
creates the independently owned `third`. Both are dropped in the inner scope.
For Exercise 3, print `updated` instead of the moved `title`.

You are ready to continue when you can complete the code, explain every
ownership transfer, justify the snapshot clone, and describe the compiler error
without consulting the solution. Revisit the linked prerequisite for any term
you cannot explain. Also demonstrate that your slice functions accept arrays,
vectors, and empty slices, and explain why a borrowed word cannot outlive its
input. This set practices selected skills from Topics 1–7, not the whole module.
For the advanced exercise, explain why the first two calls preserve the caller's
vector and the third consumes it, and trace ownership of kept and rejected titles.

Next: continue the learning path with [Shared borrowing](ownership-and-borrowing.md),
or proceed to [Module 3](../03-structs-enums-and-data-modeling/index.md) if you
have already completed the remaining Module 2 foundations.
