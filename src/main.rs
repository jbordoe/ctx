#![allow(unused)]

use clap::Parser;
use colored::*;
use dotenv_parser::parse_dotenv;
use serde::{Deserialize, Serialize};
use serde_yaml;

use std::{env, ffi::OsString, fs::File, io::Read, process::Command, path::PathBuf};

/// Set preconfigured environment variables and run the given command
#[derive(Parser)]
struct Cli {
    /// context from local .ctx.yml
    #[arg(short, long)]
    context: Option<String>,

    /// path to .env file
    #[arg(short, long)]
    file: Option<std::path::PathBuf>,

    /// set one or more environment variables
    #[arg(short, long, value_name = "VAR=VALUE")]
    env: Vec<String>,
    
    /// command to run
    command: Vec<OsString>,

    /// Display env vars being set
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Contexts {
    #[serde(rename = "contexts")]
    contexts: std::collections::HashMap<String, Context>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Context {
    variables: Vec<Variable>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Variable {
    name: String,
    value: String,
}

fn get_context_variables(context: &str) -> Option<Vec<Variable>> {
    let ctx_file = PathBuf::from(".ctx.yml");
    let mut file = match File::open(&ctx_file) {
        Ok(f) => f,
        Err(_) => return None,
    };

    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        return None;
    }

    let contexts: Contexts = match serde_yaml::from_str(&contents) {
        Ok(c) => c,
        Err(_) => return None,
    };

    match contexts.contexts.get(context) {
        Some(ctx) => Some(ctx.variables.clone()),
        None => None,
    }
}


/// Inject configured environment variables
fn main() {
    let args = Cli::parse();
    let cmd = args.command;
    let mut run_cmd = Command::new(&cmd[0]);

    if let Some(context) = args.context {
        if args.verbose {
            eprintln!(
                "Reading from context: {}",
                context.yellow()
            );
        }
        let context_variables = get_context_variables(&context);
        if let Some(variables) = context_variables {
            for v in variables.into_iter() {
                let key = v.name;
                let value = v.value;
                if args.verbose {
                    eprintln!(
                        "{}={}",
                        key.bright_yellow().bold(),
                        value.yellow()
                    );
                }
                run_cmd.env(key, value);
            }
        }
    }

    if let Some(file_path) = args.file {
        let content = std::fs::read_to_string(&file_path)
            .expect("could not read file");

        for (key, value) in parse_dotenv(&content).unwrap() {
            if args.verbose {
                eprintln!(
                    "{}={}",
                    key.bright_yellow().bold(),
                    value.yellow()
                );
            }
            run_cmd.env(key, value);
        }
    }

    for env_str in args.env {
        let parts: Vec<&str> = env_str.split('=').collect();
        if parts.len() != 2 {
            eprintln!(
                "{} {}",
                "Invalid environment variable mapping:".bright_red().bold(),
                env_str.red()
            );
            continue;
        }
        let var = parts[0];
        let value = parts[1];
        if args.verbose {
            eprintln!(
                "{}={}",
                var.bright_yellow().bold(),
                value.yellow()
            );
        }

        run_cmd.env(var, value);
    }

    for arg in &cmd[1..] {
        run_cmd.arg(arg);
    }
    let command_str = cmd 
        .iter()
        .map(|os_str| os_str.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ");

    if args.verbose {
        eprintln!(
            "{} {}",
            "Running command:".bright_purple().bold(),
            command_str.purple()
            );
    }

    run_cmd.spawn()
        .expect("failed to execute command")
        .wait();

}
