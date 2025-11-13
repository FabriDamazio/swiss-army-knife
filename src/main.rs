mod cli;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli  = Cli::parse();

    match cli.command {
        Command::Timer {} => { println!("It works!") },
    }
}
