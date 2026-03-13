# Legacy Aureus - MCP Implementation

> **⚠️ DEPRECATED** - This folder contains the old TypeScript/Node.js implementation of Aureus.

The active development has moved to `../aureus-cli/` (Rust CLI).

## What's Here

- `plugins/aureus/` - Old MCP server implementation
- `hooks/` - Legacy git hooks
- `tests/` - Vitest test suite
- `package.json` - npm dependencies

## Migration

See `../aureus-cli/MIGRATION.md` for migration guide.

## Why Deprecated?

- **Performance:** Rust CLI is ~20x faster
- **Token overhead:** Hook integration uses 99% fewer tokens
- **Maintenance:** Single binary vs Node.js ecosystem
- **User experience:** Transparent command rewriting

## Rollback

If you need to use the old version:

```bash
cd legacy
npm install
node plugins/aureus/mcp/server.js
```
