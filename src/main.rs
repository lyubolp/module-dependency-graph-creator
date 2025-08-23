use petgraph::graph::{UnGraph};
use petgraph::dot::{Dot, Config};

mod python_to_graph;
mod file_utils;
mod constants;

use crate::file_utils::discover_files;
use crate::python_to_graph::build_dependency_graph;

fn main() {
    println!("Hello, world!");

    // let g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

    // println!("{:?}", Dot::with_config(&g, &[Config::GraphContentOnly]));

    // println!("{:?}", discover_files("/home/lyubolp/pygrader/grader"));

    println!("{:?}", build_dependency_graph(vec![String::from("sample.py")]));
}