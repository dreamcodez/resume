#!/usr/bin/env node
/**
 * Simple Dynamic Port Script
 *
 * Starts Trunk on a specified port (via argument or env), or finds a random port if not provided.
 */

const { spawn } = require("child_process");
const http = require("http");

function parsePortArg() {
  // Try command-line argument first
  const argPort = process.argv[2];
  if (argPort && !isNaN(argPort)) return parseInt(argPort, 10);
  // Then environment variable
  if (process.env.TRUNK_PORT && !isNaN(process.env.TRUNK_PORT))
    return parseInt(process.env.TRUNK_PORT, 10);
  return null;
}

// Find an available port, starting from a random port
function findPort() {
  const startPort = Math.floor(Math.random() * 6000) + 3000;
  return findPortFrom(startPort);
}

function findPortFrom(startPort) {
  return new Promise((resolve) => {
    const server = http.createServer();
    server.listen(startPort, () => {
      const port = server.address().port;
      server.close(() => resolve(port));
    });
    server.on("error", () => {
      findPortFrom(startPort + 1).then(resolve);
    });
  });
}

(async () => {
  let port = parsePortArg();
  if (!port) {
    port = await findPort();
  }
  process.env.TRUNK_PORT = port.toString();
  console.log(`TRUNK_PORT=${port}`);

  const trunk = spawn("trunk", ["serve", "--port", port.toString()], {
    stdio: "inherit",
    env: { ...process.env },
  });

  trunk.on("exit", (code) => process.exit(code));
  process.on("SIGINT", () => trunk.kill("SIGINT"));
  process.on("SIGTERM", () => trunk.kill("SIGTERM"));
})();
