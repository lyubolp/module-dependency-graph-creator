mod python_to_graph;
mod python_utils;
mod file_utils;
mod constants;
mod graph;

use crate::file_utils::discover_files;
use crate::python_to_graph::build_dependency_graph;

use crate::python_utils::{extract_module_name};

fn main() {
    // let g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

    // println!("{:?}", Dot::with_config(&g, &[Config::GraphContentOnly]));

    let root_dir = "/home/lyubolp/pygrader";
    let files = discover_files(root_dir);

    let graph = build_dependency_graph(files, root_dir);


    for node in graph.get_nodes() {
        if let Ok(edges) = graph.get_edges(node) {
            println!("{} -> {:?}", node, edges);
        }

    }
}