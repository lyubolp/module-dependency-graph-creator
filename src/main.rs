mod python_to_graph;
mod python_utils;
mod file_utils;
mod constants;
mod graph;
mod graph_to_uml;
mod module;

use crate::file_utils::discover_files;
use crate::python_to_graph::build_dependency_graph;

fn main() {
    let root_dir = "/home/lyubolp/pygrader";
    let files = discover_files(root_dir);

    let graph = build_dependency_graph(files, root_dir);

    for node in graph.get_nodes() {
        if let Ok(edges) = graph.get_edges(node) {
            println!("{} -> {:?}", node, edges);
        }

    }
}