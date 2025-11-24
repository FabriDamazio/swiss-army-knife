mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli  = Cli::parse();

    match cli.command {
        Command::Timer { action } => { 
            commands::timer::handle(action);
        },
    }
}
