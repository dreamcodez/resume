#!/usr/bin/env node
/**
 * Simple Dynamic Port Script
 *
 * Finds an available port and starts Trunk on it.
 */

const { spawn } = require("child_process");
const http = require("http");

// Find an available port
function findPort(startPort = 8080) {
  return new Promise((resolve) => {
    const server = http.createServer();
    server.listen(startPort, () => {
      const port = server.address().port;
      server.close(() => resolve(port));
    });
    server.on("error", () => {
      findPort(startPort + 1).then(resolve);
    });
  });
}

// Main execution
(async () => {
  const port = await findPort();
  console.log(`TRUNK_PORT=${port}`);
  process.env.TRUNK_PORT = port.toString();

  const trunk = spawn("trunk", ["serve", "--port", port.toString()], {
    stdio: "inherit",
    env: { ...process.env },
  });

  trunk.on("exit", (code) => process.exit(code));
  process.on("SIGINT", () => trunk.kill("SIGINT"));
  process.on("SIGTERM", () => trunk.kill("SIGTERM"));
})();
