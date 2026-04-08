#!/usr/bin/env node
/**
 * Aureus Auto-Rewrite Hook for Claude Code
 * Transparently rewrites git commit → aureus commit
 *
 * Protocol: stdin JSON → stdout JSON (hookSpecificOutput)
 */

const { spawnSync } = require('child_process');

// Cache aureus availability to avoid repeated checks
let aureusChecked = false;
let hasAureus = false;

function checkAureus() {
    if (aureusChecked) return hasAureus;
    try {
        const result = spawnSync('aureus', ['--version'], {
            stdio: 'ignore', timeout: 1000, shell: false
        });
        hasAureus = result.status === 0;
    } catch { hasAureus = false; }
    aureusChecked = true;
    return hasAureus;
}

// ─── Main: read from stdin ───
let input = '';
process.stdin.setEncoding('utf8');
process.stdin.on('data', chunk => { input += chunk; });
process.stdin.on('end', () => {
    let data;
    try { data = JSON.parse(input); } catch { process.exit(0); }

    if (data?.tool_name !== 'Bash') { process.exit(0); }

    const toolInput = data.tool_input || {};
    const command = toolInput.command;
    if (!command || typeof command !== 'string') { process.exit(0); }

    // Check if aureus is available (cached)
    if (!checkAureus()) { process.exit(0); }

    // Don't modify if already aureus commit
    if (/^aureus\s+commit/.test(command)) { process.exit(0); }

    // Skip heredocs
    if (/<<[-~]?\w/.test(command)) { process.exit(0); }

    // Extract first command (before &&, ||, |, or newline)
    const firstCmd = command.split(/&&|\|\||\n|\r/)[0].trim();

    // Match: git commit [options]
    if (!/^git\s+commit\b/.test(firstCmd)) { process.exit(0); }

    // Rewrite: replace only "git commit" with "aureus commit"
    const rewritten = command.replace(/^git\s+commit\b/, 'aureus commit');

    const result = {
        hookSpecificOutput: {
            hookEventName: 'PreToolUse',
            permissionDecision: 'allow',
            permissionDecisionReason: 'Aureus: git commit → aureus commit',
            updatedInput: { ...toolInput, command: rewritten }
        }
    };
    process.stdout.write(JSON.stringify(result));
    process.exit(0);
});
