use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "sak",
    bin_name = "sak",
    version,
    author = "Fabricio Damazio <fabridamazio@gmail.com>",
    about = "Your digital Swiss Army knife for the terminal."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Timer {
        #[command(subcommand)]
        action: TimerAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum TimerAction {
    Start {
        duration: String,    
    },
}
