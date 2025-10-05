mod constants;
mod file_utils;
mod graph;
mod graph_to_uml;
mod module;
mod python_to_graph;
mod python_utils;

use crate::file_utils::discover_files;
use crate::graph_to_uml::generate_plantuml;
use crate::python_to_graph::build_dependency_graph;

fn main() {
    let root_dir = "/home/lyubolp/pygrader";
    let files = discover_files(root_dir);

    let graph = build_dependency_graph(files, root_dir);

    let content = generate_plantuml(&graph);

    for node in graph.get_nodes() {
        if let Ok(edges) = graph.get_edges(node) {
            println!("{} -> {:?}", node, edges);
        }
    }

    for line in content {
        println!("{}", line)
    }
}
