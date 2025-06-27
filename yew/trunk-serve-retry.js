#!/usr/bin/env node
/**
 * Trunk Serve Retry Script
 *
 * This script manages the Yew development server with automatic port conflict resolution.
 * It's designed to work around Playwright 1.53.1's bug with {{port}} substitution.
 *
 * Features:
 * - Automatic port conflict resolution
 * - Health checks to ensure server readiness
 * - Retry logic for failed server starts
 * - Proper signal handling for cleanup
 */

const { spawn } = require("child_process");
const http = require("http");

// Configuration
const CONFIG = {
  DEFAULT_PORT: process.env.PORT || "8080",
  MAX_RETRIES: 3,
  HEALTH_CHECK_ATTEMPTS: 30,
  HEALTH_CHECK_INTERVAL: 1000, // 1 second
  HEALTH_CHECK_TIMEOUT: 2000, // 2 seconds
  RETRY_DELAY: 2000, // 2 seconds
};

// Logging utility
const log = {
  info: (msg) => console.log(`[trunk-serve-retry] ${msg}`),
  error: (msg) => console.error(`[trunk-serve-retry] ERROR: ${msg}`),
  debug: (msg) => console.log(`[trunk-serve-retry] DEBUG: ${msg}`),
};

// Main state
let attempt = 0;
let childProcess = null;

/**
 * Find an available port starting from the given port
 */
function findAvailablePort(startPort) {
  return new Promise((resolve, reject) => {
    const server = http.createServer();

    server.listen(startPort, () => {
      const port = server.address().port;
      server.close(() => resolve(port));
    });

    server.on("error", (err) => {
      if (err.code === "EADDRINUSE") {
        log.debug(`Port ${startPort} is in use, trying ${startPort + 1}`);
        findAvailablePort(startPort + 1)
          .then(resolve)
          .catch(reject);
      } else {
        reject(err);
      }
    });
  });
}

/**
 * Health check to verify server is ready
 */
function waitForServer(port) {
  return new Promise((resolve, reject) => {
    let attempts = 0;

    const checkServer = () => {
      attempts++;
      log.debug(
        `Health check attempt ${attempts}/${CONFIG.HEALTH_CHECK_ATTEMPTS} for port ${port}`
      );

      const req = http.get(`http://localhost:${port}`, (res) => {
        log.info(`Server is ready! Status: ${res.statusCode}`);
        resolve(port);
      });

      req.on("error", (err) => {
        if (attempts >= CONFIG.HEALTH_CHECK_ATTEMPTS) {
          reject(
            new Error(
              `Server health check failed after ${CONFIG.HEALTH_CHECK_ATTEMPTS} attempts`
            )
          );
        } else {
          log.debug(
            `Server not ready yet, retrying in ${CONFIG.HEALTH_CHECK_INTERVAL}ms...`
          );
          setTimeout(checkServer, CONFIG.HEALTH_CHECK_INTERVAL);
        }
      });

      req.setTimeout(CONFIG.HEALTH_CHECK_TIMEOUT, () => {
        req.destroy();
        if (attempts >= CONFIG.HEALTH_CHECK_ATTEMPTS) {
          reject(
            new Error(
              `Server health check timeout after ${CONFIG.HEALTH_CHECK_ATTEMPTS} attempts`
            )
          );
        } else {
          setTimeout(checkServer, CONFIG.HEALTH_CHECK_INTERVAL);
        }
      });
    };

    checkServer();
  });
}

/**
 * Start trunk serve process
 */
function runTrunk(port) {
  attempt++;
  log.info(
    `Starting trunk serve (attempt ${attempt}/${CONFIG.MAX_RETRIES}) on port ${port}...`
  );

  childProcess = spawn("trunk", ["serve", "--port", port.toString()], {
    stdio: "inherit",
    env: { ...process.env, PORT: port.toString() },
  });

  log.info(`Child process started with PID: ${childProcess.pid}`);

  childProcess.on("exit", (code, signal) => {
    log.info(`Child process exited with code: ${code}, signal: ${signal}`);

    if (code === 0) {
      log.info("Server exited successfully");
      process.exit(0);
    } else {
      if (attempt < CONFIG.MAX_RETRIES) {
        log.error(
          `trunk serve failed to bind port ${port} (exit code ${code}, signal ${signal}). Retrying in ${CONFIG.RETRY_DELAY}ms...`
        );
        setTimeout(() => runTrunk(port), CONFIG.RETRY_DELAY);
      } else {
        log.error(
          `trunk serve failed after ${CONFIG.MAX_RETRIES} attempts. Port ${port} may be in use or unavailable.`
        );
        log.error(
          "Please try again, or ensure no other process is using the port."
        );
        process.exit(1);
      }
    }
  });

  // Wait for server to be ready
  waitForServer(port)
    .then((actualPort) => {
      log.info(
        `Server is ready on port ${actualPort}, script exiting successfully`
      );
      // Don't exit here - let trunk keep running
    })
    .catch((err) => {
      log.error(`Failed to wait for server: ${err.message}`);
      if (childProcess) {
        childProcess.kill("SIGTERM");
      }
      process.exit(1);
    });
}

/**
 * Cleanup function
 */
function cleanup() {
  if (childProcess) {
    log.info("Cleaning up child process...");
    childProcess.kill("SIGTERM");
  }
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
 * Main entry point
 */
async function main() {
  log.info("Script started");
  log.debug(`Environment variables:`, {
    PORT: process.env.PORT,
    NODE_ENV: process.env.NODE_ENV,
    CI: process.env.CI,
  });

  setupSignalHandlers();

  try {
    const startPort = parseInt(CONFIG.DEFAULT_PORT);
    log.info(`Looking for available port starting from ${startPort}`);

    const port = await findAvailablePort(startPort);
    log.info(`Found available port: ${port}`);

    runTrunk(port);
  } catch (err) {
    log.error(`Failed to find available port: ${err.message}`);
    process.exit(1);
  }
}

// Start the application
main().catch((err) => {
  log.error(`Unexpected error: ${err.message}`);
  process.exit(1);
});
