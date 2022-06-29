use clap::{Parser, Subcommand};
use std::{path::Path, process::ExitCode};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Stem { path: String },
    IsAbsolute { path: String },
    Parent { path: String },
    Filename { path: String },
    WithSuffix { path: String, suffix: String },
}

fn as_path<'a>(s: &'a str) -> &'a Path {
    Path::new(s)
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Stem { path } => {
            if let Some(stem) = as_path(path).file_stem() {
                println!("{}", stem.to_string_lossy());
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Commands::IsAbsolute { path } => {
            if as_path(path).is_absolute() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Commands::Parent { path } => {
            if let Some(answer) = as_path(path).parent() {
                println!("{}", answer.to_string_lossy());
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Commands::Filename { path } => {
            if let Some(answer) = as_path(path).file_name() {
                println!("{}", answer.to_string_lossy());
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Commands::WithSuffix { path, suffix } => {
            let p = as_path(path).with_extension(suffix);
            println!("{}", p.to_string_lossy());
            ExitCode::SUCCESS
        }
    }
}
