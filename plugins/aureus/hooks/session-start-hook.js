#!/usr/bin/env node

/**
 * Git Flow Master - Session Start Hook
 *
 * Automatically starts the web interface if not running,
 * then opens it in the default browser.
 *
 * Cross-platform: Windows, macOS, Linux
 */

const http = require('http');
const { spawn } = require('child_process');
const path = require('path');

const PORT = 3747;
const SERVER_URL = `http://localhost:${PORT}`;

/**
 * Check if the server is already running
 */
async function isServerRunning() {
  return new Promise((resolve) => {
    const req = http.get(SERVER_URL, (res) => {
      resolve(true);
    });

    req.on('error', () => {
      resolve(false);
    });

    req.setTimeout(1000, () => {
      req.destroy();
      resolve(false);
    });
  });
}

/**
 * Wait for server to be ready (polling instead of arbitrary timeout)
 */
async function waitForServer(maxWait = 10000, interval = 100) {
  const start = Date.now();

  while (Date.now() - start < maxWait) {
    if (await isServerRunning()) {
      return true;
    }
    await new Promise(resolve => setTimeout(resolve, interval));
  }

  throw new Error(`Server failed to start within ${maxWait}ms`);
}

/**
 * Start the web server in background
 */
async function startServer() {
  const serverPath = path.join(__dirname, '..', 'web', 'server.js');

  console.log('🚀 Starting Git Flow Master web interface...');

  const proc = spawn('node', [serverPath], {
    stdio: ['ignore', 'pipe', 'pipe'],
    detached: true,
    windowsHide: true
  });

  // Log server output for debugging
  proc.stdout.on('data', (data) => {
    console.log(`[Git Flow Master] ${data.toString().trim()}`);
  });

  proc.stderr.on('data', (data) => {
    console.error(`[Git Flow Master Error] ${data.toString().trim()}`);
  });

  proc.on('error', (err) => {
    console.error(`Failed to start server: ${err.message}`);
  });

  // Detach the process so it continues running
  proc.unref();

  // Wait for server to be actually ready (polling)
  try {
    await waitForServer();
    console.log(`✓ Git Flow Master server started on ${SERVER_URL}`);
  } catch (error) {
    console.error(`✗ Server startup failed: ${error.message}`);
    throw error;
  }
}

/**
 * Open the browser to the web interface
 */
async function openBrowser() {
  const { exec } = require('child_process');

  let command;
  switch (process.platform) {
    case 'win32':
      command = `start "" "${SERVER_URL}"`;
      break;
    case 'darwin':
      command = `open "${SERVER_URL}"`;
      break;
    default:
      command = `xdg-open "${SERVER_URL}"`;
  }

  exec(command, (err) => {
    if (err) {
      console.error(`Failed to open browser: ${err.message}`);
      console.log(`Please open ${SERVER_URL} manually`);
    } else {
      console.log(`✓ Browser opened to ${SERVER_URL}`);
    }
  });
}

/**
 * Main hook execution
 */
async function main() {
  try {
    // Check if server is already running
    const isRunning = await isServerRunning();

    if (isRunning) {
      console.log('✓ Git Flow Master web interface is already running');
    } else {
      // Start the server (with polling wait)
      await startServer();
    }

    // Open browser (or show URL if already running)
    await openBrowser();

  } catch (error) {
    console.error('Session start hook error:', error.message);
    // Don't exit with error code, just log
    // This prevents breaking the entire session if browser auto-open fails
  }
}

// Run the hook
main();
