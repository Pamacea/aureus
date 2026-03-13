# Aureus - Migration Guide: MCP to CLI Rust

> **Status:** 🚧 In Progress - Migration from TypeScript/Node.js to native Rust CLI

## Overview

Aureus is being rewritten as a high-performance native CLI tool, replacing the MCP server architecture with a direct hook-based integration inspired by RTK.

## What's Changing

| Aspect | Old (MCP) | New (CLI Rust) |
|--------|-----------|----------------|
| **Runtime** | Node.js (~100ms startup) | Native Rust (~5ms startup) |
| **Distribution** | npm install | cargo install / binary |
| **Integration** | MCP server + 18 tools | Hook rewrite + CLI commands |
| **Token overhead** | ~2000 tokens (MCP context) | ~10 tokens (hook only) |
| **Dependencies** | Node.js ecosystem | Zero runtime deps |
| **Platform** | Cross-platform via Node | Cross-platform native |

## Migration Timeline

- [x] Phase 1: Foundation (Rust project structure, parsing, git operations)
- [x] Phase 2: CLI Interface (clap commands)
- [x] Phase 3: Storage (SQLite, TOML config)
- [ ] Phase 4: Hook Integration (Claude Code PreToolUse)
- [ ] Phase 5: Testing & Documentation
- [ ] Phase 6: Release v1.0.0

## New CLI Commands

```bash
# Old (MCP tool calls)
git_versioned_commit --type UPDATE --project "Aureus"
git_suggest_version
git_create_release --version 1.1.0

# New (CLI commands)
aureus commit -m "feat: new feature"        # Auto-detects type
aureus suggest                              # Show version suggestions
aureus release --auto                       # Auto-detect version
```

## Hook Integration

The new Aureus uses a **PreToolUse hook** for transparent command rewriting:

```bash
# User says: "Commit the auth feature"
# Claude generates: git commit -m "feat: auth"
# Hook rewrites: aureus commit -m "feat: auth"
# Aureus creates: UPDATE: Project - v1.1.0
```

## Installing the New Aureus

```bash
# From source
cargo install --git https://github.com/Pamacea/aureus aureus

# Or build locally
cd aureus/aureus-cli
cargo build --release
cargo install --path .

# Initialize for Claude Code
aureus init --global
```

## Feature Comparison

| Feature | MCP Version | CLI Rust | Notes |
|---------|-------------|----------|-------|
| Versioned commits | ✅ | ✅ | Same VRC format |
| Auto version bump | ✅ | ✅ | Enhanced detection |
| Amend workflow | ✅ | ✅ | Preserved |
| Release creation | ✅ | ✅ | With CHANGELOG |
| Git hooks | ✅ | ✅ | Cross-platform |
| Web interface | ✅ | ❌ | Deprecated (not needed) |
| MCP tools | 18+ | - | Replaced by CLI |
| Token tracking | ❌ | ✅ | New feature |

## Breaking Changes

1. **No more MCP server** - All functionality moved to CLI
2. **Web interface removed** - CLI + hooks is sufficient
3. **Different install method** - Use cargo or binary instead of npm

## Rollback Plan

If you need to revert to the MCP version:

```bash
# Uninstall CLI
cargo uninstall aureus

# Reinstall from npm (if still published)
npm install -g @pamacea/aureus

# Or use local MCP version
cd aureus
npm install
node plugins/aureus/mcp/server.js
```

## Contributing

The Rust CLI is in `aureus-cli/` directory. All new development happens there.

```bash
cd aureus-cli
cargo test
cargo run -- commit -m "test: message"
```
