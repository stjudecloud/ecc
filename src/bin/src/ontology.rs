//! Ontology-related facilities.

mod init;

use clap::Parser;
use clap::Subcommand;

/// Build and maintain ontologies related to the ECC.
#[derive(Parser)]
pub struct Args {
    /// The command to run.
    #[command(subcommand)]
    command: Command,
}

/// The command to run.
#[derive(Subcommand)]
pub enum Command {
    /// Initializes an ontology directory from an existing map.
    Init(init::Args),
}

/// The main method.
pub fn main(args: Args) -> anyhow::Result<()> {
    match args.command {
        Command::Init(args) => init::main(args),
    }
}
