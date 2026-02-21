# AGENTS.md

## general

* Be critical and thorough. Prefer truth and direct feedback over politeness
   * After every response to me, end it with an emoji
   * Look around and use existing patterns and code when possible. Look for:
      * Similar components and use their patterns
      * Library code you can reuse
      * Existing dependencies from package.json or Cargo.toml that you should use
   * If you see a pattern that is not used, consider adding it, but carefully and
     judiciously
   * Always consider the developer experience:
      * Am I placing a burden on the developer with this change?
      * Is it as easy to use / execute / import / configure as possible?
   * When making _any_ changes:
      * Consider the impact on other parts of the codebase
         * What tests, documentation, etc. needs to be updated?
         * Search for other files that should be changed after what you just did
      * How has the context changed now that I've made this change?
         * Should I refactor the code to introduce an abstraction to make it more
           maintainable?
         * Should I delete anything that's now unused?
   * Check your work after you finish a task:
      * Did I address everything I was asked to?
      * Run `npm run standards` (or `tsc` / `eslint` / `commitlint` / `markdownlint` as
        appropriate)
      * Test significant changes by:
         * Running the tests
         * Running the app and manually testing the changes (Tauri MCP or Playwright MCP)

## Front-End Development

   * Pay attention to the current version of the component, and use a similar pattern as
     set by existing elements
   * Consider accessibility / a11y
   * Create reusable components rather than ad-hoc solutions

## css-scss

* If using a component library, prefer using the component's existing props etc. over
     custom styling
   * If none is available, prefer pre-existing utility classes over custom styling
   * Avoid ad-hoc CSS unless necessary
   * Use BEM or other similar naming conventions for custom CSS
   * Use CSS variables for theme-able values
   * Consider adding a custom utility class to the global CSS/SCSS if it seems
     necessary/used in multiple places

## do-not

* ABSOLUTELY DO NOT `git push` without express permission * ABSOLUTELY DO NOT create
     ad-hoc test scripts. If you absolutely must, clean up those files when you're done
   * ABSOLUTELY DO NOT ignore "pre-existing" TypeScript or linting errors. If you see
     them, fix them before proceeding
   * ABSOLUTELY DO NOT ignore "pre-existing" tests that fail. If you see them, fix them
     before proceeding
   * ABSOLUTELY DO NOT ignore "pre-existing" documentation that is out of date. If you see
     it, fix it before proceeding
   * ABSOLUTELY DO NOT use `@deprecated` on anything unless you are explicitly asked to.
     Always fully refactor and delete old code as-needed instead of deprecating it
   * ABSOLUTELY DO NOT implement functionality that already exists in a library or
     package, especially if that package is already installed in the project * Examples:
     parsing, validation, formatting
   * ABSOLUTELY DO NOT disable linting rules (ESLint, oxlint, clippy, etc.) in the config
     to get around linting errors. Fix the underlying issues
   * ABSOLUTELY DO NOT instruct me to do things like "run the dev server and test it out,"
     "run the tests," "install this module", or anything else that you can do yourself as
     part of the task

## git

* Follow these rules when running git commands and making commits:
      * https://raw.githubusercontent.com/silvermine/silvermine-info/refs/heads/master/commit-history.md
      * https://raw.githubusercontent.com/silvermine/standardization/refs/heads/master/commitlint.js

## mcps

* Always use context7 when I need code generation, setup or configuration steps, or
     library/API documentation. This means you should automatically use the Context7 MCP
     tools to resolve library ID and get library docs without me having to explicitly ask.
   * If you make UI changes, use MCP tools to test them in a real environment unless
     project-specific rules say not to
      * Use the Tauri MCP when working within a Tauri app
      * Use Playwright for other projects

## npm

* Always use `npm` instead of `pnpm` or `yarn`
   * Always use the --save-exact flag when installing a dependency
   * Use the `-y` flag with `npx` when running a command

## testing

* When writing tests, prefer practical e2e tests over unit tests, but add unit tests
     for critical functionality or complex logic
   * If you write tests, always run them

## typescript-javascript

Follow these standards:

   * https://github.com/silvermine/silvermine-info/raw/refs/heads/master/coding-standards/typescript.md
   * https://raw.githubusercontent.com/silvermine/silvermine-info/refs/heads/master/coding-standards.md

## General Code Style & Formatting

   * Use English for all code and documentation
   * Use JSDoc to document public classes and methods
   * ALWAYS wrap comments to utilize the full line length
   * Combine adjacent single-line const/let/var declarations into one declaration
   * Use early returns to reduce indentation

## Naming Conventions

   * Use PascalCase for classes
   * Use camelCase for variables, functions, and methods
   * Use kebab-case for file and directory names
   * Use UPPERCASE for environment variables
   * Avoid magic numbers and define constants
   * When it has an acronym or initialism, use all lowercase or all caps, never mixed-case:
      * `url` or `URL`, _never_ `Url`
      * `id` or `ID`, _never_ `Id`. Prefer `ID` over `id` when writing docs/sentences
        unless documenting a third-party entity or when specifically referring to a code
        object with that exact casing (parameter, variable, etc.)

## Functions & Logic

   * Avoid deeply nested blocks by:
      * Using early returns
      * Extracting logic into utility functions
   * Use higher-order functions (map, filter, reduce) to simplify logic
   * Use arrow functions for simple cases (<3 instructions), named functions otherwise
   * Use default parameter values instead of null/undefined checks
   * Use RO-RO (Receive Object, Return Object) for passing and returning multiple
     parameters

