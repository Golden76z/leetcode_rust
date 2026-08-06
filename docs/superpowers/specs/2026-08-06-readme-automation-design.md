# README automation — auto-update solutions table

**Date:** 2026-08-06
**Goal:** Automate the README solutions table via a manually-triggered GitHub Action, so new solved problems are added without hand-editing. Supersedes the "Maintenance" section of `2026-08-06-readme-progression-design.md` (which was hand-editing; automation was out of scope there).

## Overview

- **Trigger:** `workflow_dispatch` only — the user runs it from the GitHub Actions tab or `gh workflow run update-readme.yml`. No push trigger.
- **Publication:** the workflow commits the regenerated README directly to `main` (bot commit, French conventional-commit message `docs(readme): …`). No commit when there is no diff.
- **Source of truth:** the README itself. The script only *adds* missing rows and moves overflow rows to the archive; it never rewrites existing rows, so manual tweaks survive.
- **Metadata source:** commit messages in the repo's existing format, plus the public LeetCode API for title/link/difficulty.

## Files

- `scripts/update_readme.py` — all logic, Python 3 stdlib only, structured as pure functions. CLI: `--dry-run` prints the resulting README diff without writing.
- `scripts/test_update_readme.py` — unit tests (`python3 -m unittest`), written TDD.
- `.github/workflows/update-readme.yml` — checkout (full history, `fetch-depth: 0`), run script, commit & push if `git diff` is non-empty.

## Script logic

1. **Discover problems:** list top-level directories matching `*_<number>` (e.g. `daily_temperatures_739`). Extract problem numbers.
2. **Parse README:** collect numbers already present in the main table and in the `<details>` archive table. Missing numbers = new problems.
3. **Per new problem, parse git log:** most recent commit whose subject starts with `<number> - `. Format: `<number> - <name> - <technique> (<runtime> - <memory>)`.
   - Technique = text after the second ` - ` up to the trailing parenthesized group.
   - Runtime/Memory = trailing `(Xms - Ymb)` group, rendered as `Xms / Ymb`.
   - Commit not found or segment unparsable → `—` for that field; the run never fails on parsing.
   - Order of insertion: commit date, most recent first.
4. **LeetCode API:** one GET to `https://leetcode.com/api/problems/all/`, mapping frontend question id → official title, slug (for the `https://leetcode.com/problems/<slug>/` link), difficulty (🟢 Easy / 🟡 Medium / 🔴 Hard).
   - API unreachable or id missing → title derived from the directory name (underscores → spaces, title-case), link omitted (plain text), difficulty `—`. Run still succeeds.
5. **Rebuild sections:**
   - Insert new rows at the top of the main table.
   - Enforce the 30-row cap: overflow rows (oldest) move to the archive table inside `<details>`. On first overflow, replace the "Nothing here yet" placeholder with a table using the same columns.
   - Update the `## Solutions — N solved` heading (N = main table + archive rows).
   - Everything else in the README (header, leetcard block, HTML comment) is preserved byte-for-byte.

## Workflow

```yaml
on: workflow_dispatch
permissions: contents: write
```

Steps: checkout with `fetch-depth: 0` → run `python3 scripts/update_readme.py` → if README changed, commit as `github-actions[bot]` with message `docs(readme): mise à jour du tableau des solutions` and push to `main`. No anti-loop guard needed (no push trigger).

## Error handling

- Parsing failures degrade to `—`, never abort.
- LeetCode API failures degrade as described above, never abort.
- No new problem directories → script exits 0 with no diff → workflow ends without committing.

## Testing

- Unit tests for: commit-subject parsing, directory-number extraction, README table parsing, row rendering, 30-row overflow into archive, count update.
- End-to-end check: `--dry-run` run locally against the real repo; the untracked-at-design-time `daily_temperatures_739` problem (commit `739 - Daily temperatures - Stack (0ms - 4.38mb)`) serves as the first real case.

## Out of scope

- Push-triggered runs (may be added later by adding a `push` trigger plus an anti-loop guard).
- Rewriting or normalizing existing rows.
- Per-problem write-ups, grouping by technique.
