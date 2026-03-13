# Aureus VRC - Project Configuration

> **Version:** 0.9.2 (CLI Rust) | **Last Updated:** 2025-03-13

> **Aureus VRC Integration:** See `~/.claude/AUREUS.md` for Versioned Release Convention details.

---

## Project Overview

**Aureus VRC** is a native Rust CLI tool implementing the **Versioned Release Convention (VRC)** for Git workflow automation with automatic semantic versioning.

*Named after the ancient Roman gold coin, symbolizing quality and excellence in Git workflow automation.*

---

## Quick Start

```bash
# Install Aureus VRC CLI
cargo install aureus-vrc

# Initialize for Claude Code
aureus-vrc init --global

# Create a versioned commit
aureus-vrc commit -m "feat: new feature"
```

---

## CLI Commands Available

| Command | Description |
|---------|-------------|
| `aureus-vrc commit` | Create versioned commit |
| `aureus-vrc amend` | Amend last commit (same version) |
| `aureus-vrc release` | Create release with tag |
| `aureus-vrc suggest` | Get version suggestions |
| `aureus-vrc init` | Initialize for Claude Code |
| `aureus-vrc hooks` | Manage git hooks |
| `aureus-vrc config` | Manage configuration |
| `aureus-vrc stats` | Show statistics |
| `aureus-vrc update` | Update to latest version |
| `aureus-vrc completion` | Generate shell completion |

---

## Auto-Rewrite Hook

The Aureus VRC hook transparently rewrites `git commit` → `aureus-vrc commit`:

1. **Install:** `aureus-vrc init --global`
2. **Restart:** Claude Code
3. **Done:** All `git commit` commands automatically use VRC

---

## Shell Completion

Generate completion scripts for your shell:

```bash
# Bash
aureus-vrc completion bash > ~/.local/share/bash-completion/completions/aureus-vrc

# Zsh
aureus-vrc completion zsh > ~/.zsh/completions/_aureus-vrc

# Fish
aureus-vrc completion fish > ~/.config/fish/completions/aureus-vrc.fish

# PowerShell
aureus-vrc completion powershell | Out-File -Append $PROFILE
```

---

## Project Structure

```
aureus/
├── src/
│   ├── convention/    # VRC parsing
│   ├── git/           # Git operations
│   ├── storage/       # Config & state
│   └── commands/      # CLI commands
│       ├── commit.rs
│       ├── amend.rs
│       ├── release.rs
│       ├── update.rs  # Self-update
│       └── completion.rs  # Shell completions
├── tests/
└── .legacy/           # Old MCP implementation (deprecated)
```

---

## Requirements

- **Rust:** >= 1.70
- **Git:** >= 2.0.0
- **Claude Code:** Latest version (for hook integration)
- **jq:** For hook functionality (cross-platform JSON parsing)

---

## Development

```bash
# Build
cargo build --release

# Run tests
cargo test

# Check for updates
aureus-vrc update

# Generate completions
aureus-vrc completion <shell>
```

---

## License

MIT © Yanis
