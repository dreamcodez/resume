// Debug utilities for CDP and dev server testing
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[tokio::test]
async fn debug_dev_server() {
    println!("=== DEBUG DEV SERVER ===");

    // Test dev server startup
    println!("1. Testing dev server startup...");
    let port = portpicker::pick_unused_port().expect("No free port found");
    let _url = format!("http://localhost:{}", port);

    println!("2. Spawning trunk serve on port {}...", port);
    let mut child = Command::new("trunk")
        .args(&["serve", "--port", &port.to_string()])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start dev server");

    println!("3. Dev server PID: {}", child.id());

    // Wait a bit and check if it's still running
    thread::sleep(Duration::from_secs(5));

    match child.try_wait() {
        Ok(Some(status)) => println!("✗ Dev server exited with status: {:?}", status),
        Ok(None) => println!("✓ Dev server still running"),
        Err(e) => println!("✗ Error checking dev server: {}", e),
    }

    // Cleanup
    let _ = child.kill();
    println!("=== DEV SERVER DEBUG COMPLETE ===");
}
