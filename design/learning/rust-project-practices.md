# Rust project structure and best practices

Date: 2026-11-10

Status: Accepted plan; lesson planned.

Add a practical project-organization topic to Module 5. Keep deeper API design,
architecture, and design patterns in Module 6.

## Key decisions

- Teach conventional Cargo layout using the official Cargo documentation.
- Cover library/binary separation, modules, visibility, test placement, developer
  checks, dependencies, repository files, and a minimal CI workflow.
- Use Google's Rust Style Guide as supplementary guidance. Explain which
  recommendations apply generally and which belong to Chromium or Fuchsia;
  do not substitute Chromium's flat layout for Cargo conventions.
- Hands-on work reorganizes the task example into a library and thin CLI binary,
  with unit, integration, and documentation tests. Include collapsed solutions.
- Completion requires tests plus formatting, Clippy, and compilation checks.

## Related material and references

- [Module 5 syllabus](../../docs/syllabus.md#5-error-handling-modules-and-testing-planned)
- [Practice ledger](data-structures-and-patterns-practice.md)
- [Cargo package layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
- [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html)
