//! Scaffolding of an ontology directory.

use std::collections::VecDeque;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::bail;
use convert_case::Boundary;
use convert_case::Case;
use convert_case::Casing as _;
use ontology::Node;
use petgraph::Direction;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use petgraph::visit::Bfs;

/// Ontology directory structure operations.
pub struct Directory;

impl Directory {
    /// Scaffolds a directory structure from a graph.
    pub fn scaffold_from_graph(
        path: PathBuf,
        root_index: NodeIndex,
        graph: DiGraph<Node, ()>,
    ) -> anyhow::Result<()> {
        let mut bfs = Bfs::new(&graph, root_index);
        // SAFETY: the root is always expected to be in the graph.
        let root_name = graph.node_weight(root_index).unwrap().name().inner();

        while let Some(index) = bfs.next(&graph) {
            // This should always unwrap because we're walking with a BFS
            // specifically built on the graph in question (and we're not
            // modifying it).
            let node = graph.node_weight(index).unwrap();

            let mut current_node = node.clone();
            let mut current_index = index;
            let mut path_elements = VecDeque::new();

            loop {
                let mut neighbors = graph
                    .neighbors_directed(current_index, Direction::Incoming)
                    .collect::<Vec<_>>();

                if neighbors.is_empty() {
                    let current_node_name = current_node.name().inner();

                    if current_node_name != root_name {
                        bail!("found a root node named {current_node_name}");
                    }

                    // NOTE: if we get here, we've reached the root node.
                    break;
                }

                assert!(neighbors.len() == 1, "there should only be one parent!");

                // SAFETY: we just checked that there is exactly one neighbor.
                current_index = neighbors.pop().unwrap();
                current_node = graph.node_weight(current_index).unwrap().clone();
                // SAFETY: this should always unwrap, as the node is clearly
                // connected as the parent within the graph.
                path_elements.push_front(current_node.name().inner().to_string());
            }

            path_elements.push_back(format!("{}.yml", node.name().inner()));

            let file = path_elements
                .into_iter()
                .map(|path| {
                    clean_path_name(path)
                        .from_case(Case::Title)
                        // This keeps gene names together instead of splitting
                        // them (e.g., `kmt2a` instead of `kmt-2-a`).
                        .without_boundaries(&[Boundary::DigitUpper, Boundary::DigitLower])
                        .to_case(Case::Kebab)
                })
                .fold(path.clone(), |mut acc, part| {
                    acc.push(part);
                    acc
                });

            // SAFETY: because we pass in a path to the function, the parent
            // will always be present and this will unwrap.
            std::fs::create_dir_all(file.parent().unwrap())
                .with_context(|| format!("creating directory: {}", file.display()))?;

            let writer = File::create(&file)
                .map(BufWriter::new)
                .with_context(|| format!("opening writer to {}", file.display()))?;

            serde_yaml::to_writer(writer, node).context("serializing node")?;
        }

        Ok(())
    }
}

/// Characters to remove from file names.
const CHARS_TO_REMOVE: &[char] = &[',', ';'];

/// Cleans a file name of unwanted sequences.
fn clean_path_name(mut name: String) -> String {
    for c in CHARS_TO_REMOVE {
        name = name.replace(*c, "");
    }

    name
}