## Data Handling

   * Avoid excessive use of primitive types; encapsulate data in composite types
   * Prefer immutability for data:
      * Use readonly for immutable properties
      * Use `as const` for literals that never change

## Style

BAD: DO NOT DO THIS
```typescript
const a = 1;

const b = 2;

const c = 3;
```

GOOD:
```typescript
const a = 1,
      b = 2,
      c = 3;
```

## ABSOLUTELY DO NOT:

   * DO NOT Use `any` in TS code

## vue

* Assume Vue version 3.5+ unless you see otherwise in package.json
   * Use Vue 3 composition API, SFCs
   * Prefer composition using slots over props when it makes sense
   * Be aware of the different kinds of components: Page-level, layout, UI components that
     contain no business logic, and business-logic-level components
   * For styling:
      * Use SCSS
      * Use scoped styles
      * Look at ./css-scss.md

## writing

## Content

   * **Be specific, not generic.** Replace vague claims of importance with concrete facts.
     "Inventor of the first train-coupling device" instead of "a revolutionary titan of
     industry."
   * **Skip the significance speeches.** Do not add sentences about how something "marks a
     pivotal moment," "represents a broader trend," "highlights the enduring legacy," or
     "underscores the importance." Let facts speak for themselves.
   * **Avoid superficial analysis.** Do not attach "-ing" phrases that editorialize:
     "creating a lively community," "showcasing the brand's dedication," "demonstrating
     ongoing relevance."
   * **No promotional language.** Cut puffery like "nestled within," "vibrant," "rich
     tapestry," "seamlessly connecting," "gateway to," "breathtaking." Write neutrally.
   * **Attribute specifically or not at all.** Do not use weasel phrases like "has been
     described as," "is considered," "researchers note," "scholars argue" without naming
     who. If you cannot name a source, remove the claim.
   * **Do not exaggerate consensus.** Do not present one or two sources as "several
     publications" or "multiple scholars." Do not imply lists are non-exhaustive when
     sources give no indication other examples exist.
   * **Skip "Challenges and Future Prospects" formulas.** Do not write "Despite its
     [positive words], [subject] faces challenges..." followed by vague optimism about
     future initiatives.
   * **No hedging preambles.** Do not acknowledge something is "relatively minor" before
     explaining its importance anyway.

## Word Choice

Avoid overused AI vocabulary:

   * **High-frequency offenders:** delve, tapestry, landscape, multifaceted, intricate,
     nuanced, pivotal, comprehensive, innovative, cutting-edge, groundbreaking,
     transformative, paradigm, foster, leverage, spearhead, underscore, highlight,
     crucial, vital, robust, seamless, holistic, synergy, realm, beacon, testament,
     embark, unveil, unravel, commendable, meticulous, intrinsic, Moreover,
     Furthermore, Notably, Importantly, Indeed, Thus, Hence, Therefore,
     Consequently
   * One or two may be coincidental; clusters are evidence of overuse

## Grammar & Structure

   * **Use simple verbs.** Prefer "is" and "are" over "serves as," "stands as," "marks,"
     "represents," "constitutes," "features," "offers." Write "She is chairman" not
     "She serves as chairman."
   * **Avoid negative parallelisms.** Do not write "not only ... but also," "it's not just
     about ... it's ...," "not ... but rather." These constructions try to appear balanced
     but often add nothing.
   * **Break the rule of three.** Do not default to "adjective, adjective, and adjective"
     or "phrase, phrase, and phrase" patterns. Vary list lengths.
   * **Avoid elegant variation.** If you name something, use that name again. Do not cycle
     through synonyms like "the protagonist," "the key player," "the eponymous character"
     to avoid repetition.
   * **No false ranges.** "From X to Y" requires a real scale. "From the Big Bang to the
     cosmic web" or "from problem-solving to artistic expression" are meaningless ranges
     that sound impressive but say nothing.

## Formatting & Style

   * **Use sentence case for headings.** Do not capitalize every word in section titles.
   * **Avoid excessive boldface.** Do not bold every key term or create "key takeaways"
     lists with bolded headers.
   * **No emojis** unless explicitly requested

## Communication

   * **No meta-commentary.** Do not include phrases like "In this section, we will
     discuss..." or "Below is a detailed overview based on available information."
   * **No disclaimers about knowledge gaps.** Do not write "While specific details are not
     extensively documented..." or "As of my last knowledge update..."
   * **No placeholder text.** Do not leave brackets like [describe the specific section]
     or dates like 2025-XX-XX.
   * **No collaborative language in output.** Do not write "Would you like me to..." or
     "Here's a template you can customize" in content meant for publication.

Source: https://en.wikipedia.org/wiki/Wikipedia:Signs_of_AI_writing

<!-- BEGIN BEADS INTEGRATION v:1 profile:minimal hash:ca08a54f -->
## Beads Issue Tracker

This project uses **bd (beads)** for issue tracking. Run `bd prime` to see full workflow context and commands.

### Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work
bd close <id>         # Complete work
```

### Rules

- Use `bd` for ALL task tracking — do NOT use TodoWrite, TaskCreate, or markdown TODO lists
- Run `bd prime` for detailed command reference and session close protocol
- Use `bd remember` for persistent knowledge — do NOT use MEMORY.md files

## Session Completion

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd dolt push
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds
<!-- END BEADS INTEGRATION -->
