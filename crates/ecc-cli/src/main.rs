//! The main binary for building and deploying the Encyclopedia of Composable
//! Characteristics (ECC) and associated ontologies.

use clap::Parser;
use clap::Subcommand;

pub mod check;
pub mod ontology;

/// A tool for building and deploy the Encyclopedia of Composable
/// Characteristics (ECC) and associated ontologies.
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

    /// Build and maintain ontologies.
    Ontology(ontology::Args),
}

#[allow(clippy::missing_docs_in_private_items)]
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    match args.command {
        Command::Check(args) => check::main(args),
        Command::Ontology(args) => ontology::main(args),
    }
}
