use std::str::FromStr;
use std::{env, error::Error, net::TcpListener};

use ipnetwork::IpNetwork;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!(
            "Usage: {} -ip <address> -p <starting_port> <ending_port> [-cidr <CIDR>]",
            args[0]
        );
        return Ok(());
    }

    let mut addr = String::new();
    let mut starting_port: usize = 0;
    let mut ending_port: usize = 0;
    let mut cidr: Option<String> = None; // Optional CIDR

    // Parse arguments
    for arg in args.iter().enumerate() {
        match arg {
            (i, val) if val == "-ip" => addr = args[i + 1].clone(),
            (_i, val) if val == "-l" => addr = "127.0.0.1".to_string(),
            (i, val) if val == "-p" => {
                starting_port = args[i + 1].parse()?;
                ending_port = args[i + 2].parse()?;
            }
            (i, val) if val == "-cidr" => cidr = Some(args[i + 1].clone()),
            _ => {}
        }
    }

    // Handle the case where both IP and CIDR are provided (conflict)
    if let Some(_cidr) = cidr.clone() {
        if !addr.is_empty() {
            println!("Can't process when both IP and CIDR are given.");
            return Ok(());
        }
    }

    // Validate required arguments
    if starting_port == 0 || ending_port == 0 {
        println!("Missing required arguments.");
        return Ok(());
    }

    // If CIDR is provided, get all the IPs in that range and scan ports
    if let Some(cidr_value) = cidr {
        let addresses = get_ips(&cidr_value)?;
        for addr in addresses.iter() {
            for port in starting_port..=ending_port {
                match try_listen(&addr.to_string(), port) {
                    Ok(port) => println!("Port {} is available on {}", port, addr),
                    Err(msg) => println!("Port {}: {}", port, msg),
                }
            }
        }
    }

    // If CIDR is not provided, scan the given single IP address
    for port in starting_port..=ending_port {
        match try_listen(&addr, port) {
            Ok(port) => println!("Port {} is available on {}", port, addr),
            Err(msg) => println!("Port {}: {}", port, msg),
        }
    }

    Ok(())
}

fn try_listen(address: &str, port: usize) -> Result<usize, String> {
    let full_address = format!("{}:{}", address, port);
    match TcpListener::bind(&full_address) {
        Ok(_listener) => Ok(port),
        Err(_) => Err("Port is busy".to_string()),
    }
}

fn get_ips(targets: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut ips = Vec::new();

    if let Ok(network) = IpNetwork::from_str(targets) {
        for ip in network.iter() {
            ips.push(ip.to_string());
        }
    } else {
        return Err("Invalid IP address or CIDR format".into());
    }

    Ok(ips)
}
