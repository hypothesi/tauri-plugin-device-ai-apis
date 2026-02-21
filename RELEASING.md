# Releasing

This repository publishes to npm from GitHub Actions only when a signed and
verified tag is pushed.

## Requirements

- You have push access to the repository.
- You have a configured GPG key for Git tag signing.
- npm Trusted Publishing is configured for this repository/package.

## Release steps

1. Ensure `main` is up to date and clean.

```bash
git checkout main
git pull --ff-only
```

2. Create a signed annotated tag.

```bash
git tag -s vX.Y.Z -m "vX.Y.Z"
```

3. Push the tag.

```bash
git push origin vX.Y.Z
```

## What the release workflow enforces

- Tag name must match `v*`.
- Tag must be annotated (not lightweight).
- Tag signature must be verified by GitHub.
- Publish uses npm Trusted Publishing (OIDC) with automatic provenance.

If any check fails, publish is skipped and the workflow fails.
