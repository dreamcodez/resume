const { spawn } = require("child_process");
const path = require("path");

describe("Dynamic Port Script", () => {
  test("should use random ports for different instances", async () => {
    const scriptPath = path.join(__dirname, "..", "trunk-serve-dynamic.js");

    // Run the script multiple times to get different ports
    const ports = [];
    const promises = [];

    for (let i = 0; i < 3; i++) {
      const promise = new Promise((resolve) => {
        const child = spawn("node", [scriptPath], {
          cwd: path.join(__dirname, ".."),
          env: { ...process.env },
        });

        let output = "";
        child.stdout.on("data", (data) => {
          output += data.toString();
        });

        child.stderr.on("data", (data) => {
          output += data.toString();
        });

        // Kill after a short delay to get the port
        setTimeout(() => {
          child.kill("SIGTERM");
          resolve(output);
        }, 1000);
      });

      promises.push(promise);
    }

    const outputs = await Promise.all(promises);

    // Extract ports from outputs
    for (const output of outputs) {
      const match = output.match(/TRUNK_PORT=(\d+)/);
      if (match) {
        ports.push(parseInt(match[1]));
      }
    }

    // Verify we got ports
    expect(ports.length).toBeGreaterThan(0);

    // Verify ports are in the expected range (3000-9000)
    for (const port of ports) {
      expect(port).toBeGreaterThanOrEqual(3000);
      expect(port).toBeLessThanOrEqual(9000);
    }

    // Verify we got different ports (with some tolerance for edge cases)
    const uniquePorts = new Set(ports);
    expect(uniquePorts.size).toBeGreaterThan(1);

    console.log("Ports used:", ports);
  });

  test("should find available port when starting port is busy", async () => {
    const scriptPath = path.join(__dirname, "..", "trunk-serve-dynamic.js");

    // Start a server on a known port to make it busy
    const http = require("http");
    const testPort = 3456;
    const server = http.createServer();

    await new Promise((resolve) => {
      server.listen(testPort, resolve);
    });

    try {
      // Run the script - it should find a different port
      const child = spawn("node", [scriptPath], {
        cwd: path.join(__dirname, ".."),
        env: { ...process.env },
      });

      let output = "";
      child.stdout.on("data", (data) => {
        output += data.toString();
      });

      child.stderr.on("data", (data) => {
        output += data.toString();
      });

      // Kill after a short delay to get the port
      await new Promise((resolve) => {
        setTimeout(() => {
          child.kill("SIGTERM");
          resolve();
        }, 1000);
      });

      // Extract port from output
      const match = output.match(/TRUNK_PORT=(\d+)/);
      expect(match).toBeTruthy();

      const port = parseInt(match[1]);
      expect(port).not.toBe(testPort); // Should not use the busy port
      expect(port).toBeGreaterThanOrEqual(3000);
      expect(port).toBeLessThanOrEqual(9000);
    } finally {
      server.close();
    }
  });
});
