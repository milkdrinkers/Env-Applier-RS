/*
 * MIT License
 *
 * Copyright (c) 2025 darksaid98
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

mod app;
mod config;
mod env;
#[cfg(test)]
mod tests;
mod utils;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Env-Applier-RS")]
#[command(version = "1.0.0")]
#[command(about = "A simple CLI for replacing environment variables in configuration files.", long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Apply configuration
    Apply {
        #[arg(short, long, value_name = "FILE", help = "Path to config file")]
        config: Option<PathBuf>,
    },
    // Deapply configuration
    Deapply {
        #[arg(short, long, value_name = "FILE", help = "Path to config file")]
        config: Option<PathBuf>,
    },
    // Parse configuration
    Parse {
        #[arg(short, long, value_name = "FILE", help = "Path to config file")]
        config: Option<PathBuf>,
    },
    // List files defined in configuration
    Files {
        #[arg(short, long, value_name = "FILE", help = "Path to config file")]
        config: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    env::load().expect("Failed to load environment files");
    match &cli.command {
        Commands::Apply { config } => handle_apply(config).await,
        Commands::Deapply { config } => handle_deapply(config).await,
        Commands::Parse { config } => handle_parse(config).await,
        Commands::Files { config } => handle_files(config).await,
    }?;

    Ok(())
}

async fn handle_apply(config: &Option<PathBuf>) -> anyhow::Result<()> {
    println!("Applying configuration...");
    let potential_config = if let Some(path) = config {
        println!("Using config file: {}", path.display());
        Some(path.to_path_buf())
    } else {
        println!("Using default configuration");
        None
    };

    let cfg = config::load_config(potential_config).await?;
    let changes = app::apply(&cfg).await?;
    println!("Applied {} changes", changes);

    Ok(())
}

async fn handle_deapply(config: &Option<PathBuf>) -> anyhow::Result<()> {
    println!("Deapplying configuration...");
    let potential_config = if let Some(path) = config {
        println!("Using config file: {}", path.display());
        Some(path.to_path_buf())
    } else {
        println!("Using default configuration");
        None
    };

    let cfg = config::load_config(potential_config).await?;
    let changes = app::deapply(&cfg).await?;
    println!("Deapplied {} changes", changes);

    Ok(())
}

async fn handle_parse(config: &Option<PathBuf>) -> anyhow::Result<()> {
    println!("Parsing configuration...");
    let potential_config = if let Some(path) = config {
        println!("Using config file: {}", path.display());
        Some(path.to_path_buf())
    } else {
        println!("Using default configuration");
        None
    };

    config::load_config(potential_config).await?;
    println!("The config has been validated.");

    Ok(())
}

async fn handle_files(config: &Option<PathBuf>) -> anyhow::Result<()> {
    let potential_config = if let Some(path) = config {
        Some(path.to_path_buf())
    } else {
        None
    };

    let cfg = config::load_config(potential_config).await?;
    match app::get(&cfg).await {
        Ok(files) => {
            if files.is_empty() {
                println!();
            } else {
                let mut output = String::with_capacity(files.len() * 80);
                for file in files {
                    output.push_str(file.as_str());
                    output.push('\n');
                }
                print!("{}", output);
            }
        }
        Err(..) => {
            println!();
        }
    } ;

    Ok(())
}
