# OpenMCP

[![CI](https://github.com/jerryan/OpenMCP/actions/workflows/ci.yml/badge.svg)](https://github.com/jerryan/OpenMCP/actions/workflows/ci.yml)

OpenMCP is a lightweight desktop companion for configuring MCP servers and generating the JSON snippets needed to attach them to OpenCode.

## Why this stack

- Native desktop app with `Rust + eframe/egui`
- Fast startup and low runtime overhead
- No Node or browser runtime required for the shipped app
- Cross-platform target for macOS, Linux, and Windows from one codebase

## First release scope

- Guided form for defining MCP servers
- Project profile and global profile separation
- Validation for server id, command, args, and environment variables
- Generated OpenCode-ready JSON preview
- Copyable setup steps and target path guidance
- Direct OpenCode config management: install, uninstall, and refresh installed MCP servers without manually editing JSON
- Local autosave under the user's config directory

## New: File-Backed Install/Uninstall

- App reads and writes the OpenCode config file directly
- One-click install of enabled MCP servers
- One-click uninstall for selected/installed servers
- Current installation state is loaded from file and shown in the UI

## Run

```bash
cargo run
```

## Build

```bash
cargo build --release
```

## Smoke test

```bash
./scripts/smoke.sh
```

Environment options:

- `OPENMCP_SMOKE_RUN_GUI=1` to also run a short GUI startup smoke check (5 seconds). If omitted, it only runs compile/test/build checks.

## Packaging direction

- macOS: signed `.app` bundle later
- Windows: `.exe` now, installer later
- Linux: single binary now, AppImage or `.deb` later

## Notes

This first version focuses on making MCP server wiring understandable and repeatable. It intentionally generates the server block and setup instructions without making risky assumptions about a user's exact OpenCode config file path or merge strategy.
