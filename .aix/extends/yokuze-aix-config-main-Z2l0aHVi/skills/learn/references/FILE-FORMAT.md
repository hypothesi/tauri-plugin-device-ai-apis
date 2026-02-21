# Memory File Format

Every file in `.context/` follows this format.

## YAML Frontmatter

```yaml
---
description: One-line summary of what this file contains
importance: high | medium | low
---
```

### Fields

| Field | Required | Description |
|-------|----------|-------------|
| `description` | Yes | One-line summary. Used during Recall to decide whether to load the file. Write it so you can scan 10+ descriptions quickly and pick the relevant ones. |
| `importance` | Yes | `high` = core knowledge you need for most tasks. `medium` = useful for specific areas. `low` = nice to have, first to drop when defragmenting. |

## Body Content

Write the body in markdown. Follow these guidelines:

- **Be concise.** Use bullet points over prose. Every line should earn its place.
- **Be factual.** State what is, not what might be. If something is uncertain, say so
  explicitly.
- **Be specific.** Name files, functions, modules, and versions. Vague statements like
  "the app uses a modern architecture" are useless.
- **No filler.** No introductory paragraphs, no restating the description, no "as noted
  above" transitions.
- **Use headings** to organize content within a file. This makes it scannable and
  splittable during defragmentation.

## Naming Conventions

- **Files**: `kebab-case.md` — descriptive of the content, not the date or sequence
  (e.g., `error-handling.md` not `2024-01-15-notes.md`)
- **Directories**: `kebab-case` — broad topic categories, not deeply nested (prefer
  2 levels maximum: `.context/domain/api-layer.md`)

## When to Create a New File vs. Append

**Append** when:
- The new information belongs to the same topic as an existing file
- Adding it keeps the file under ~200 lines

**Create a new file** when:
- No existing file covers the topic
- The existing file is already large and the new content is a distinct subtopic
- The new information represents a different category (e.g., a learning vs. domain
  knowledge)

## Examples

### Good Memory Entry

```yaml
---
description: Vue component patterns — composition API conventions, prop validation, emit typing
importance: medium
---

# Vue Component Patterns

- All components use `<script setup lang="ts">` with Composition API
- Props defined with `defineProps<{...}>()` — always typed, never runtime validation
- Emits defined with `defineEmits<{...}>()` — always typed
- Components organized by feature in `src/components/<feature>/`, not by type
- Shared composables live in `src/composables/` and are prefixed with `use`
- Component files use PascalCase: `UserProfile.vue`, not `user-profile.vue`
```

### Bad Memory Entry

```yaml
---
description: Some notes about the project
importance: medium
---

# Notes

The project is a modern web application built with Vue. It uses TypeScript and has
various components. The architecture seems well-organized and follows best practices.
I noticed some interesting patterns while exploring the codebase that might be useful
to remember for future reference.
```

What's wrong: vague description, filler prose, no specific details, no actionable
information.
