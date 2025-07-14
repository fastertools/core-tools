use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 && args[1] == "stop" {
        stop_server();
        return;
    }
    
    println!("Starting Spin server for testing...");
    
    // Create log files
    let stdout_file = File::create("spin_stdout.log").expect("Failed to create stdout log");
    let stderr_file = File::create("spin_stderr.log").expect("Failed to create stderr log");
    
    // Start the Spin server with logging
    let spin_process = Command::new("spin")
        .args(&["up", "--listen", "127.0.0.1:3000"])
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .spawn()
        .expect("Failed to start Spin server");
    
    let pid = spin_process.id();
    
    // Save PID to file for later stopping
    let mut pid_file = File::create("spin.pid").expect("Failed to create PID file");
    writeln!(pid_file, "{}", pid).expect("Failed to write PID");
    
    println!("Spin server started with PID: {}", pid);
    println!("Logs are being written to:");
    println!("  - spin_stdout.log");
    println!("  - spin_stderr.log");
    
    // Give the server time to start
    println!("Waiting for server to start...");
    thread::sleep(Duration::from_secs(3));
    
    println!("\nServer should be running on http://127.0.0.1:3000");
    println!("To stop the server, run: ./test_server stop");
    println!("To view logs, run: tail -f spin_*.log");
}

fn stop_server() {
    // Read PID from file
    match std::fs::read_to_string("spin.pid") {
        Ok(pid_str) => {
            if let Ok(pid) = pid_str.trim().parse::<u32>() {
                println!("Stopping Spin server with PID: {}", pid);
                
                Command::new("kill")
                    .arg(pid.to_string())
                    .output()
                    .expect("Failed to kill process");
                
                // Clean up PID file
                let _ = std::fs::remove_file("spin.pid");
                println!("Server stopped");
            } else {
                println!("Invalid PID in spin.pid file");
            }
        }
        Err(_) => {
            println!("No spin.pid file found. Server may not be running.");
        }
    }
}