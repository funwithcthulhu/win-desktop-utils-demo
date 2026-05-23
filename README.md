# win-desktop-utils-demo

Tiny companion app for
[`win-desktop-utils`](https://github.com/funwithcthulhu/win-desktop-utils).

This repo shows the crate in a small app-shaped binary rather than isolated
library examples. The default run prepares desktop state and prints paths; shell
actions only happen when you pass an explicit flag.

## What It Demonstrates

- `DesktopApp::with_company`
- app-data directory creation
- single-instance startup guard
- elevation checks
- opening URLs with the Windows shell
- revealing an app-data directory in Explorer
- creating an Internet Shortcut `.url` file

## Run

```powershell
cargo run
cargo run -- --elevation
cargo run -- --create-docs-shortcut
cargo run -- --reveal-data-dir
cargo run -- --open-docs
```

The first command does not open UI. It prints the detected platform, the demo
app-data path, and a compact table of supported actions with side-effect labels.
The `--open-*` and `--reveal-*` commands use the Windows shell and may open a
browser or Explorer window. `--create-docs-shortcut` writes or overwrites the
demo docs shortcut in app data.

## Why This Exists

The main crate keeps its API focused. This companion repo gives new users a
quick, concrete app-startup shape they can copy into GUI, tray, launcher, or
local utility projects.

## Links

- Crate: https://crates.io/crates/win-desktop-utils
- Docs: https://docs.rs/win-desktop-utils
- Main repo: https://github.com/funwithcthulhu/win-desktop-utils

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license
