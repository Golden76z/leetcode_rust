# README — LeetCode progression display

**Date:** 2026-08-06
**Goal:** Turn the 2-line README into a progression dashboard for the LeetCode account `Golden76z`, showing live account activity and the exercises solved in this repo.

## Structure

1. **Header** — title, one-line intro, note on repo layout (one crate per problem, `problem_name_number`) and how to run a solution (`cargo run` / `cargo test` inside a problem directory).
2. **Live stats cards** — leetcard.jacoblin.cool SVGs for `Golden76z`, linked to the LeetCode profile:
   - Main stats card with the `heatmap` extension (solved counts by difficulty + submission activity calendar).
   - Theme-aware via GitHub's `<picture>` + `prefers-color-scheme` (dark and light variants).
   - Auto-updating, zero maintenance; only dependency is the leetcard service.
3. **Solutions table** — single flat table, **most recently solved first**, capped at the **30 most recent** rows. Columns: `#`, Problem (linked to leetcode.com), Difficulty (🟢/🟡/🔴), Technique, Runtime / Memory (from commit messages; `—` when not recorded).
4. **Collapsible archive** — a `<details><summary>` block below the table. Rows beyond the 30 most recent move there (renders as a clickable expander on GitHub). Currently 29 solutions exist, so the archive starts empty with an HTML comment documenting the rule.

## Maintenance

Static markdown, updated by hand (or by Claude in-session): each new solved problem adds one row at the top of the table; when the table exceeds 30 rows, the oldest visible row moves into the archive block.

## Out of scope

- GitHub Action / script automation (may be added later if hand-editing becomes tedious).
- Grouping by technique; per-problem write-ups.
