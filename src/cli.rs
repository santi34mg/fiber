use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{fs, path::PathBuf, process};

#[derive(Debug, Deserialize)]
struct Config {
    entry_point: String,
    entry_function: Option<String>,
}

const DEFAULT_CONFIG_PATH: &str = "fiber.toml";

fn load_config() -> Option<Config> {
    let s = std::fs::read_to_string(DEFAULT_CONFIG_PATH).ok()?;
    toml::from_str(&s).ok()
}

#[derive(Parser, Debug)]
#[command(name = "fiber", about = "Run Fiber programs")]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Run {
        file: Option<PathBuf>,
    },
    Init {
        dir: PathBuf,
    }
}

pub fn parse_args() -> (PathBuf, Option<String>) {
    let args = Args::parse();

    match args.command {
        Some(Command::Init { dir }) => {
            fs::create_dir_all(&dir).unwrap_or_else(|e| {
                eprintln!("Error creating project directory: {e}");
                process::exit(1);
            });
            let config = format!(
                "entry_point = \"{}/main.fbr\"\nentry_function = \"main\"",
                dir.to_string_lossy()
            );
            fs::write(dir.join("fiber.toml"), config).unwrap();
            fs::write(dir.join("main.fbr"), "// Fiber entry point\n").unwrap();
            println!("Initialized new Fiber project at {}", dir.display());
            process::exit(0);
        }
        Some(Command::Run { file }) => {
            if let Some(f) = file {
                (f, None)
            } else if let Some(cfg) = load_config() {
                (PathBuf::from(cfg.entry_point), cfg.entry_function)
            } else {
                eprintln!("Usage: fiber run <source-file>\nOr create a fiber.toml with an 'entry_point' key");
                process::exit(2);
            }
        }
        None => {
            eprintln!("Usage: fiber <command>\nCommands: run, init");
            process::exit(2);
        }
    }
}
