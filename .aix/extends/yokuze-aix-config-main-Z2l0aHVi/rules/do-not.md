---
trigger: always_on
---

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
