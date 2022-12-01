#![allow(unused)]

use clap::Parser;
use std::env;
use std::ffi::OsString;
use std::process::Command;

/// Set preconfigured environment variables and run the given command
#[derive(Parser)]
struct Cli {
    /// path to .env file
    #[arg(short, long)]
    env: std::path::PathBuf,

    /// command to run
    command: Vec<OsString>,
}

/// Inject configured environment variables
fn main() {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.env)
        .expect("could not read file");
    
    let cmd = args.command;
    let mut run_cmd = Command::new(&cmd[0]);
    for line in content.lines() {
    // TODO: handle comments, validate env var format
        let split: Vec<&str> = line.split("=").collect();
        let key = split[0].to_string();
        let value = split[1].to_string();

        run_cmd.env(key, value);
    }
    for arg in &cmd[1..] {
        run_cmd.arg(arg);
    }
    run_cmd.spawn();
}
