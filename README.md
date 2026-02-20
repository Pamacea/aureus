# ⚡ Claude Git - Git Flow Master Plugin

> **Version:** 0.7.1
> **Author:** Yanis
> **Category:** Version Control

---

## 🎯 Overview

**Claude Git** is a comprehensive Git automation plugin for Claude Code that implements the **Versioned Release Convention** - a structured approach to commit messages and version management, with a **premium modern UI** inspired by CloudMem.

### ✨ Key Features

- **🎨 Premium Web Interface** - Modern UI at http://localhost:3747
  - Light/dark theme with auto system detection
  - Real-time statistics dashboard
  - Slide-in sidebar for settings
  - Toast notifications (non-blocking)
  - GPU-accelerated animations

- **🚀 Auto-Start** - Web interface auto-launches on session start
  - Server starts automatically if not running
  - Browser opens to dashboard
  - Cross-platform (Windows/macOS/Linux)

- **📊 Versioned Release Convention** - Structured commits
  - Format: `TYPE: Project Name - vX.Y.Z`
  - SemVer versioning (MAJOR/MINOR/PATCH)
  - Auto-generated commit messages

- **🔧 Smart Automation**
  - Auto-generate versioned commits
  - Amend workflow for small fixes
  - Auto releases with CHANGELOG
  - Conflict resolution assistance

- **🛡️ Security & Performance**
  - XSS protection with input validation
  - CSP headers configured
  - Memory leak prevention
  - Optimized GPU rendering

---

## 📦 Installation

### From Claude Code Marketplace

```bash
# In Claude Code
/install-plugin claude-git
```

### Manual Installation

```bash
git clone https://github.com/Pamacea/claude-git.git
cd claude-git
npm install
```

---

## 🚀 Quick Start

### 1. Auto-Start (Automatic)

When installed, the plugin automatically:
- ✅ Starts the web interface at **http://localhost:3747**
- ✅ Opens your default browser
- ✅ Detects git repositories
- ✅ Shows real-time statistics

### 2. Create a Versioned Commit

```
User: Create a commit for the new authentication feature
```

The plugin will:
1. Analyze staged changes
2. Get version suggestions from API
3. Generate message: `UPDATE: My Project - v1.1.0`
4. Execute the commit

### 3. Use MCP Tools

```bash
# Get version suggestions
git_suggest_version

# Create versioned commit
git_versioned_commit --type UPDATE --project "My Project"

# Amend last commit (keeps version)
git_amend_commit

# Create release with tag
git_create_release --version 1.1.0
```

---

## 📝 Versioned Release Convention

### Format

```
TYPE: PROJECT NAME - vVERSION

[optional body with bullet points]
```

### Commit Types

| Type | Description | SemVer Bump | Example |
|------|-------------|-------------|---------|
| **RELEASE** | Major release - Breaking changes | MAJOR | `RELEASE: My Project - v2.0.0` |
| **UPDATE** | Minor update - New features | MINOR | `UPDATE: My Project - v1.1.0` |
| **PATCH** | Patch - Bug fixes, improvements | PATCH | `PATCH: My Project - v1.0.1` |

### Examples

#### RELEASE Example (Major)
```
RELEASE: Git Flow Master - v2.0.0

- Breaking: Redesigned commit message format
- Breaking: Changed hook configuration structure
- Added: New amend workflow for small fixes
```

#### UPDATE Example (Minor)
```
UPDATE: Git Flow Master - v1.1.0

- Added: Premium web interface with light/dark theme
- Added: Auto-start on session launch
- Added: Real-time status API endpoint
- Improved: Cross-platform compatibility
```

#### PATCH Example (Patch)
```
PATCH: Git Flow Master - v1.0.1

- Fixed: Memory leak in event listeners
- Fixed: XSS vulnerability in API responses
- Fixed: Race condition in server startup
```

---

## 🎨 Web Interface

Access the premium dashboard at **http://localhost:3747**

### Features

#### 📊 Statistics Dashboard
- Repository count
- Hooks installed
- Recent commits
- Server uptime

#### ⚙️ Settings Sidebar
- Convention configuration editor
- Project name customization
- Default commit type selection

#### 🎯 Quick Actions
- **Scan All Repositories** - Discover git repos
- **Load Current Repo** - Track working directory
- **Refresh State** - Update dashboard

#### 🌓 Theme System
- **Light mode** - Clean, bright interface
- **Dark mode** - Easy on the eyes
- **Auto detection** - Follows system preference
- **Persistent** - Saved in localStorage

### API Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /api/status` | Server health & statistics |
| `GET /api/config` | Get configuration |
| `PUT /api/config` | Update configuration |
| `GET /api/state` | Get tracked repositories |
| `GET /api/suggest/version` | Get version suggestions |
| `POST /api/repo/hooks/install` | Install git hooks |
| `POST /api/repo/commit` | Create a commit |

---

## 🔧 MCP Tools (18+ Tools)

All tools available via MCP protocol:

