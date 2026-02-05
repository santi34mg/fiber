use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{fs, path::PathBuf, process};

use crate::driver;

#[derive(Debug, Deserialize)]
pub struct CompilerConfig {
    pub main_module_path: PathBuf,
    pub is_debug: bool,
}

#[derive(Parser, Debug)]
#[command(name = "fiber", about = "Run Fiber programs")]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(short, long, default_value = DEFAULT_CONFIG_PATH)]
    config_path: String,

    #[arg(short, long, default_value_t = false)]
    is_debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Build { file: Option<PathBuf> },
    Init { dir: PathBuf },
}

const DEFAULT_CONFIG_PATH: &str = "fiber.toml";

pub fn load_config(args: &Args) -> Option<CompilerConfig> {
    let mut config_path = DEFAULT_CONFIG_PATH;
    match args.config_path.as_str() {
        ref path if *path != DEFAULT_CONFIG_PATH => {
            config_path = path;
        }
        _ => {}
    }
    let s = std::fs::read_to_string(config_path).ok()?;
    toml::from_str(&s).ok()
}

pub fn parse_args() -> Args {
    Args::parse()
}

pub fn exec_command(args: Args) {
    match &args.command {
        Some(Command::Init { dir }) => {
            init_command(dir);
        }
        Some(Command::Build { file }) => {
            let config = match load_config(&args) {
                Some(c) => c,
                None => {
                    eprintln!("Failed to load configuration.");
                    std::process::exit(1);
                }
            };
            let file = match file {
                Some(f) => f,
                None => &config.main_module_path,
            };
            build_command(file, &config);
        }
        None => {
            print_usage();
            process::exit(1);
        }
    }
}

fn init_command(dir: &PathBuf) {
    fs::create_dir_all(&dir).unwrap_or_else(|e| {
        eprintln!("Error creating project directory: {e}");
        process::exit(1);
    });
    let config = "main_module_path = \"main.fib\"\nis_debug = true";
    fs::write(dir.join("fiber.toml"), config).unwrap();
    fs::write(dir.join("main.fib"), "// Fiber entry point\n").unwrap();
    println!("Initialized new Fiber project at {}", dir.display());
}

fn build_command(file: &PathBuf, config: &CompilerConfig) {
    driver::run_pipeline(file, config.is_debug);
}

fn print_usage() {
    eprintln!("Usage: fiber <command>\nCommands: run, init");
}
