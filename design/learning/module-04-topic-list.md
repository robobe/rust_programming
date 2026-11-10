# Module 4: traits and generics progression

Date: 2026-11-09

Status: Topic 1 implemented; Topics 2–10 planned.

Define a beginner progression from generic data to shared behavior, dispatch,
and a tested generic stack, without changing module numbering.

## Key decisions

- Teach generic functions and types before trait definitions and bounds.
- Explain common derives, associated types, and implementation rules before
  comparing static and dynamic dispatch. Keep advanced trait features for later.
- End with the assigned Stack<T> practice and a trait implemented by two types.
- Topic 1 uses owned identity and borrowed first-item functions to introduce
  type parameters, inference, and explicit arguments without premature bounds.
  Demonstrate an invalid generic addition to motivate Topic 4's trait bounds.
- Keep the generic-stack ledger entry planned until its prerequisite and Topic 9
  are implemented; introductory generic-function exercises do not complete it.

## Related material

- [Module 4 index](../../docs/modules/04-traits-and-generics/index.md)
- [Generic functions](../../docs/modules/04-traits-and-generics/01-generic-functions.md)
- [Runnable demo](../../examples/04-traits-and-generics/01-generic-functions/README.md)
- [Practice ledger](data-structures-and-patterns-practice.md)
- [Syllabus](../../docs/syllabus.md)
