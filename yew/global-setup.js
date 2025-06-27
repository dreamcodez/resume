const { spawn } = require("child_process");
const path = require("path");
const http = require("http");

function findPort(startPort = 3000) {
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

module.exports = async (config) => {
  // Find a free port
  const port = await findPort(Math.floor(Math.random() * 6000) + 3000);
  process.env.TRUNK_PORT = String(port);

  // Start the dynamic trunk server with the port as an argument
  const child = spawn(
    "node",
    [path.join(__dirname, "trunk-serve-dynamic.js"), String(port)],
    {
      cwd: __dirname,
      env: { ...process.env, TRUNK_PORT: String(port) },
      stdio: "inherit", // Pipe output to parent's stdio
    }
  );

  // Wait for the server to start by checking if the port is listening
  await new Promise((resolve, reject) => {
    let attempts = 0;
    const maxAttempts = 300; // 30 seconds with 100ms intervals

    const checkPort = () => {
      attempts++;
      const testServer = http.createServer();
      testServer.listen(port, () => {
        testServer.close(() => {
          console.log(`✅ Server ready on port ${port}`);
          resolve();
        });
      });
      testServer.on("error", () => {
        if (attempts >= maxAttempts) {
          reject(
            new Error(
              `Server failed to start on port ${port} within 30 seconds`
            )
          );
        } else {
          // Port not ready yet, try again in 100ms
          setTimeout(checkPort, 100);
        }
      });
    };

    // Start checking after a short delay to let the server start
    setTimeout(checkPort, 1000);

    child.on("exit", (code) => {
      reject(
        new Error(`Trunk server exited with code ${code} before port was ready`)
      );
    });
  });

  // The OS will handle cleanup when Playwright exits
};
