---
name: test-local-warp
description: Test command-signatures changes locally by running Warp terminal against a local build of this crate. Use when the user wants to verify completion spec changes, generator changes, or other modifications in a real Warp session before merging.
---

# Testing command-signatures with a Local Warp Build

## Overview

`warp-command-signatures` is consumed by the `warp` app as a git dependency. To test changes locally, temporarily point `warp` at a local checkout of this repo, then build and run Warp.

## Prerequisites

- A local checkout of both `warp` and `command-signatures`.
- **Computer use must be enabled** — Warp is a GUI application, so verifying completions requires screen interaction.

## Steps

### 1. Patch the dependency in warp

In the `warp` repo's root `Cargo.toml`, find the `warp-command-signatures` line under `[workspace.dependencies]`:

```toml
warp-command-signatures = { git = "ssh://git@github.com/warpdotdev/command-signatures.git", rev = "...", default-features = false }
```

Replace it with a local path. Adjust the relative path based on where the two repos are checked out:

```toml
warp-command-signatures = { path = "../command-signatures/command-signatures", default-features = false }
```

Note the nested `command-signatures/command-signatures` — the outer directory is the repo root (Cargo workspace), the inner one is the `warp-command-signatures` crate.

### 2. Build and run Warp

From the `warp` repo:

```sh
cargo run --features fast_dev
```

### 3. Verify changes

Open the locally-built Warp and test completions for the commands you modified. Type the command and use the Tab key to trigger the completion menu to confirm specs and generators behave as expected.

> **Note:** The completions menu does not work with equal-sign-delimited options (e.g. `--foo=bar`). Always use space-delimited syntax when testing (e.g. `--foo bar`).

### 4. Clean up

Revert the `Cargo.toml` change in `warp` before committing. The local path override should never be checked in.
