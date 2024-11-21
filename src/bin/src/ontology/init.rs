//! Initialization of an ontology directory.

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::anyhow;
use anyhow::bail;
use clap::Parser;
use ontology::Node;
use petgraph::graph::DiGraph;

pub mod directory;

use directory::Directory;

/// Initializes a directory from a pre-existing ontology mapping.
#[derive(Parser)]
pub struct Args {
    /// The tab-separated value file containing the existing ontology.
    ///
    /// See the example file or the definition for nodes within the ontology
    /// package to learn about the required columns and their individual
    /// requirements.
    tsv: PathBuf,

    /// The directory to output the ontology files.
    #[clap(short)]
    output_directory: PathBuf,
}

/// The main method.
pub fn main(args: Args) -> anyhow::Result<()> {
    let mut reader = File::open(&args.tsv)
        .with_context(|| format!("opening file: {}", args.tsv.display()))
        .map(BufReader::new)
        .map(|reader| {
            csv::ReaderBuilder::new()
                .delimiter(b'\t')
                .from_reader(reader)
        })?;

    let mut nodes = Vec::new();
    let mut indexes = HashMap::new();

    for result in reader.deserialize() {
        let node: Node = result?;
        nodes.push(node)
    }

    let mut graph = DiGraph::new();

    for node in &nodes {
        let name = node.name().inner().to_string();

        if indexes.contains_key(&name) {
            bail!("attempted to insert node twice: {name}");
        }

        let index = graph.add_node(node.clone());
        indexes.insert(name, index);
    }

    let mut root = None;

    for node in nodes {
        let name = node.name().inner().to_string();
        let node_index = indexes
            .get(&name)
            .cloned()
            .ok_or(anyhow!("specified node does not exist: {name}"))?;

        let parent = node.parent().inner().to_string();

        if parent.is_empty() {
            if root.is_some() {
                bail!("found multiple roots: {} and {}", root.unwrap(), name);
            }

            root = Some(name);
            continue;
        }

        let parent_index = indexes
            .get(&parent)
            .cloned()
            .ok_or(anyhow!("specified parent node does not exist: {parent}"))?;

        graph.add_edge(parent_index, node_index, ());
    }

    let root = root.ok_or(anyhow!("unable to identify root!"))?;
    // SAFETY: this node is guaranteed to be in the indexes, because we already
    // looked it up in the operations earlier on. So this will always unwrap.
    let root = *indexes.get(&root).unwrap();

    Directory::scaffold_from_graph(args.output_directory, root, graph)
        .context("scaffolding the ontology directory")?;

    Ok(())
}
