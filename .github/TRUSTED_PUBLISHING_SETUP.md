# First-time publishing setup

Complete these steps once before the first signed-tag release.

## npm Trusted Publishing

1. Open the npm package access page for `@hypothesi/tauri-plugin-device-ai-apis`.
2. Add a Trusted Publisher with these values:
   * Provider: `GitHub Actions`
   * Owner or organization: `hypothesi`
   * Repository: `tauri-plugin-device-ai-apis`
   * Workflow file: `release.yml`
   * Environment: leave blank
3. If npm does not allow Trusted Publishing yet because the package has never been published,
   publish the package once from a maintainer account with:

   ```bash
   npm publish --access public
   ```

   Then add the Trusted Publisher and switch back to the tag-driven flow.

## crates.io publishing

1. Create a crates.io token from <https://crates.io/settings/tokens>.
2. Add it to GitHub repository secrets as `CARGO_REGISTRY_TOKEN`.
3. Before the first release, confirm both crate names are either still available or already
   owned by the publishing account:
   * `device-ai`
   * `tauri-plugin-device-ai-apis`
4. Confirm the publishing account is an owner of both crates once they exist.

## Verified signed tags

1. Generate or choose the GPG key you will use for releases.
2. Upload the public key to GitHub.
3. Configure Git locally to sign tags with that key, for example:

   ```bash
   git config --global user.signingkey <key-id>
   git config --global tag.gpgSign true
   ```

4. Create release tags with `git tag -s ...` so GitHub can verify them.
