# Complete Module 3's task-model progression

Date: 2026-11-08

Status: Accepted

Publish the remaining planned lessons: pattern matching, Option, Result, and
the integrated task model. Preserve the existing three lesson URLs.

## Key decisions

- Use the existing Task fields and Pending/Completed/Cancelled status names.
  Topic 4 evolves Cancelled to carry a reason, demonstrating borrowed and owned
  destructuring. Later demos retain that variant without introducing lifetimes.
- Teach position-based get before identifier-based lookup. A lookup returns
  Option<&Task>; completion returns Result<(), TaskError>.
- Reject repeated completion and completion of cancelled tasks without changing
  their status. Leave error propagation, persistence, and public API validation
  for the later modules already assigned those concerns.
- Each lesson includes three contextual quizzes, a runnable standalone demo,
  exercises, and links to its prerequisite and next topic.
- Give the integrated demo unit tests covering construction, lookup, successful
  transitions, and rejected transitions. Other demos use assertions.

## Related material

- [Module index](../../docs/modules/03-structs-enums-and-data-modeling/index.md)
- [Pattern matching](../../docs/modules/03-structs-enums-and-data-modeling/04-pattern-matching.md)
- [Option](../../docs/modules/03-structs-enums-and-data-modeling/05-option.md)
- [Result](../../docs/modules/03-structs-enums-and-data-modeling/06-result.md)
- [Integrated model](../../docs/modules/03-structs-enums-and-data-modeling/07-task-model.md)
- [Examples](../../examples/03-structs-enums-and-data-modeling/07-task-model/README.md)
