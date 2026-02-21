# Example Tauri app

This sample app exercises the local `@hypothesi/tauri-plugin-device-ai-apis` package and
the local Rust plugin crate end to end.

## What it is for

- `npm run dev`: browser-only Vite preview. Useful for the JS package's browser fallbacks,
  but it does not load the Rust plugin.
- `npm run tauri dev`: desktop Tauri run against the local plugin checkout.
- `npm run tauri android dev` / `npm run tauri ios dev`: mobile runs through the
  plugin-hosted Android/iOS bridge once the platform toolchains are installed.

## Typical local workflow

From the repository root:

```bash
npm run build
cd examples/tauri-app
npm run tauri dev
```

Run the root build first after changing `guest-js/`, because this example consumes the
locally built package output.
