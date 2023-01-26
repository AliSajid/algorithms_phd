mod cli;
mod helpers;

use clap::Parser;
use cli::Cli;
use cli::Commands;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Test { exercise }) => {
            println!("Test command with Exercise {}", exercise);
        }
        Some(Commands::Solve { exercise }) => {
            println!("Solve command with  Exercise {}", exercise);
        }
        None => {
            println!("No command provided");
        }
    }
}
