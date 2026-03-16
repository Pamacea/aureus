//! Init command implementation

use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;

use crate::cli::InitCommand;

const AUREUS_MD: &str = r#"# Aureus VRC Integration

> **Note**: Add this to your project's CLAUDE.md:
> ```markdown
> See \`~/.claude/AUREUS.md\` for Versioned Release Convention (VRC) details.
> ```

Aureus VRC provides **Versioned Release Convention** for Git workflows.

## Quick Start

```bash
# Create a versioned commit
git commit -m "feat: new feature"
# → Automatically rewritten to: aureus-vrc commit

# Suggest versions
aureus-vrc suggest

# Create a release
aureus-vrc release --auto
```

## Convention Configuration

### Commit Format

```
TYPE: PROJECT - vX.Y.Z

- Change description
```

### Commit Types

| Type | SemVer | Trigger Keywords | Usage |
|------|--------|------------------|-------|
| **RELEASE** | MAJOR (X.0.0) | `!`, `BREAKING`, `breaking` | Breaking changes |
| **UPDATE** | MINOR (0.X.0) | `feat`, `refactor`, `add` | New features |
| **PATCH** | PATCH (0.0.X) | `fix`, `bug`, `patch` | Bug fixes |

### Auto-Detection Rules

When you run `git commit -m "..."`, Aureus auto-detects the type:

```bash
git commit -m "feat: add authentication"
# → UPDATE: MyProject - v1.1.0

git commit -m "fix: login bug"
# → PATCH: MyProject - v1.1.1

git commit -m "BREAKING: change API"
# → RELEASE: MyProject - v2.0.0
```

### Customizing Convention

Edit this file to customize keywords:

```markdown
## Aureus Custom Convention

### Release Keywords
! BREAKING breaking major refactor API-change

### Update Keywords
feat feature added new refactor enhance improve

### Patch Keywords
fix bugfix patch corrected hotfix typo
```

### Project Name Detection

1. **Config**: Set in `~/.aureus-vrc/config.toml`:
   ```toml
   [project]
   name = "MyProject"
   ```

2. **Auto**: Falls back to directory name

3. **Override**: Use `aureus-vrc commit --project CustomName`

## Commands Reference

| Command | Description |
|---------|-------------|
| `aureus-vrc commit -m "msg"` | Create versioned commit |
| `aureus-vrc amend -m "more info"` | Amend last commit (same version) |
| `aureus-vrc release --auto` | Create release with tag |
| `aureus-vrc suggest` | Show version suggestions |
| `aureus-vrc config set project.name X` | Set project name |
| `aureus-vrc hooks status` | Check hooks status |

## Hook Behavior

The `PreToolUse` hook intercepts:
- `git commit -m "message"` → `aureus-vrc commit -m "message"`
- `git commit` (no message) → `aureus-vrc commit` (prompts for message)

To bypass: `git commit --no-verify`

## Token Savings

Using Aureus saves tokens by:
- ✅ No MCP server overhead (native CLI)
- ✅ Auto-formatting commit messages
- ✅ Version auto-detection from keywords
- ✅ Single binary (~3MB RAM vs ~50MB for Node.js)
"#;

const HOOK_SCRIPT_BASH: &str = r#"#!/bin/bash
# Aureus auto-rewrite hook for Claude Code PreToolUse (Unix/macOS/Linux)
# Transparently rewrites git commit → aureus-vrc commit

if ! command -v aureus-vrc &>/dev/null || ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
CMD=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Skip if not git commit
if ! echo "$CMD" | grep -qE '^git[[:space:]]+commit'; then
  exit 0
fi

# Extract message if present
MESSAGE=""
if echo "$CMD" | grep -q '\-m'; then
  MESSAGE=$(echo "$CMD" | sed -n 's/.*-m[[:space:]]*"\([^"]*\)".*/\1/p')
fi

# Rewrite to aureus-vrc commit
if [ -n "$MESSAGE" ]; then
  REWRITTEN="aureus-vrc commit -m \"$MESSAGE\""
else
  REWRITTEN="aureus-vrc commit"
fi

ORIGINAL_INPUT=$(echo "$INPUT" | jq -c '.tool_input')
UPDATED_INPUT=$(echo "$ORIGINAL_INPUT" | jq --arg cmd "$REWRITTEN" '.command = $cmd')

jq -n \
  --argjson updated "$UPDATED_INPUT" \
  '{
    "hookSpecificOutput": {
      "hookEventName": "PreToolUse",
      "permissionDecision": "allow",
      "permissionDecisionReason": "Aureus auto-rewrite",
      "updatedInput": $updated
    }
  }'
"#;

const HOOK_SCRIPT_POWERSHELL: &str = r#"# Aureus auto-rewrite hook for Claude Code PreToolUse (Windows)
# Transparently rewrites git commit to aureus-vrc commit
# Requires: aureus-vrc CLI

$ErrorActionPreference = "SilentlyContinue"

# Check if aureus-vrc is available
$hasAureus = Get-Command aureus-vrc -ErrorAction SilentlyContinue

if (-not $hasAureus) {
    exit 0
}

# Read input from stdin
$inputObj = $Input | ConvertFrom-Json
$CMD = $inputObj.tool_input.command

# Skip if not git commit
if ($CMD -notmatch "^git\s+commit") {
    exit 0
}

# Extract message if present
$MESSAGE = ""
if ($CMD -match '\-m\s+"([^"]+)"') {
    $MESSAGE = $matches[1]
}

