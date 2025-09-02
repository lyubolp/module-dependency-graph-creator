// use petgraph::graph::{UnGraph};
// use petgraph::dot::{Dot, Config};

mod python_to_graph;
mod python_utils;
mod file_utils;
mod constants;

use crate::file_utils::discover_files;
use crate::python_to_graph::build_dependency_graph;

use crate::python_utils::{extract_module_name};

fn main() {
    // let g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

    // println!("{:?}", Dot::with_config(&g, &[Config::GraphContentOnly]));

    let root_dir = "/home/lyubolp/pygrader";
    let files = discover_files(root_dir);

    println!("{:?}", build_dependency_graph(files, root_dir));
    

    println!("{:?}", extract_module_name(&String::from("grader.checks.requirements_check.RequirementsCheck"), "/home/lyubolp/pygrader"));

}