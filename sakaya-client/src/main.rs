use clap::Parser;
use std::process::Command;

#[derive(Parser)]
#[command(version)]
#[command(arg_required_else_help = true)]
struct Cli {
    container: String,
}

fn main() {
    let cli = Cli::parse();

    let status = Command::new("nixos-container").args(["status", &cli.container]).output().expect("Failed to get nixos-container status");

    if String::from_utf8_lossy(&status.stdout) == "down\n" {
        let _ = Command::new("nixos-container").args(["start", &cli.container]).output().expect("Failed to run nixos-container start");
    } else {
        let _ = Command::new("machinectl").args(["reboot", &cli.container]).output().expect("Failed to run machinectl reboot");
    }

    let mut user: String = "user@".to_owned();
    user.push_str(&cli.container);

    let _ = Command::new("machinectl").args(["shell", user.as_str(), "/usr/bin/env", "sakaya-server"]).output().expect("Failed to start a user shell with sakaya-server");
}
