Agent Instructions for Copilot Agents

Purpose

- For every chat, the agent must identify or confirm a task, produce a plan, create TODOs, request confirmation, execute the plan, and write progress logs.

Behavior (required)

- If no explicit task is provided: infer the most likely task and ask the user to confirm the inferred task before doing any work.
- When a task is provided or confirmed:
  1. Produce a short plan with clear steps.
  2. Convert the plan into TODOs and publish them via the `manage_todo_list` tool. The TODOs must cover all non-trivial steps.
  3. Present the plan and TODOs to the user and ask for confirmation before executing any step.
  4. After user confirmation, execute steps one-by-one. Do not start multiple unrelated steps in parallel unless the user approves.
  5. For each step, write progress entries to a dedicated task log in `.github/internal_progress_logs` following the required log format (see below).
  6. If an issue or error is encountered during a step, log the issue immediately (see Issues format) and mark the TODO appropriately.
  7. When the task completes, write a final summary entry to the task log and update the TODOs statuses via `manage_todo_list`.

Required tools and interactions

- Always use the `manage_todo_list` tool to create and update TODOs for multi-step tasks. At most one TODO may be `in-progress` at a time.
- Use `apply_patch` to make any repository file changes.
- Create or append logs within `.github/internal_progress_logs` for every task.

Log file naming convention

- File name: `task-<YYYYMMDD>_<HHMMSS>-<slug>.md`
  - Use UTC timestamps (ISO-ish): `20260205_142312`
  - Slug: short lowercase hyphenated summary (e.g., `update-readme`).
  - Example: `task-20260205_142312-update-readme.md`

Required log format (MUST follow exactly)

- All log files are plain Markdown. The agent MUST append entries; never overwrite previous entries except to correct its own previous malformed entry (and then note the correction).

- Header block (at file start):
  - Task: <short title>
  - Task-ID: <unique-id>
  - Created: <UTC ISO timestamp>
  - Status: <planned|in-progress|blocked|completed>

- Step entries (one line each, chronological). Format:
  [<UTC ISO timestamp>] STEP <n> | <short-step-title> | <status> | <details>
  - `<status>` is one of: STARTED, COMPLETED, SKIPPED, FAILED
  - `<details>` is concise: command run, files changed, or short note. If longer notes are needed, append a short paragraph below the line.

- Issue entries (one line each). Format:
  [<UTC ISO timestamp>] ISSUE | SEVERITY=<critical|major|minor|info> | <short-description> | ACTION=<planned-action>

- Artifact entries (one line each). Format:
  [<UTC ISO timestamp>] ARTIFACT | <path-or-link> | <short-description>

- Final summary entry (one line). Format:
  [<UTC ISO timestamp>] TASK COMPLETE | STATUS=<success|partial|failure> | SUMMARY=<one-line summary> | DURATION=<hh:mm:ss>

Examples

- Header:
  Task: Update README badges
  Task-ID: task-20260205_142312-update-readme.md
  Created: 2026-02-05T14:23:12Z
  Status: in-progress

- Step entry:
  [2026-02-05T14:23:15Z] STEP 1 | Create badge images | STARTED | generating with script gen_badges.sh
  [2026-02-05T14:23:42Z] STEP 1 | Create badge images | COMPLETED | produced 3 files: badges/ci.svg, badges/coverage.svg

- Issue entry:
  [2026-02-05T14:24:00Z] ISSUE | SEVERITY=minor | network timeout downloading badge font | ACTION=retry-after-30s

- Final summary:
  [2026-02-05T14:30:10Z] TASK COMPLETE | STATUS=success | SUMMARY=Badges generated and README updated | DURATION=00:07:55

Agent responsibilities regarding logs

- Create a new log file at task start and write the header.
- Append a STEP STARTED entry immediately before executing a step and append STEP COMPLETED/FAILED when finished.
- When encountering an error, append an ISSUE entry with severity and planned action; surface the issue to the user in the chat as well.
- Add ARTIFACT entries for files changed, created, or important URLs.
- Add a TASK COMPLETE entry with a succinct summary and duration.

Permissions and safety

- Never log or store secrets (API keys, passwords, private tokens). If a step would require secrets, note the requirement in the log (e.g., `requires secret: CI_TOKEN`) but do not write the secret value.

Notes for human reviewers

- This file defines required runtime behaviors for Copilot agents in this repository. If you change the logging format, update existing logs to match only with a corrective note explaining the migration.

Commit guidelines (post-task)

After completing a task and writing the task log, the agent MUST create one or more git commits that capture the final changes. The commits must be constructed according to the following constraints and rules. These rules are authoritative and intended to produce a clean, minimal commit history representing the completed work.

Absolute constraints (MUST NOT be violated)

- Do not introduce any new changes to any file beyond the provided final state.
- Do not remove or revert existing changes in the final state.
- Do not modify code, comments, formatting, naming, or structure beyond what already exists in the final state.
- Do not add refactors, cleanups, or preparatory edits.
- Do not reorder edits inside files unless the reordering already exists in the final state.
- The union of all commits must exactly reproduce the provided final code state. No commit may contain changes not already present.

Commit construction rules

- Partition the final changes into commits such that:
  1. Each commit has a single, well-defined purpose.
  2. Commits are as small as possible while remaining meaningful.
  3. Commits are internally coherent and logically reversible.
  4. Partial file changes are allowed and expected if that improves atomicity.
  5. Splitting changes within a single file across commits is allowed only when those splits represent distinct logical purposes.

Commit message rules

- For each commit:
  1. Use a concise, imperative commit subject line (e.g., "Add X", "Fix Y").
  2. In the commit body, briefly describe the intent or rationale for the grouped changes.
  3. Do not reference future commits or the final combined state.
  4. Do not fabricate exploratory reasoning, abandoned approaches, or trial-and-error behavior.

Procedure

- Stage and commit changes without making further edits. If the final state contains multiple logical changes, create multiple commits that together reproduce the final state when applied in order.
- Before committing, append a short ARTIFACT entry to the task log listing the created commit SHAs and a one-line description for each and said entry into the commit.

Example (high-level)

- Commit 1: "Add copilot instructions" — adds the instructions file with logging requirements.
- Commit 2: "Add internal log template" — adds the `internal_progress_logs/log_template.md` file containing the template and example.
