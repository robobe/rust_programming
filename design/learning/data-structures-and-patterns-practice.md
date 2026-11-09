# Data structures and design patterns: hands-on practice

Date: 2026-11-09

Status: Accepted plan; exercises below are planned.

Build practical computer-science skills throughout the Rust course. Use standard
collections first, then implement selected structures after their ownership and
type-system prerequisites. Teach patterns through concrete requirements and
tested refactoring rather than adding abstractions without a need.

## Key decisions

- Preserve module numbering. Attach practice to the relevant existing or future
  topic, using a focused practice section where a dedicated topic is needed.
- Add practice to existing lessons as pending follow-up work; implement assigned
  practice alongside future lessons once its prerequisites are taught.
- Introduce time and space complexity alongside the operations being practiced.
- Each exercise needs a clear function or method interface, expected behavior,
  edge cases, runnable code, tests or assertions, and a collapsed solution.
- A row becomes implemented only when its lesson, example, solution, and checks
  exist. Record their paths in that row. Do not mark existing related material
  as completing an exercise without checking these requirements.

## Practice ledger

| Target module / topic | Prerequisites | Exercise and learning goal | Required checks | Status |
| --- | --- | --- | --- | --- |
| 1: arrays, functions, loops | Arrays, functions, conditions, loops | Linear search returning a Boolean; explain linear search cost | Empty input, present and absent values, duplicates | Planned |
| 2: mutable slices | Shared and mutable borrowing, slices | Reverse a slice in place without cloning; explain linear time and constant auxiliary space | Empty, singleton, odd and even lengths | Planned |
| 2: collections | Vec methods, loops, borrowing | Remove duplicates while preserving first-occurrence order; explain the cost of repeated searches | Empty, all equal, all distinct, mixed duplicates | Planned |
| 3: structs, Option | Vec, impl, Option | Wrap a Vec-backed integer stack with push and pop; handle empty removal | LIFO order, empty pop, repeated push/pop | Planned |
| 4: generics | Module 3 stack, generic types and impl blocks | Generalize the stack to Stack<T> without unnecessary Clone bounds | Integer and owned String values, empty stack | Planned |
| 5: testing | Stack exercise, unit and integration tests, module visibility | Organize and test the stack's public behavior across module boundaries | Empty operations and interleaved operations through its public API | Planned |
| 6: patterns | Structs, enums, traits, errors, tests | Builder, Strategy, Adapter, State, and Newtype exercises driven by concrete requirements; compare enum and trait approaches | Valid construction, invalid inputs, strategy substitution, adapter behavior, rejected transitions | Planned |
| 6: collection practice | Option, generics, tests | Queue with VecDeque, frequency table with HashMap, uniqueness with HashSet; choose a standard collection deliberately | FIFO and empty removal, repeated keys, duplicate values; avoid relying on hash iteration order | Planned |
| 8: iterators | Collection practice, closures, iterator adapters | Rework a frequency/filtering exercise as an iterator pipeline and explain ownership | Equivalent loop and pipeline results, empty input, no matches | Planned |
| 9: linked structures | Box, Option, ownership, introductory recursion | Implement a small singly linked stack and binary search tree; explain ownership and traversal costs | Empty structures, stack LIFO, tree insert/find, duplicate policy stated and tested | Planned |
| 9: graph practice | Vec, VecDeque, HashSet, loops, preceding structure practice | Adjacency-list graph with BFS and DFS; explain visited tracking and traversal cost | Cycles, disconnected vertices, isolated start, deterministic neighbors; define invalid-start behavior | Planned |

The Module 6 collection practice and Module 9 structure/graph practice form
focused data-structure sections within those modules, not new numbered modules.
Each pattern exercise should state the requirement that justifies the pattern
and compare it with the simpler initial solution.

## Related material

- [Syllabus](../../docs/syllabus.md)
- [Module 1](../../docs/modules/01-fundamentals/index.md)
- [Module 2](../../docs/modules/02-ownership-and-borrowing/index.md)
- [Module 3](../../docs/modules/03-structs-enums-and-data-modeling/index.md)
- [Authoring instructions](../../AGENTS.md)
