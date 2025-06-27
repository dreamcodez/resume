#!/usr/bin/env node
/**
 * Trunk Serve Dynamic Port Script
 *
 * This script starts Trunk with port 0 (random port) and communicates
 * the actual port back to Playwright through environment variables.
 * This works around the {{port}} substitution bug in Playwright.
 */

const { spawn } = require("child_process");

// Logging utility
const log = {
  info: (msg) => console.log(`[trunk-serve-dynamic] ${msg}`),
  error: (msg) => console.error(`[trunk-serve-dynamic] ERROR: ${msg}`),
  debug: (msg) => console.log(`[trunk-serve-dynamic] DEBUG: ${msg}`),
};

let childProcess = null;

/**
 * Clean up on exit
 */
function cleanup() {
  if (childProcess) {
    log.info("Cleaning up child process...");
    childProcess.kill("SIGTERM");
  }
}

/**
 * Start trunk serve process with port 0
 */
function runTrunk() {
  log.info("Starting trunk serve with port 0 (dynamic port allocation)...");

  childProcess = spawn("trunk", ["serve", "--port", "0"], {
    stdio: ["inherit", "pipe", "inherit"], // Capture stdout to extract port
    env: { ...process.env },
  });

  log.info(`Child process started with PID: ${childProcess.pid}`);

  // Monitor stdout to extract the port and set environment variable
  childProcess.stdout.on("data", (data) => {
    const output = data.toString();
    console.log(output); // Forward output to console

    // Look for port in Trunk's startup message
    // Trunk typically outputs something like "Serving on http://127.0.0.1:PORT"
    const portMatch = output.match(/Serving on http:\/\/[^:]+:(\d+)/);
    if (portMatch) {
      const port = portMatch[1];
      log.info(`Detected port: ${port}`);

      // Set environment variable for Playwright to read
      process.env.TRUNK_PORT = port;

      // Also write to stdout in a format Playwright can parse
      console.log(`TRUNK_PORT=${port}`);
    }
  });

  childProcess.on("exit", (code, signal) => {
    log.info(`Child process exited with code: ${code}, signal: ${signal}`);
    cleanup();

    if (code === 0) {
      log.info("Server exited successfully");
      process.exit(0);
    } else {
      log.error(`trunk serve failed (exit code ${code}, signal ${signal})`);
      process.exit(1);
    }
  });
}

/**
 * Signal handlers
 */
function setupSignalHandlers() {
  process.on("SIGINT", () => {
    log.info("Received SIGINT, cleaning up...");
    cleanup();
    process.exit(0);
  });

  process.on("SIGTERM", () => {
    log.info("Received SIGTERM, cleaning up...");
    cleanup();
    process.exit(0);
  });

  process.on("exit", () => {
    cleanup();
  });
}

/**
 * Main function
 */
async function main() {
  setupSignalHandlers();
  runTrunk();
}

// Run the script
main().catch((err) => {
  log.error(`Script failed: ${err.message}`);
  cleanup();
  process.exit(1);
});
