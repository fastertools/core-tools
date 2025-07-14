use std::env;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::path::Path;
use std::thread;
use std::time::Duration;

const PID_FILE: &str = "spin.pid";
const STDOUT_LOG: &str = "spin_stdout.log";
const STDERR_LOG: &str = "spin_stderr.log";
const LOG_DIR: &str = ".spin/logs";

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("status");
    let default_port = "3000".to_string();
    let port = args.get(2).unwrap_or(&default_port);

    match command {
        "start" => start_server(port),
        "stop" => stop_server(),
        "restart" => {
            stop_server();
            thread::sleep(Duration::from_secs(2));
            start_server(port);
        }
        "status" => check_status(port),
        "logs" => follow_logs(),
        _ => {
            eprintln!("Usage: cargo run --bin server {{start|stop|restart|status|logs}} [port]");
            eprintln!("  start   - Start the MCP server (default port 3000)");
            eprintln!("  stop    - Stop the MCP server");
            eprintln!("  restart - Restart the MCP server");
            eprintln!("  status  - Check server status");
            eprintln!("  logs    - Follow server logs");
            std::process::exit(1);
        }
    }
}

fn start_server(port: &str) {
    if let Ok(pid) = read_pid() {
        if process_exists(pid) {
            println!("Server already running (PID: {})", pid);
            std::process::exit(1);
        }
        let _ = fs::remove_file(PID_FILE);
    }

    println!("Starting MCP server on port {}...", port);
    
    // Create log files
    let stdout_file = fs::File::create(STDOUT_LOG).expect("Failed to create stdout log");
    let stderr_file = fs::File::create(STDERR_LOG).expect("Failed to create stderr log");

    // Start spin process in background
    let child = Command::new("spin")
        .args(&["up", "--listen", &format!("127.0.0.1:{}", port)])
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .stdin(Stdio::null())
        .spawn()
        .expect("Failed to start spin server");

    // Detach the process by taking its ID and letting it run independently
    let pid = child.id();
    
    // Write PID to file
    write_pid(pid);
    
    println!("Server started (PID: {})", pid);
    println!("Available at: http://127.0.0.1:{}/mcp", port);
    println!("Logs: cargo run --bin server logs");
    
    // Don't wait for the child - let it run in background
    // The process will continue running even after this program exits
}

fn stop_server() {
    match read_pid() {
        Ok(pid) => {
            if process_exists(pid) {
                println!("Stopping server (PID: {})...", pid);
                let _ = Command::new("kill")
                    .arg(pid.to_string())
                    .output();
                let _ = fs::remove_file(PID_FILE);
                println!("Server stopped");
            } else {
                println!("Server not running");
                let _ = fs::remove_file(PID_FILE);
            }
        }
        Err(_) => {
            println!("No PID file found");
        }
    }
}

fn check_status(port: &str) {
    match read_pid() {
        Ok(pid) => {
            if process_exists(pid) {
                println!("Server running (PID: {})", pid);
                println!("Available at: http://127.0.0.1:{}/mcp", port);
                println!("Logs: cargo run --bin server logs");
                std::process::exit(0);
            } else {
                println!("Server not running (stale PID file)");
                let _ = fs::remove_file(PID_FILE);
                std::process::exit(1);
            }
        }
        Err(_) => {
            println!("Server not running");
            std::process::exit(1);
        }
    }
}

fn follow_logs() {
    println!("=== Following Server Logs ===");
    println!("Press Ctrl+C to stop");
    
    // Follow both log files
    let mut cmd = Command::new("tail")
        .args(&["-f", STDOUT_LOG, STDERR_LOG])
        .spawn()
        .expect("Failed to start tail command");
    
    // Also follow component logs if they exist
    if Path::new(LOG_DIR).exists() {
        let _ = Command::new("tail")
            .args(&["-f"])
            .arg(format!("{}/*.log", LOG_DIR))
            .spawn();
    }
    
    let _ = cmd.wait();
}

fn read_pid() -> Result<u32, std::io::Error> {
    let content = fs::read_to_string(PID_FILE)?;
    content.trim().parse().map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid PID")
    })
}

fn write_pid(pid: u32) {
    let mut file = fs::File::create(PID_FILE).expect("Failed to create PID file");
    writeln!(file, "{}", pid).expect("Failed to write PID");
}

fn process_exists(pid: u32) -> bool {
    Command::new("kill")
        .args(&["-0", &pid.to_string()])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}