# Aureus - Project Configuration

> **Version:** 1.0.0 (CLI Rust) | **Last Updated:** 2025-03-13

---

## Project Overview

**Aureus** is a native Rust CLI tool implementing the **Versioned Release Convention (VRC)** for Git workflow automation with automatic semantic versioning.

*Named after the ancient Roman gold coin, symbolizing quality and excellence in Git workflow automation.*

---

## Quick Start

```bash
# Install Aureus CLI
cargo install --git https://github.com/Pamacea/aureus aureus

# Initialize for Claude Code
aureus init --global

# Create a versioned commit
aureus commit -m "feat: new feature"
```

---

## CLI Commands Available

| Command | Description |
|---------|-------------|
| `aureus commit` | Create versioned commit |
| `aureus amend` | Amend last commit (same version) |
| `aureus release` | Create release with tag |
| `aureus suggest` | Get version suggestions |
| `aureus init` | Initialize for Claude Code |
| `aureus hooks` | Manage git hooks |
| `aureus config` | Manage configuration |
| `aureus stats` | Show statistics |

---

## Auto-Rewrite Hook

The Aureus hook transparently rewrites `git commit` → `aureus commit`:

1. **Install:** `aureus init --global`
2. **Restart:** Claude Code
3. **Done:** All `git commit` commands automatically use VRC

---

## Project Structure

```
aureus/
├── aureus-cli/         # Rust CLI (ACTIVE DEVELOPMENT)
│   ├── src/
│   │   ├── convention/    # VRC parsing
│   │   ├── git/           # Git operations
│   │   ├── storage/       # Config & state
│   │   └── commands/      # CLI commands
│   └── hooks/
│       └── aureus-rewrite.sh
│
└── legacy/             # Old MCP implementation (DEPRECATED)
    └── plugins/aureus/
```

---

## Requirements

- **Rust:** >= 1.70
- **Git:** >= 2.0.0
- **Claude Code:** Latest version (for hook integration)

---

## License

MIT © Yanis
