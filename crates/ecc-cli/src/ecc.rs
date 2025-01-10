//! Composable characteristic-related facilities.

use clap::Parser;
use clap::Subcommand;

mod check;

/// Build and maintain the composable characteristics tree.
#[derive(Parser)]
pub struct Args {
    /// The command to run.
    #[command(subcommand)]
    command: Command,
}

/// The command to run.
#[derive(Subcommand)]
pub enum Command {
    /// Checks the composable characteristic tree is valid.
    Check(check::Args),
}

/// The main method.
pub fn main(args: Args) -> anyhow::Result<()> {
    match args.command {
        Command::Check(args) => check::main(args),
    }
}
