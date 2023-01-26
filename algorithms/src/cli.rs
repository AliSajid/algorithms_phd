use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap()]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Test {
        #[clap(value_name = "EXERCISE")]
        exercise: String,
    },
    Solve {
        #[clap(value_name = "EXERCISE")]
        exercise: String,
    },
}
