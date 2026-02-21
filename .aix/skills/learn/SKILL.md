---
name: learn
description: >
  Maintain persistent memory across sessions using a .context/ directory.
  Only activates when .context/ already exists OR the user explicitly asks to
  initialize it. When active, use it to recall prior context at session start, record
  learnings after significant tasks, or when the user asks you to remember something.
  Also handles memory consolidation and defragmentation.
---

# Learn — Continuous Learning

You maintain a `.context/` directory at the project root as your persistent memory. Inside
it, structured markdown files with YAML frontmatter organize what you know. You control
what stays loaded in context and what you retrieve on demand.

**Important:** If no `.context/` directory exists, this skill is a complete no-op. Do not
initialize, record, recall, or do anything else described here unless the user explicitly
asks you to set up context tracking (e.g. "initialize .context", "start tracking context",
"remember this").

## When to use this skill

- **No `.context/` directory exists** → Do nothing (no-op) unless the user explicitly asks
  to initialize
- **User explicitly asks to initialize context tracking** → Initialize
- **Starting a session with an existing `.context/`** → Recall
- **After completing a significant task (`.context/` exists)** → Record
- **User says "remember this" or similar** → Initialize first if needed, then Record
- **Memory feels stale, disorganized, or bloated** → Consolidate or Defragment

## 1. Initialize

Run this once per project, when no `.context/` directory exists.

1. Create the directory structure:

   ```
   .context/
   ├── system/
   ├── domain/
   └── learnings/
   ```

2. Add `.context/` to the project's `.gitignore` (if not already present).

3. Bootstrap initial memory by spawning concurrent subagents to explore the codebase.
   Assign each a distinct focus:
   - **Architecture** — entry points, module boundaries, data flow, deployment
   - **Patterns & conventions** — naming, file organization, component structure, idioms
   - **Dependencies & tooling** — key packages, build system, test framework, dev scripts

4. Merge subagent findings into initial memory files:
   - `system/overview.md` — project summary, tech stack, top-level structure
   - `system/preferences.md` — user preferences discovered so far (can be empty initially)
   - `domain/architecture.md` — system architecture and data flow
   - `domain/patterns.md` — recurring patterns and abstractions

   Follow the format in `references/FILE-FORMAT.md` for every file you create.

## 2. Record

After completing a task or when the user asks you to remember something, persist the
insight to `.context/`.

### What to record

- Decisions and their rationale
- Surprises — things that behaved differently than expected
- Corrections to your prior understanding
- User preferences and working style
- Non-obvious relationships between parts of the codebase

### What to skip

- Transient details (one-off debugging steps, temporary workarounds)
- Things already documented in code comments or READMEs
- Obvious facts about well-known libraries or frameworks

### How to record

1. Determine the right file. Check existing files in `.context/` first — append to an
   existing file if the topic matches. Create a new file only if no existing file covers
   the topic.
2. Write the entry following `references/FILE-FORMAT.md`.

## 3. Recall

At the start of a task or session, load relevant memories.

1. List the `.context/` directory tree to see what memory files exist.
2. Read the YAML frontmatter (`description` field) of each file to understand its
   contents without loading the full body.
3. **Always load** every file in `system/`.
4. **Selectively load** files from `domain/` and `learnings/` based on the current task.
   Use the `description` and `importance` fields to decide.
5. If a task touches an area you have no memory of, note this as a gap — fill it during
   the Record step after the task.

## 4. Consolidate

Periodically review and refine your memory. This converts raw, incremental notes into
clean, organized knowledge. Do this:

- After completing a major feature or multi-step task
- At the end of a session, before the conversation closes
- When you notice memory files have grown messy or contradictory
- When the user asks you to clean up or consolidate your memory

Follow the full checklist in `references/CONSOLIDATION.md`. The short version:

1. Review the files in `.context/` and the current conversation
2. Rewrite incremental notes into concise summaries
3. Promote stable learnings from `learnings/` into `domain/`
4. Delete information that is no longer accurate
5. Update all frontmatter descriptions

Consider spawning a subagent for consolidation so it does not block the main interaction.

## 5. Defragment

When memory files become unwieldy, reorganize them.

- **Split** files that exceed ~200 lines into focused subtopic files
- **Merge** small files that overlap significantly into a single file
- **Rename** files and directories whose names no longer reflect their contents
- **Update** frontmatter descriptions after every structural change
After defragmenting, verify the directory structure still makes sense by listing it and
reading the updated frontmatter.
