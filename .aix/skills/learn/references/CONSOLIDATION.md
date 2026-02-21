# Memory Consolidation Guide

Consolidation converts raw, incremental memory into clean, organized knowledge. Think of
it as the difference between a pile of sticky notes and a well-maintained reference
document.

## When to Consolidate

- **End of a major task** — You just finished a feature, refactor, or investigation that
  produced multiple memory entries
- **Session boundary** — The conversation is wrapping up and you recorded things during it
- **Memory is growing unwieldy** — Files are long, redundant, or contradictory
- **User requests it** — The user explicitly asks you to clean up or consolidate

## Consolidation Checklist

### 1. Survey recent changes

Run `git log --oneline -20` inside `.context/` to see what changed recently. Read the
commit messages to understand what was added or modified.

### 2. Review the current conversation

Scan the current conversation for insights you have not yet recorded. Look for:
- Decisions that were made and why
- Corrections the user made to your assumptions
- Patterns you noticed while working
- Preferences the user expressed

### 3. Audit each recently-changed file

For every memory file touched in recent commits, ask:

- **Is this still accurate?** Delete or correct anything that is no longer true.
- **Is this concise?** Rewrite verbose entries into tight bullet points.
- **Does this duplicate another file?** Merge overlapping content.
- **Is the frontmatter current?** Update `description` and `importance` if the content
  has changed.

### 4. Promote stable learnings

When an entry in `learnings/` has proven itself across multiple tasks — it is not a
one-off observation but a reliable pattern — move it to the appropriate file in `domain/`.

Indicators that a learning is ready for promotion:
- You have relied on it more than once
- The user has confirmed it
- It describes a stable aspect of the codebase, not a temporary state

### 5. Prune outdated entries

Delete entries that are no longer relevant:
- A refactored module's old architecture description
- Preferences the user has since changed
- Gotchas for bugs that have been fixed
- Temporary workarounds that have been replaced with proper solutions

### 6. Update frontmatter

After any content changes, re-read the file and update:
- `description` — Does the one-liner still accurately summarize the file?
- `importance` — Has the importance changed based on how often this knowledge is needed?

### 7. Commit atomically

Stage and commit all consolidation changes as a single commit:

```bash
cd .context && git add -A && git commit -m "consolidate: <brief summary of what changed>" && cd ..
```

## Reflection Prompts

Use these to guide your consolidation thinking:

- What did I learn during this task that I did not know before?
- What surprised me? Where were my assumptions wrong?
- Which of my existing memories turned out to be incorrect or incomplete?
- Are there patterns emerging across multiple learnings that deserve their own file?
- Is there anything the user told me that I should remember for next time?
- Did I waste time on something that better memory would have prevented?

## Record-vs-Skip Heuristics

### Record

- Decisions with non-obvious rationale
- User corrections to your behavior or assumptions
- Relationships between components that are not obvious from the code
- Conventions that are not enforced by tooling
- Things you had to figure out through trial and error

### Skip

- Facts you can re-derive by reading the code
- Standard library behavior or well-documented framework features
- Steps you took that did not lead anywhere (unless the dead end itself is instructive)
- Transient state (current branch name, today's failing test, etc.)

## Subagent Pattern for Non-Blocking Consolidation

When consolidation would interrupt the user's workflow, spawn a subagent:

1. The subagent reads the current `.context/` state and the conversation history
2. It performs the consolidation checklist independently
3. It commits its changes to `.context/`
4. The main agent continues working with the user

This mirrors the "sleep-time compute" pattern: memory management happens asynchronously
so the user never waits for it.
