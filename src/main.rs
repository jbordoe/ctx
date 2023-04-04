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
    for (key, value) in parse_dotenv(&content).unwrap() {
        run_cmd.env(key, value);
    }
    for arg in &cmd[1..] {
        run_cmd.arg(arg);
    }
    run_cmd.spawn()
        .expect("failed to execute command")
        .wait();
}
