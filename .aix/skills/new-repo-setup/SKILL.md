---
name: new-repo-setup
description: Standardize a new or freshly-scaffolded project repository. Use when the user has created a new project folder (possibly via a generator like `npm create`) and wants to apply consistent tooling, linting, formatting, and AI configuration. Handles both personal projects and Silvermine team projects.
---

# New Repo Setup

Standardize a newly created project repository with consistent tooling and configuration.

## Step 1 — Determine repo type

Before doing anything else, ask the user:

> Is this a **personal** project or a **Silvermine** (non-personal/team) project?

Branch based on their answer.

---

## Personal Project Setup

### 1. AIX configuration

Set up AI tooling configuration so the repo inherits shared rules, skills, and prompts.

1. Run `npx @a1st/aix install github:yokuze/aix-config#main`
2. Verify that `ai.json` was created and contains an `extends` referencing
   `github:yokuze/aix-config#main`
3. If an `ai.json` already exists, ensure the `extends` field is present rather than
   overwriting other configuration

### 2. Package.json — `dev` script

Ensure `package.json` has a `dev` script so `npm run dev` starts the development server.

- Inspect the project to determine the correct dev command. For example:
  - **Vite / React / Vue / Svelte**: `vite` or `vite dev`
  - **Next.js**: `next dev`
  - **Nuxt**: `nuxt dev`
  - **Astro**: `astro dev`
  - **Tauri**: `cargo tauri dev` or the appropriate tauri CLI command
  - **Plain TypeScript**: `tsc --watch` or `tsx watch src/index.ts`
- If unsure, ask the user what the dev command should be
- If a `dev` script already exists and looks correct, leave it alone

### 3. Linting and formatting — oxlint + oxfmt

Install and configure oxlint with oxfmt for linting and formatting.

#### Install dependencies

```bash
npm install --save-dev --save-exact oxlint @stylistic/eslint-plugin
```

#### Create `oxlint.json` in the project root

Use this base configuration and adapt the `overrides` section based on which frameworks
the project uses:

```json
{
   "$schema": "./node_modules/oxlint/configuration_schema.json",
   "jsPlugins": ["@stylistic/eslint-plugin"],
   "categories": {
      "correctness": "error",
      "perf": "error",
      "suspicious": "error"
   },
   "rules": {
      "curly": "error",
      "@stylistic/brace-style": ["error", "1tbs", { "allowSingleLine": false }],
      "@stylistic/indent": ["error", 3, { "VariableDeclarator": "first" }],
      "@stylistic/padding-line-between-statements": [
         "error",
         { "blankLine": "always", "prev": ["const", "let"], "next": "*" },
         { "blankLine": "any", "prev": ["const", "let"], "next": ["const", "let"] }
      ],
      "@stylistic/quotes": ["error", "single", { "avoidEscape": true }],
      "@stylistic/semi": ["error", "always"],
      "no-unused-vars": [
         "error",
         { "argsIgnorePattern": "^_", "varsIgnorePattern": "^_", "caughtErrorsIgnorePattern": "^_" }
      ],
      "max-depth": ["warn", { "max": 4 }],
      "max-nested-callbacks": ["warn", { "max": 3 }],
      "max-lines-per-function": [
         "warn",
         { "max": 300, "skipBlankLines": true, "skipComments": true }
      ],
      "max-params": ["warn", { "max": 4 }]
   },
   "overrides": [],
   "ignorePatterns": [
      "**/dist/**",
      "**/node_modules/**",
      "**/*.js",
      "**/*.cjs",
      "**/*.mjs",
      "**/.aix/**"
   ]
}
```

#### Adapt overrides for the project

Inspect the project to determine which frameworks are in use, then add relevant overrides.
Common overrides:

