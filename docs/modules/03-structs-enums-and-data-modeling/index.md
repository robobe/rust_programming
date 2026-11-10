# Module 3: Structs, enums, and data modeling

## By the end, you will be able to

- Group related values in structs and choose meaningful field types.
- Define methods and associated functions in `impl` blocks.
- Represent alternatives with enums and handle them with patterns.
- Use `Option<T>` for missing values and `Result<T, E>` for operation outcomes.
- Model a task with a status enum and methods for reading and updating it.

Prerequisite: [Module 2: Ownership and borrowing](../02-ownership-and-borrowing/index.md).
All seven topics are published. Follow the topics in order, extending the same `Task` example
from its first struct through the final model.

---

## Learning path

### 1. Structs: grouping related data

- Named fields, field types, and creating a `Task` instance
- Reading fields and updating fields through a mutable binding
- Field initialization shorthand
- Reading an owned `String` field through a reference versus moving it out
- `#[derive(Debug)]` and debug printing to inspect a value; traits come in Module 4
- Short extension: struct update syntax, tuple structs, and unit-like structs

Study [Structs: grouping related data](01-structs.md) first.

### 2. Methods and associated functions

- Attaching behavior to a type with an `impl` block
- Choosing `&self`, `&mut self`, or `self` for `Task` methods using ownership rules
- Calling methods with dot syntax
- Associated functions such as `Task::new` and the conventional meaning of `new`
- Returning values and keeping methods focused on one responsibility

Continue with [Methods and associated functions](02-methods.md).

### 3. Enums: representing alternatives

- Defining variants and constructing enum values
- Unit-like, tuple-like, and struct-like variants
- Storing different data in different variants
- Choosing a struct for related fields and an enum for alternatives
- Adding `TaskStatus` in place of ambiguous status strings or conflicting Boolean flags

Continue with [Enums: representing alternatives](03-enum.md).

### 4. Pattern matching and destructuring

- Exhaustive `match` expressions and returning a value from each arm
- Destructuring structs and enum variants to access their fields
- Binding values, ignoring fields with `_` and `..`, and using match guards
- Comparing a match on `&TaskStatus` with one that moves owned variant data
- Choosing `if let` for one interesting pattern and handling other cases when needed

Continue with [Pattern matching and destructuring](04-pattern-matching.md).

### 5. Option: representing missing values

- `Some(T)` and `None` as explicit alternatives to a missing value
- Reading `Option<T>` notation without needing to define a generic type yet
- Handling both cases with `match` or using `if let` for the present case
- Looking up a task by position with slice or vector `get`; a position is not a task identifier
- Understanding `Option<&Task>` as a lookup that borrows a task when present
- Avoiding `unwrap()` when absence is an expected outcome

Continue with [Option: representing missing values](05-option.md).

### 6. Result: representing operation outcomes

- `Ok(T)` and `Err(E)` as success and failure variants
- Distinguishing an absent value from an operation that failed
- Handling both outcomes explicitly with `match`
- Returning `Result<(), TaskError>` from a completion method that rejects an already completed task; `()` means no success value
- Leaving error propagation with `?` and broader error handling for Module 5

Continue with [Result: representing operation outcomes](06-result.md).

### 7. Putting the task model together

- Combining the `Task` struct, identifier, owned title, and `TaskStatus` developed above
- Using its existing methods to inspect and change status
- Keeping status-specific data in enum variants when the model needs it
- Storing tasks in a `Vec<Task>` and finding one by identifier with a simple loop that returns `Option<&Task>`
- Reporting an already completed task with `Result<(), TaskError>`
- Testing construction, status changes, missing tasks, and rejected operations

Continue with [Putting the task model together](07-task-model.md).

---

## Completion goal and next module

Model a task with a struct, status enum, and methods. Demonstrate a successful
status change, a missing task through `Option`, and a rejected operation through
`Result`, then explain why each outcome uses that type.

Next: [Module 4: Traits and generics](../04-traits-and-generics/index.md)
(planned). Encapsulation and validated API design are developed further in
Module 6; borrowed structs and advanced ownership in enums return in Module 9.
