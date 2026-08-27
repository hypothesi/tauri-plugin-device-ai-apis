# GitHub Actions workflows

This repository has three GitHub Actions workflows:

1. `test.yml` runs standards, Rust tests, the JavaScript build, and the example app build on
   pushes to `main` and on pull requests.
2. `build-platforms.yml` runs cross-platform compile gates on pushes to `main` and on pull
   requests:
   - `rust-desktop` — `cargo check` on Ubuntu, Windows, and macOS runners (the macOS runner
     is the only cheap CI place where the `cfg(macos)` objc/Speech code compiles; Windows
     covers `windows.rs`).
   - `rust-mobile-check` — `cargo check --target aarch64-linux-android`
     (type-check only, no linking, no NDK needed). The iOS Rust target can't
     be checked on Linux (Apple SDK headers) — it is covered by
     `ios-example` on a macOS runner.
   - `android-example` — builds the example app's Android APK, compiling the Kotlin plugin
     (`DeviceAiPlugin.kt`). This is the only gate that catches Kotlin compile errors.
   - `ios-example` builds the Swift package and the example app for the iOS simulator.
     The direct package build compiles `DeviceAiPlugin.swift`; simulator builds need no
     signing.
3. `release.yml` runs only when a `v*` tag is pushed. The tag must be an annotated tag with a
   GitHub-verified signature.

## Release workflow behavior

When a signed release tag is pushed, `release.yml` does this:

1. Verifies the tag is annotated and GitHub marks its signature as verified.
2. Confirms the tag version matches:
   * `package.json`
   * `Cargo.toml`
   * `crates/device-ai/Cargo.toml`
   * the root crate's `device-ai` dependency version
3. Re-runs the repository release gates:
   * `npm run standards`
   * `cargo test --all-targets --all-features`
   * `npm run build`
   * the example app build
   * `cargo package --locked` for `device-ai`
   * `cargo package --list` for the plugin crate, because the plugin depends on `device-ai`
     being present on crates.io before a full publish verification can succeed
4. Publishes `device-ai` to crates.io.
5. Publishes `tauri-plugin-device-ai-apis` to crates.io, retrying while the `device-ai`
   release propagates through the crates.io index.
6. Publishes `@hypothesi/tauri-plugin-device-ai-apis` to npm with provenance.
7. Creates a GitHub release for the tag with generated release notes.

## Required repository configuration

Before the first release, complete the one-time setup in `.github/TRUSTED_PUBLISHING_SETUP.md`.

The workflow expects:

1. npm Trusted Publishing for `@hypothesi/tauri-plugin-device-ai-apis`
2. a repository secret named `CARGO_REGISTRY_TOKEN`
3. maintainers who create release tags to have a GPG key configured locally and uploaded to
   GitHub so tag signatures verify