- **Test files** (`**/*.test.ts`, `**/__tests__/**/*.ts`): Turn off `max-lines-per-function`
- **Astro files** (`**/*.astro`): Turn off `@stylistic/indent`, `@stylistic/semi`,
  `@stylistic/brace-style`, `@stylistic/padding-line-between-statements`, and
  `max-lines-per-function` (Astro's template syntax conflicts with these rules)
- **Vue SFCs** (`**/*.vue`): Consider turning off indent and brace-style rules if the
  template section conflicts
- **Config files** (`*.config.ts`, `*.config.js`): Consider turning off
  `max-lines-per-function` if configs are lengthy

#### Adapt ignorePatterns for the project

Add any project-specific build output directories or generated files to `ignorePatterns`.
Common additions:

- `**/.nuxt/**`, `**/.next/**`, `**/.astro/**` for framework build caches
- `**/build/**`, `**/out/**` for alternative output directories
- `**/*.d.ts` for generated type declarations

#### Add npm scripts

Add these scripts to `package.json`:

```json
{
   "scripts": {
      "lint": "oxlint",
      "lint:fix": "oxlint --fix",
      "fmt": "oxfmt --write .",
      "fmt:check": "oxfmt --check ."
   }
}
```

If the project already has `lint` or `fmt` scripts, adapt the naming or ask the user.

### 4. Final checks

- Run `npm run lint` to verify the linting config works
- Run `npm run fmt:check` to verify formatting works
- Fix any issues that arise from the initial lint/format pass
- Ensure `npm run dev` starts correctly (or at least that the script is defined)

---

## Silvermine (Non-Personal) Project Setup

For Silvermine team projects, follow the standardization guide from
`@silvermine/standardization`. The source of truth is:
https://github.com/silvermine/standardization

The following steps are taken from the README.md file there, so they may be out of date. Fetch the latest to be sure. However, here is a quick reference:

### 1. Install standardization package

```bash
npm install --save-dev --save-exact @silvermine/standardization
```

### 2. EditorConfig

Symlink the EditorConfig file:

```bash
ln -s ./node_modules/@silvermine/standardization/.editorconfig .editorconfig
```

### 3. Commitlint

1. Create `commitlint.config.js` in the project root:

   ```javascript
   'use strict';

   module.exports = {
      extends: [ '@silvermine/standardization/commitlint.js' ],
   };
   ```

2. Find the short hash of the initial commit: `git log --oneline | tail -1`
3. Add the `commitlint` script to `package.json`:

   ```json
   "commitlint": "commitlint --from <short-hash>"
   ```

### 4. Markdownlint

1. Create `.markdownlint-cli2.cjs` in the project root:

   ```javascript
   'use strict';

   const sharedStandards = require(`@silvermine/standardization/.markdownlint-cli2.shared.cjs`);

   module.exports = {
      ...sharedStandards,
   };
   ```

2. Add the `markdownlint` script to `package.json`:

   ```json
   "markdownlint": "markdownlint-cli2"
   ```

### 5. SASS linting (if applicable)

If the project uses SCSS/SASS:

1. Create `.stylelintrc.yml` in the project root:

   ```yml
   extends: ./node_modules/@silvermine/standardization/.stylelintrc.yml
   ```

2. Add the `stylelint` script to `package.json`:

   ```json
   "stylelint": "stylelint './path/to/scss/source/**/*.scss'"
   ```

   Replace the path with the actual SCSS source directory.

### 6. ESLint

If the project uses JavaScript or TypeScript:

1. Install `@silvermine/eslint-config` and `@silvermine/eslint-plugin` per their
   respective README instructions
2. Add the `eslint` script to `package.json`:

   ```json
   "eslint": "eslint ."
   ```

### 7. Standards script

Combine all linting scripts into a single `standards` script in `package.json`:

```json
"standards": "npm run commitlint && npm run markdownlint && npm run eslint"
```

Add `stylelint` to the chain if the project uses SCSS.

### 8. Browserslist (if applicable)

If the project is a front-end application, symlink the appropriate browserslist config:

- **Public-facing projects (broad support)**:

  ```bash
  ln -s ./node_modules/@silvermine/standardization/browserslist/.browserslistrc-broad-support .browserslistrc
  ```

- **Internal projects (narrow support)**:

  ```bash
  ln -s ./node_modules/@silvermine/standardization/browserslist/.browserslistrc-narrow-support .browserslistrc
  ```

### 9. Release process (if publishing to npm)

Add the release scripts to `package.json`:

```json
"release:preview": "node ./node_modules/@silvermine/standardization/scripts/release.js preview",
"release:prep-changelog": "node ./node_modules/@silvermine/standardization/scripts/release.js prep-changelog",
"release:finalize": "node ./node_modules/@silvermine/standardization/scripts/release.js finalize"
```

Ensure `package.json` has a `repository.url` field pointing to the canonical repo URL.

### 10. check-node-version (if enforcing Node/npm versions)

Add a `check-node-version` script to `package.json`:

```json
"check-node-version": "check-node-version --npm <desired-version>"
```

Ask the user which npm version to enforce.

### 11. Final checks

- Run `npm run standards` to verify all linting passes
- Fix any issues from the initial standards run
