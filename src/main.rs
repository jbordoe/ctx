#![allow(unused)]

use clap::Parser;
use dotenv_parser::parse_dotenv;
use std::env;
use std::ffi::OsString;
use std::process::Command;

/// Set preconfigured environment variables and run the given command
#[derive(Parser)]
struct Cli {
    /// path to .env file
    #[arg(short, long)]
    file: Option<std::path::PathBuf>,

    /// set one or more environment variables
    #[arg(short, long, value_name = "VAR=VALUE")]
    env: Vec<String>,
    
    /// command to run
    command: Vec<OsString>,
}

/// Inject configured environment variables
fn main() {
    let args = Cli::parse();
    let cmd = args.command;
    let mut run_cmd = Command::new(&cmd[0]);

    if let Some(file_path) = args.file {
        let content = std::fs::read_to_string(&file_path)
            .expect("could not read file");

        for (key, value) in parse_dotenv(&content).unwrap() {
            run_cmd.env(key, value);
        }
    }

    for env_str in args.env {
        let parts: Vec<&str> = env_str.split('=').collect();
        if parts.len() != 2 {
            println!("Invalid environment variable mapping: {}", env_str);
            continue;
        }
        let var = parts[0];
        let value = parts[1];
        run_cmd.env(var, value);
        println!("Set environment variable {} to {}", var, value);
    }

    for arg in &cmd[1..] {
        run_cmd.arg(arg);
    }
    run_cmd.spawn()
        .expect("failed to execute command")
        .wait();
}
