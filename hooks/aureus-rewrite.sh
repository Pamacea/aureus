#!/bin/bash
# Aureus auto-rewrite hook for Claude Code PreToolUse:Bash
# Transparently rewrites git commit → aureus commit with VRC format
#
# This hook intercepts Bash commands before execution and rewrites them
# to use Aureus for proper Versioned Release Convention formatting.

# Dependencies check
if ! command -v aureus &>/dev/null || ! command -v jq &>/dev/null; then
  exit 0
fi

set -euo pipefail

INPUT=$(cat)
CMD=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$CMD" ]; then
  exit 0
fi

# Extract the first meaningful command
FIRST_CMD="$CMD"

# Skip if already using aureus
case "$FIRST_CMD" in
  aureus\ *|*/aureus\ *) exit 0 ;;
esac

# Skip commands with heredocs
case "$FIRST_CMD" in
  *'<<'*) exit 0 ;;
esac

REWRITTEN=""

# === INTERCEPT: git commit ===
if echo "$FIRST_CMD" | grep -qE '^git[[:space:]]+commit([[:space:]]|$)'; then
  # Extract message if present
  if echo "$FIRST_CMD" | grep -q '\-m'; then
    MESSAGE=$(echo "$FIRST_CMD" | sed -n 's/.*-m[[:space:]]*"\([^"]*\)".*/\1/p')
    if [ -n "$MESSAGE" ]; then
      REWRITTEN="aureus commit -m \"$MESSAGE\""
    else
      REWRITTEN="aureus commit"
    fi
  else
    REWRITTEN="aureus commit"
  fi

# === INTERCEPT: git push (future: could auto-release) ===
elif echo "$FIRST_CMD" | grep -qE '^git[[:space:]]+push([[:space:]]|$)'; then
  # For now, let git push through normally
  # Future: could run aureus release --auto before push
  exit 0
fi

# If no rewrite needed, approve as-is
if [ -z "$REWRITTEN" ]; then
  exit 0
fi

# Build the updated tool_input with all original fields preserved
ORIGINAL_INPUT=$(echo "$INPUT" | jq -c '.tool_input')
UPDATED_INPUT=$(echo "$ORIGINAL_INPUT" | jq --arg cmd "$REWRITTEN" '.command = $cmd')

# Output the rewrite instruction
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
