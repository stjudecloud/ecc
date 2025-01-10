//! Checking of a composable characteristic tree.

use std::path::PathBuf;

use clap::Parser;
use colored::Colorize as _;
use ecc::Characteristic;
use tracing::info;

/// The canonical locations where the composable characteristics might live.
const CANONICAL_DIRS: &[&str] = &[
    "../ecc", // If we are executing the tool from the crates directory.
];

/// Checks that a composable characteristic tree is valid.
#[derive(Parser)]
pub struct Args {
    /// The path to the composable characteristic directory.
    path: Option<PathBuf>,
}

/// The main method.
pub fn main(args: Args) -> anyhow::Result<()> {
    let root = args
        .path
        .or_else(|| {
            // SAFETY: in all the contexts we care about, this should be available.
            let cwd = std::env::current_dir().expect("cwd to be available");

            for path in CANONICAL_DIRS {
                let path = cwd.join(path);

                if path.is_dir() {
                    info!("hooked characteristic tree: {}", path.display());
                    return Some(path);
                }
            }

            None
        })
        .map(|path| path.canonicalize().expect("path to canonicalize"))
        .unwrap_or_else(|| panic!("could not find composable characteristic directory"));

    let paths = format!("{}/**/*.yml", root.display());
    info!("characteristic glob: `{paths}`");

    for result in glob::glob(&paths).expect("glob to resolve") {
        let ecc = result.expect("file path to resolve");
        print!("{}.. ", ecc.display().to_string().bold());

        let contents = std::fs::read_to_string(ecc).expect("file to be read");
        match serde_yaml::from_str::<Characteristic>(&contents) {
            Ok(_) => println!("{}", "OK".green()),
            Err(err) => {
                println!("{}\n", "FAIL".red());
                println!("-------\nDetails\n-------\n\n{}\n\n", err);
            }
        }
    }

    Ok(())
}
