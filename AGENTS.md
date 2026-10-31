# Learning Repository

This repository is a technical learning site.

## Goal

Teach Rust progressively, from fundamentals to a practical capstone.
Keep `CONTEXT.md` to the current domain model and terminology.

## Agent role

Act as a Rust course-design and documentation collaborator. Before writing, first
understand the learner profile, course goal, progression, and existing project
structure. Keep requirements, syllabus, navigation, and lessons consistent;
ask focused questions when a decision materially changes the course; and verify
documentation builds after edits.

## Code design rules

Apply SOLID principles pragmatically: give modules and classes clear
responsibilities, separate calculation logic from simulation and UI, and keep
interfaces small. Prefer composition and introduce abstractions only when they
support an actual extension or testing need.

## Documentation rules

When adding a lesson:

1. Start with “By the end, you will be able to”.
2. Explain the intuition before the necessary mathematics.
3. Add a diagram when it clarifies the concept.
   Use a bulleted topic list for module and lesson introductions; do not use
   Mermaid diagrams merely to list topics or learning goals. Reserve diagrams
   for concepts, relationships, or execution flows that benefit from a visual.
4. Add a small runnable example and a hands-on exercise.
5. Add 3–5 option-based checkpoint quizzes, especially immediately after
   physics and mathematics explanations, and link the prerequisite and next
   topic.
6. Separate major sections with `---`.
7. Keep MkDocs Material code-copy enabled for every code block.
8. Add a concise docstring to every Python method or function introduced in a
   lesson example.

Use the existing `.quiz` radio-option pattern from `docs/javascripts/quiz.js`:
each checkpoint must have plausible options, one correct answer, a unique radio
`name`, a visible `Check answer` button, and a brief explanation through
`data-explanation`. Do not use free-form review questions as the only knowledge
check. Place a quiz beside the code example or explanation it tests, so the
learner does not need to scroll between the question and its context. Do not
put all code-specific quizzes in one summary block at the end. Reserve the end
of the lesson for a broader quiz that checks generic understanding of the topic.

After the learner clicks `Check answer` with the correct option selected,
highlight that option with a soft green background and green border, then show
`✓ Correct.` with the explanation underneath. Keep explanations hidden until
a correct answer is checked; do not add separate explanation dropdowns. For an
incorrect answer, show `Try again.` without revealing the correct option or
explanation. Clear any previous success highlight when checking another answer.
Reuse the shared quiz JavaScript and CSS for this feedback.

Put lessons in `docs/modules/<module>/`, images in that lesson's `images/`
folder, and runnable code in the matching `examples/<module>/` folder.
Give each demo its own `examples/<module>/<demo>/` folder with a `README.md`
explaining prerequisites, exact commands to run it from the repository root
using the course `.venv`, expected outputs, and any limitations.
After documentation changes, run `uv run mkdocs build --strict`.

## Design-document rules

Before implementing a non-trivial physics, control, architecture, integration,
or research decision, save its plan under `design/` in the matching group:
`physics/`, `navigation/`, `integration/`, `learning/`, or `research/`.
Update [`design/README.md`](design/README.md) whenever a design note is added,
moved, or materially revised.

Each design note must include a clear title, creation or revision date, short
description, status, key decisions, links to related lessons/examples, and
Mermaid diagrams when a flow or boundary would be clearer than prose. Use the
latest Git commit date for existing notes; use the current date for new or
revised notes.

## Git rules

Only the Git user `amire2000` may create commits in this repository. Before
committing, verify that `git config user.name` is exactly `amire2000`. Do not
change the configured identity or override the commit author to satisfy this
rule; if the identity differs or this is not a Git repository, do not commit.

When asked to commit, show the proposed commit message and wait for explicit
approval before creating the commit.
