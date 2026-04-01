# OpenMCP Architecture

## Product goal

OpenMCP helps a user go from "I have an MCP server command" to "I can paste the right config into OpenCode" without hand-editing JSON from scratch.

## Core workflow

1. Pick profile scope: global or project
2. Enter a server definition
3. Review generated JSON
4. Copy JSON and follow the path instructions for OpenCode
5. Save the profile locally for later reuse
6. Install/Uninstall MCP servers directly into the OpenCode config file from the UI

## Data model

- `WorkspaceProfile`
  - Profile metadata
  - Collection of `McpServer`
- `McpServer`
  - `id`
  - `transport`
  - `command`
  - `args`
  - `cwd`
  - `env`
  - `notes`

## UX principles

- Prefer explicit fields over hidden magic
- Keep the generated config visible at all times
- Validate early and explain errors plainly
- Show safe defaults, but keep paths editable

## Persistence

Profiles are stored as JSON in the user config directory:

- macOS: `~/Library/Application Support/openmcp/profiles.json`
- Linux: `~/.config/openmcp/profiles.json`
- Windows: `%APPDATA%\openmcp\profiles.json`

## Config File Management

- Read and write the actual OpenCode JSON config under the user-specified target path.
- Keep unrelated keys untouched and only mutate `mcpServers`.
- Create timestamped backup files before each write (`.backup-<timestamp>`).
- Support one-click install and uninstall actions per server and bulk "enabled only" operations.

## Packaging rationale

`eframe/egui` provides a single Rust-native codebase with a fast feedback loop and reasonably small packaged output, which fits the "light and fast" constraint better than an Electron-class runtime.
