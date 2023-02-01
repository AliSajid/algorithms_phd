mod cli;
mod helpers;

use clap::Parser;
use cli::Cli;
use cli::Commands;

fn main() {
    let cli = Cli::parse();

    let solutions = exercises::solutions();

    match cli.command {
        Some(Commands::Test { exercise }) => {
            println!("Test command with Exercise {}", exercise);
            let solution = solutions.get(&exercise).unwrap();
            let input = solution.read_input(exercise);
            let output = solution.solve(input);
            println!("Output: {}", output);
        }
        Some(Commands::Solve { exercise }) => {
            println!("Solve command with Exercise {}", exercise);
            let solution = solutions.get(&exercise).unwrap();
            let input = solution.read_input(exercise);
            let output = solution.solve(input);
            println!("Output: {}", output);
        }
        None => {
            println!("No command provided");
        }
    }
}
