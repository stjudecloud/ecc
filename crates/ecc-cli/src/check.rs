//! Checking of a composable characteristic tree.

use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::diagnostic::Label;
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::ColorChoice;
use codespan_reporting::term::termcolor::StandardStream;
use colored::Colorize as _;
use ecc::Characteristic;
use tracing::info;

/// Checks that a composable characteristic tree is valid.
#[derive(Parser)]
pub struct Args {
    /// The path to the composable characteristic directory.
    path: PathBuf,
}

/// The main method.
pub fn main(args: Args) -> anyhow::Result<()> {
    let paths = format!("{}/**/*.yml", args.path.display());
    info!("characteristic glob: `{paths}`");

    let mut stdout = std::io::stdout();
    let mut failed = false;

    for result in glob::glob(&paths).expect("glob to resolve") {
        let ecc_file = result.expect("file path to resolve");
        print!("{}.. ", ecc_file.display().to_string().bold());

        let contents = std::fs::read_to_string(&ecc_file).expect("file to be read");

        match serde_yaml::from_str::<Characteristic>(&contents) {
            Ok(_) => {
                println!("{}", "OK".green());
                stdout.flush().unwrap();
            }
            Err(err) => {
                failed = true;

                println!("{}\n", "FAIL".red());
                stdout.flush().unwrap();

                let file = SimpleFile::new(ecc_file.display().to_string(), contents.clone());

                let index = match err.location() {
                    Some(location) => location.index(),
                    None => contents.len(),
                };

                let diagnostic = Diagnostic::error().with_labels(vec![
                    Label::primary((), index..index).with_message(err.to_string()),
                ]);

                let writer = StandardStream::stdout(ColorChoice::Always);

                let config = term::Config {
                    ..Default::default()
                };

                term::emit(&mut writer.lock(), &config, &file, &diagnostic)?;
            }
        }
    }

    if failed {
        std::process::exit(1);
    }

    Ok(())
}
