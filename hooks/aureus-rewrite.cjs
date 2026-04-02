/**
 * Aureus Auto-Rewrite Hook for Claude Code
 * Transparently rewrites git commit → aureus commit
 *
 * This hook intercepts Bash commands before execution and rewrites them
 * to use Aureus for proper Versioned Release Convention formatting.
 */

const { spawnSync } = require('child_process');

// Cache aureus availability to avoid repeated checks
let aureusChecked = false;
let hasAureus = false;

function checkAureus() {
    if (aureusChecked) return hasAureus;

    try {
        const result = spawnSync('aureus', ['--version'], {
            stdio: 'ignore',
            timeout: 1000,
            shell: false
        });
        hasAureus = result.status === 0;
    } catch {
        hasAureus = false;
    }
    aureusChecked = true;
    return hasAureus;
}

function preToolUse(context, toolName, toolInput) {
    // Only process Bash commands
    if (toolName !== 'Bash') {
        return;
    }

    const command = toolInput?.command;
    if (!command || typeof command !== 'string') {
        return;
    }

    // Check if aureus is available (cached)
    if (!checkAureus()) {
        return;
    }

    // Don't modify if already aureus commit
    if (/^aureus\s+commit/.test(command)) {
        return;
    }

    // Skip commands with heredocs (they break simple regex)
    // Note: git commit -m "msg" is fine, but heredocs like <<EOF need special handling
    if (command.includes('<<')) {
        return;
    }

    // Extract first command (before &&, ||, |, or newline)
    // This handles multi-line commands with heredocs
    const firstCmd = command.split(/&&|\|\||\n|\r/)[0].trim();

    // Match: git commit [options]
    // Use word boundary to avoid matching git-commit or other variants
    const gitCommitRegex = /^git\s+commit\b/;
    if (!gitCommitRegex.test(firstCmd)) {
        return;
    }

    // Rewrite: replace only the "git commit" part with "aureus commit"
    // Keep everything else (including message, flags, heredocs) intact
    const rewritten = command.replace(/^git\s+commit\b/, 'aureus commit');

    return {
        permissionDecision: 'allow',
        permissionDecisionReason: 'Aureus auto-rewrite: git commit → aureus commit',
        updatedInput: {
            ...toolInput,
            command: rewritten
        }
    };
}

module.exports = { preToolUse };
