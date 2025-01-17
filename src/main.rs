use std::process::Command;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!(
            "Usage:\n\
            {} -p <port>             (Check if a port is available)\n\
            {} -p <start> <end>      (Check if a range of ports is available)\n\
            {} -n <port>             (Check if a port is available, free it if not)\n\
            {} -n <start> <end>      (Check if a range of ports is available, free them if not)",
            args[0], args[0], args[0], args[0]
        );
        return Ok(());
    }

    let command = args[1].as_str();
    let start_port: usize = args.get(2).and_then(|p| p.parse().ok()).unwrap_or(0);
    let end_port: usize = args
        .get(3)
        .and_then(|p| p.parse().ok())
        .unwrap_or(start_port);

    if start_port == 0 || (end_port != start_port && end_port < start_port) {
        println!("Invalid or missing port range.");
        return Ok(());
    }

    match command {
        "-p" => {
            for port in start_port..=end_port {
                match try_listen(port) {
                    Ok(_) => println!("Port {} is available.", port),
                    Err(msg) => println!("Port {} is not available: {}", port, msg),
                }
            }
        }
        "-n" => {
            for port in start_port..=end_port {
                match try_listen(port) {
                    Ok(_) => println!("Port {} is available.", port),
                    Err(_) => {
                        println!("Port {} is busy. Attempting to make it available...", port);
                        match kill_process_on_port(port) {
                            Ok(_) => println!("Successfully freed port {}.", port),
                            Err(msg) => println!("Failed to free port {}: {}", port, msg),
                        }
                    }
                }
            }
        }
        _ => println!("Invalid command. Use -p or -n."),
    }

    Ok(())
}

fn try_listen(port: usize) -> Result<(), String> {
    let check_command = format!("lsof -t -i:{}", port);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&check_command)
        .output()
        .map_err(|_| "Failed to execute lsof command".to_string())?;

    if !output.stdout.is_empty() {
        return Err(format!("Port {} is in use", port));
    }
    Ok(())
}

fn kill_process_on_port(port: usize) -> Result<(), String> {
    let pid_command = format!("lsof -t -i:{}", port);
    let pid_output = Command::new("sh")
        .arg("-c")
        .arg(&pid_command)
        .output()
        .map_err(|_| "Failed to execute PID command".to_string())?;

    let pid = String::from_utf8(pid_output.stdout)
        .map_err(|_| "Failed to parse PID output".to_string())?
        .trim()
        .to_string();

    if pid.is_empty() {
        return Err("No process found on the port".to_string());
    }

    println!("Found process with PID: {} on port {}", pid, port);

    let kill_command = format!("kill -9 {}", pid);
    Command::new("sh")
        .arg("-c")
        .arg(&kill_command)
        .output()
        .map_err(|_| "Failed to execute kill command".to_string())?;

    println!("Killed process with PID: {}", pid);
    Ok(())
}
