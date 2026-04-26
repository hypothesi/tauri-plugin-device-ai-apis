# Releasing

This repository releases from GitHub Actions only when a signed and verified `v*` tag is
pushed. The release workflow validates the committed version, reruns release checks, publishes
both Rust crates plus the npm package, and then creates a GitHub release.

## Requirements

1. You have push access to the repository.
2. You have a configured GPG key for Git tag signing, and GitHub can verify that key.
3. npm Trusted Publishing is configured for `@hypothesi/tauri-plugin-device-ai-apis`.
4. The repository has a `CARGO_REGISTRY_TOKEN` secret for crates.io publishing.

For one-time setup, follow `.github/TRUSTED_PUBLISHING_SETUP.md`.

## Files that must stay in sync

Before tagging a release, keep these version fields aligned:

1. `package.json`
2. `Cargo.toml`
3. `crates/device-ai/Cargo.toml`
4. The root `device-ai` dependency entry in `Cargo.toml`

## Release steps

1. Ensure `main` is up to date and clean.

   ```bash
   git checkout main
   git pull --ff-only
   ```

2. Update the release version in the repository before tagging:
   * `package.json`
   * `Cargo.toml`
   * `crates/device-ai/Cargo.toml`
   * the `device-ai` dependency version in `Cargo.toml`

3. Run the release validation and normal quality gates:

   ```bash
   npm run release:validate -- vX.Y.Z
   npm run standards
   cargo test --all-targets --all-features
   npm run build
   cd examples/tauri-app && npm ci && npm run build
   cd ../..
   cargo package --locked --allow-dirty -p device-ai
   cargo package --list --allow-dirty -p tauri-plugin-device-ai-apis
   ```

4. Commit the version changes.

5. Create a signed annotated tag.

   ```bash
   git tag -s vX.Y.Z -m "vX.Y.Z"
   ```

6. Push the branch and the tag.

   ```bash
   git push origin main
   git push origin vX.Y.Z
   ```

## What the release workflow enforces

1. The tag name must match `v*`.
2. The tag must be annotated, not lightweight.
3. GitHub must verify the tag signature.
4. The tag version must match all committed manifest versions.
5. `npm run standards`, `cargo test --all-targets --all-features`, `npm run build`, the
   example app build, and cargo package checks must all pass.
6. `device-ai` publishes before `tauri-plugin-device-ai-apis`.
7. npm publish uses Trusted Publishing with provenance.
8. A GitHub release is created only after publish succeeds.

If any check fails, publish is skipped and the workflow fails.

The plugin crate preflight uses `cargo package --list` because the plugin depends on
`device-ai`, and that dependency is not available from crates.io until the release job has
published it. The actual publish job still performs the full crates.io-backed publish after
`device-ai` is available.
