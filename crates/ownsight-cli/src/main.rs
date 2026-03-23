mod commands;
mod output;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "cargo-ownership-viz")]
#[command(about = "Visualize Rust ownership and borrowing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "ownership-viz")]
    OwnershipViz {
        #[arg(short, long, help = "Input file to analyze")]
        file: Option<String>,
        
        #[arg(short, long, help = "Analyze code from stdin")]
        stdin: bool,
        
        #[arg(short, long, help = "Output format: text, json, timeline")]
        output: Option<String>,
        
        #[arg(short, long, help = "Teaching mode (simplified) or debug mode (precise)")]
        mode: Option<String>,
        
        #[arg(short = 'b', long, help = "Analysis backend: simple (syntax-based) or mir (compiler-based)")]
        backend: Option<String>,
        
        #[arg(long, help = "Specific function to analyze")]
        function: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::OwnershipViz { file, stdin, output, mode, backend, function } => {
            commands::analyze::run(file, stdin, output, mode, backend, function)?;
        }
    }
    
    Ok(())
}
