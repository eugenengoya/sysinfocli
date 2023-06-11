use std::{env, process};
// use gethostname::gethostname;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            // "--name" => display_computer_name(),
            "--os" => display_operating_system(),
            "--ip" => display_ip_address(),
            "--help" => display_help(),
            _ => {
                println!("Invalid option. Use --help to see available options.");
                process::exit(1);
            }
        }
    } else {
        display_help();
    }
}

// fn display_computer_name() {
//     let hostname = gethostname().unwrap_or_else(|_| "Unknown".to_string());
//     println!("Computer name: {}", hostname);
// }

fn display_operating_system() {
    let output = Command::new("uname").arg("-a").output().expect("Failed to execute command");
    let os_info = String::from_utf8_lossy(&output.stdout);
    println!("Operating System: {}", os_info);
}

fn display_ip_address() {
    let output = Command::new("hostname").arg("-I").output().expect("Failed to execute command");
    let ip_info = String::from_utf8_lossy(&output.stdout);
    println!("IP Address: {}", ip_info);
}

fn display_help() {
    println!("Usage: program [OPTION]");
    println!("Options:");
    // println!("  --name       Display the computer name");
    println!("  --os         Display the operating system");
    println!("  --ip         Display the IP address");
    println!("  --help       Display this help message");
}
