# Iteration and slicing practice

Date: 2026-11-07

Status: Accepted

Extend the beginner ownership review with practice using the borrowed APIs
already introduced in Topics 2.6–2.7.

## Key decisions

- Add two exercises after the snapshot exercise: read and mutate a numeric
  sub-slice, then return a borrowed first word from text.
- Keep solutions collapsed and include the runnable source through snippets.
- Use assertion-based checks for unchanged outer elements, arrays, empty
  slices, empty text, whitespace, and Unicode text.
- Use whitespace splitting instead of arbitrary byte indexing for text.
- Assess selected skills from Topics 1–7, without claiming lifetime coverage.
- Add an advanced exercise that mutates a slice of owned strings, reads their
  byte lengths, then consumes the vector to select owned strings without clones.
- Link every exercise directly to the topics that teach its required concepts.

## Related material

- [Review lesson](../../docs/modules/02-ownership-and-borrowing/09-hands-on.md)
- [Iteration prerequisite](../../docs/modules/02-ownership-and-borrowing/06-borrowing-collections-and-iterators.md)
- [Slices prerequisite](../../docs/modules/02-ownership-and-borrowing/07-slices-and-api-boundaries.md)
- [Runnable solutions](../../examples/02-ownership-and-borrowing/09-hands-on/iteration-and-slicing/README.md)
- [Advanced solution](../../examples/02-ownership-and-borrowing/09-hands-on/owned-title-pipeline/README.md)