| Tool | Description |
|------|-------------|
| `git_versioned_commit` | Create versioned commit |
| `git_amend_commit` | Amend last commit (same version) |
| `git_suggest_version` | Get version suggestions |
| `git_get_last_commit` | Get last commit details |
| `git_validate_message` | Validate commit format |
| `git_generate_message` | Generate versioned message |
| `git_get_status` | Repository status |
| `git_get_log` | Commit history |
| `git_get_branch` | Branch information |
| `git_get_diff` | Staged/unstaged diff |
| `git_create_release` | Create release with tag |
| `git_get_tags` | List version tags |
| `git_install_hooks` | Install git hooks |
| `git_uninstall_hooks` | Uninstall git hooks |
| `git_analyze_commits` | Analyze for version bump |
| `git_get_config` | Get plugin config |
| `git_update_config` | Update plugin config |
| `git_get_tracked_repos` | List tracked repositories |

---

## 🪝 Git Hooks

### Pre-Commit Hook
- ✅ Secret scanning
- ✅ Linting
- ✅ Type checking
- ✅ Tests (optional)

### Commit Message Hook
- ✅ Validates Versioned Release Convention
- ✅ Checks type (RELEASE/UPDATE/PATCH)
- ✅ Enforces version format (vX.Y.Z)

### Cross-Platform Support
- **Unix**: `.sh` scripts with execute permissions
- **Windows**: `.ps1` PowerShell scripts with batch wrappers

---

## ⚙️ Configuration

Create `.git-flow-config.json` in your project root:

```json
{
  "project": {
    "name": "My Project",
    "defaultBranch": "main"
  },
  "commit": {
    "types": {
      "RELEASE": "Major release - Breaking changes",
      "UPDATE": "Minor update - New features",
      "PATCH": "Patch - Bug fixes and improvements"
    },
    "rules": {
      "subjectMaxLength": 100,
      "requireVersion": true,
      "requireProjectName": true
    }
  },
  "release": {
    "bumpMajor": ["RELEASE"],
    "bumpMinor": ["UPDATE"],
    "bumpPatch": ["PATCH"],
    "changelogFile": "CHANGELOG.md"
  },
  "hooks": {
    "preCommit": {
      "lint": true,
      "typecheck": true,
      "test": false,
      "secretScan": true
    },
    "commitMsg": {
      "validate": true,
      "allowAmend": true
    }
  }
}
```

---

## 🎯 Skills

| Skill | Description |
|-------|-------------|
| `/versioned-commit` | Create versioned commit |
| `/amend-commit` | Amend last commit |
| `/auto-release` | Create release from commits |
| `/fix-conflict` | Resolve merge conflicts |
| `/suggest-version` | Get version suggestions |

---

## 🔄 Amend Workflow

For small fixes to an existing release, **amend** instead of creating a new version:

```
User: Amend the commit with a small fix
```

Result:
```
PATCH: My Project - v1.0.1

- Fixed: Pre-commit hook pattern
- Fixed: Additional edge case  ← Added via amend
```

---

## 📋 Requirements

- **Node.js**: >= 18.0.0
- **Git**: >= 2.0.0
- **Claude Code**: Latest version
- **Browser**: Chrome, Firefox, Safari, Edge (for web interface)

---

## 📁 Project Structure

```
claude-git/
├── .claude-plugin/
│   └── marketplace.json       # Marketplace configuration
├── plugins/
│   └── git-master/
│       ├── .claude-plugin/
│       │   └── plugin.json    # Plugin configuration
│       ├── agents/
│       │   └── system.md      # Agent system prompt
│       ├── skills/
│       │   └── *.md          # Skill documentation
│       ├── hooks/
│       │   ├── session-start-hook.js  # Auto-start web UI
│       │   ├── pre-commit.ps1
│       │   ├── commit-msg.ps1
│       │   └── ...
│       ├── mcp/
│       │   └── server.js      # MCP server
│       ├── web/
│       │   ├── server.js      # Web interface server
│       │   └── public/
│       │       ├── index.html     # Premium UI
│       │       ├── styles.css     # Theme system
│       │       ├── app.js         # Alpine.js logic
│       │       ├── app-v070.js    # v0.7.0 features
│       │       └── toast.js       # Notifications
│       └── .git-flow-config.json
├── README.md                   # This file
├── CHANGELOG.md                # Version history
└── ADVERSARIAL_REVIEW_v0.7.0.md # Security audit
```

---

## 🆕 What's New in v0.7.1

### Security Fixes
- ✅ Fixed memory leak in event listeners
- ✅ Fixed XSS vulnerability (API validation)
- ✅ Fixed race condition in server startup

### New Features
- ✨ Toast notification system (replaces alert())
- ✨ Input validation & sanitization
- ✨ Improved accessibility (ARIA labels)

### Performance
- ⚡ Removed excessive GPU acceleration
- ⚡ Optimized CSS rendering

### UI/UX
- 🎨 Light/dark theme with auto-detection
- 🎨 Sidebar for settings
- 🎨 Real-time status indicator
- 🎨 Statistics dashboard with animations

See [CHANGELOG.md](./CHANGELOG.md) for full version history.

---

## 📄 License

MIT © Yanis

---

## 🔗 Links

- [Versioned Release Convention](./plugins/git-master/docs/GIT_CONVENTIONS.md)
- [Web Interface README](./plugins/git-master/web/README.md)
- [MCP Server README](./plugins/git-master/mcp/README.md)
- [Hooks README](./plugins/git-master/hooks/README.md)
- [GitHub Repository](https://github.com/Pamacea/claude-git)
- [Issue Tracker](https://github.com/Pamacea/claude-git/issues)

---

**Made with ⚡ by Yanis • Powered by Claude Code**
