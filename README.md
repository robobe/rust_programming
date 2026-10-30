# Rust Programming Course

A beginner-friendly MkDocs Material site for learning Rust through small
examples and a command-line capstone.

## Use

1. Copy this directory and rename it for the new topic.
2. Replace bracketed placeholders in `README.md`, `CONTEXT.md`, `AGENTS.md`,
   and `mkdocs.yml`.
3. Run the site:

   ```bash
   uv sync
   uv run mkdocs serve
   ```

4. Validate before publishing:

   ```bash
   uv run mkdocs build --strict
   ```

## Layout

- `docs/` — MkDocs pages
- `docs/modules/` — lessons grouped by module
- `examples/` — runnable examples matching modules
- `design/` — decisions and plans that need to outlive implementation
- `.vscode/tasks.json` — build and preview tasks
- `AGENTS.md` and `CONTEXT.md` — Codex project guidance