# Rewrite to aureus-vrc commit
if ($MESSAGE) {
    $REWRITTEN = "aureus-vrc commit -m `"$MESSAGE`""
} else {
    $REWRITTEN = "aureus-vrc commit"
}

# Build output
$UPDATED = @{
    command = $REWRITTEN
} | ConvertTo-Json -Compress

$result = @{
    hookSpecificOutput = @{
        hookEventName = "PreToolUse"
        permissionDecision = "allow"
        permissionDecisionReason = "Aureus auto-rewrite"
        updatedInput = ($UPDATED | ConvertFrom-Json)
    }
}

Write-Output ($result | ConvertTo-Json -Compress)
"#;

pub fn execute(cmd: InitCommand) -> Result<()> {
    if cmd.global {
        init_global(cmd.force, cmd.no_hooks)
    } else {
        init_local(cmd.force)
    }
}

fn init_global(force: bool, no_hooks: bool) -> Result<()> {
    println!("\n{}", "Initializing Aureus for Claude Code...".bold().cyan());

    let home = dirs::home_dir().context("Cannot determine home directory")?;
    let aureus_dir = home.join(".aureus");
    let claude_dir = home.join(".claude");

    // Create directories
    fs::create_dir_all(&aureus_dir)
        .context("Failed to create .aureus directory")?;

    fs::create_dir_all(&claude_dir.join("hooks"))
        .context("Failed to create .claude/hooks directory")?;

    // Write AUREUS.md
    let aureus_md_path = claude_dir.join("AUREUS.md");
    if !aureus_md_path.exists() || force {
        fs::write(&aureus_md_path, AUREUS_MD)
            .context("Failed to write AUREUS.md")?;
        println!("  ✓ {}", "Created ~/.claude/AUREUS.md".green());
    } else {
        println!("  {}", "~/.claude/AUREUS.md already exists".dimmed());
    }

    // Install hook if not disabled
    if !no_hooks {
        let is_windows = cfg!(windows);
        let (hook_filename, hook_content) = if is_windows {
            ("aureus-rewrite.ps1", HOOK_SCRIPT_POWERSHELL)
        } else {
            ("aureus-rewrite.sh", HOOK_SCRIPT_BASH)
        };

        let hook_path = claude_dir.join("hooks").join(hook_filename);
        if !hook_path.exists() || force {
            // Convert LF to CRLF on Windows for PowerShell scripts
            let content_to_write = if is_windows {
                hook_content.replace('\n', "\r\n")
            } else {
                hook_content.to_string()
            };

            fs::write(&hook_path, content_to_write)
                .context("Failed to write hook script")?;

            // Make executable on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&hook_path)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&hook_path, perms)?;
            }

            println!("  ✓ {}", format!("Created ~/.claude/hooks/{}", hook_filename).green());
        }

        // Update settings.json
        let _ = update_settings_json(&claude_dir, hook_filename);
    }

    println!();
    println!("✓ {}", "Aureus initialized successfully!".green());
    println!();
    println!("{}", "Next steps:".bold());
    println!("  1. Restart Claude Code");
    println!("  2. Try: {}", "git commit -m \"feat: new feature\"".cyan());
    println!("     → Will be rewritten to aureus-vrc commit automatically");

    Ok(())
}

fn init_local(force: bool) -> Result<()> {
    let cwd = std::env::current_dir()
        .context("Cannot get current directory")?;

    let aureus_md_path = cwd.join("AUREUS.md");

    if !aureus_md_path.exists() || force {
        fs::write(&aureus_md_path, AUREUS_MD)
            .context("Failed to write AUREUS.md")?;
        println!("✓ {}", "Created AUREUS.md".green());
    } else {
        println!("{}", "AUREUS.md already exists".dimmed());
    }

    Ok(())
}

fn update_settings_json(claude_dir: &std::path::Path, hook_filename: &str) -> Result<()> {
    let settings_path = claude_dir.join("settings.json");

    let mut settings = if settings_path.exists() {
        let content = fs::read_to_string(&settings_path)?;
        serde_json::from_str(&content).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    // Ensure hooks.PreToolUse exists
    if !settings["hooks"]["PreToolUse"].is_array() {
        settings["hooks"]["PreToolUse"] = serde_json::json!([]);
    }

    let is_windows = cfg!(windows);
    let hook_path = format!("~/.claude/hooks/{}", hook_filename);

    // Determine matcher based on platform
    let matcher = if is_windows { "Command" } else { "Bash" };

    // Clean up ALL existing aureus-rewrite hooks (removes duplicates)
    let pre_tool_uses = settings["hooks"]["PreToolUse"].as_array_mut()
        .context("Invalid hooks.PreToolUse format")?;

    let filtered_hooks: Vec<_> = pre_tool_uses.iter()
        .filter(|entry| {
            // Keep entries that are NOT aureus-rewrite hooks
            !entry.get("hooks")
                .and_then(|h| h.as_array())
                .map(|hooks| {
                    hooks.iter().any(|h| {
                        h.get("type")
                            .and_then(|t| t.as_str())
                            .map(|t| t == "command")
                            .unwrap_or(false)
                            && h.get("command")
                                .and_then(|c| c.as_str())
                                .map(|c| c.contains("aureus-rewrite"))
                                .unwrap_or(false)
                    })
                })
                .unwrap_or(false)
        })
        .cloned()
        .collect();

    // Replace with filtered hooks (no duplicates)
    *pre_tool_uses = filtered_hooks;

    // Add the single clean hook
    let aureus_hook = serde_json::json!({
        "matcher": matcher,
        "hooks": [{
            "type": "command",
            "command": hook_path
        }]
    });

    pre_tool_uses.push(aureus_hook);
    let _ = fs::write(&settings_path, serde_json::to_string_pretty(&settings)?);
    println!("  ✓ {}", "Updated ~/.claude/settings.json (cleaned duplicates)".green());

    Ok(())
}
