//! Common operations in the integration tests.
#![allow(tail_expr_drop_order)]

use std::path::PathBuf;

use anyhow::Context as _;
use anyhow::anyhow;
use glob::glob;
use serde::Deserialize;

/// Reads an ontology node fixture within the integration tests.
pub fn read_fixture<D>(fixture: &str) -> anyhow::Result<Vec<D>>
where
    D: for<'de> Deserialize<'de>,
{
    // NOTE: this is the root of the crate itself, not the workspace.
    let mut path = std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .expect("crate root to be available at compile time");
    path.push("tests");
    path.push("fixtures");
    path.push(fixture);

    let search_path = format!("{}.*", path.display());
    let mut files = glob(&search_path)
        .with_context(|| format!("searching for files with glob `{search_path}`"))?
        .collect::<Result<Vec<_>, _>>()
        .context("reading matching files")?;

    let path = match files.len() {
        0 => return Err(anyhow!("no files matched the pattern `{search_path}`")),
        1 => files.pop().unwrap(),
        v => {
            return Err(anyhow!(
                "expected one file matching pattern `{search_path}`, found {v} files"
            ));
        }
    };

    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("reading fixture at path: {}", path.display()))?;

    // SAFETY: the glob above _requires_ that the path have an extension, so
    // this will always unwrap.
    let ext = path
        .extension()
        .map(|ext| ext.to_string_lossy())
        .unwrap()
        .to_string();

    match ext.as_str() {
        "yaml" | "yml" => {
            serde_yaml::from_str(&contents).context("deserializing identifier from YAML")
        }
        v => Err(anyhow!("unhandled fixture extension: {v}")),
    }
}
