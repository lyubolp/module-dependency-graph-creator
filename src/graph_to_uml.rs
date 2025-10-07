use std::collections::{HashSet, VecDeque};

use crate::tree::{TreeNode, insert};
use crate::{graph::Graph, module::PythonModule};

pub fn generate_plantuml(graph: &Graph<PythonModule>) -> Vec<String> {
    let colors: Vec<&str> = vec![
        "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd", "#8c564b", "#e377c2", "#7f7f7f",
        "#bcbd22", "#17becf",
    ];

    let mut result: Vec<String> = vec![];

    result.push(String::from("@startuml"));

    result.push(String::from(""));

    result.push(String::from("skinparam packageStyle rectangle"));
    result.push(String::from("skinparam linetype ortho"));
    result.push(String::from("skinparam class {"));
    result.push(String::from("    BackgroundColor White"));
    result.push(String::from("    BorderColor Black"));
    result.push(String::from("}"));
    result.push(String::from("left to right direction"));

    result.push(String::from(""));
    // TODO - Probably need a tree representation of the modules to properly nest them

    let tree_node = build_tree_from_dependency_graph(graph);

    let modules: HashSet<String> = graph
        .get_nodes()
        .map(|item| item.get_name().clone())
        .collect();

    for item in declare_modules_into_packages(&tree_node, &modules) {
        result.push(item);
    }

    // for node in graph.get_nodes() {
    //     let content = String::from(&format!("[\"{}\"]", node.get_name()));
    //     result.push(content);
    // }

    result.push(String::from(""));

    for node in graph.get_nodes() {
        if let Ok(edges) = graph.get_edges(node) {
            for (i, edge) in edges.iter().enumerate() {
                let content = String::from(&format!(
                    "[\"{}\"] -[{}]-> [\"{}\"]",
                    node.get_name(),
                    colors[i % colors.len()],
                    edge.get_name()
                ));
                result.push(content);
            }
            result.push(String::from(""));
        }
    }

    result.push(String::from("@enduml"));
    result
}

fn build_tree_from_dependency_graph(graph: &Graph<PythonModule>) -> TreeNode {
    let mut root = TreeNode::new(String::from("pygrader"));

    for node in graph.get_nodes() {
        let mut packages: Vec<String> = node
            .get_packages()
            .iter()
            .filter(|item| *item != "")
            .map(|item| item.clone())
            .collect();
        packages.push(node.get_name().clone());

        insert(&mut root, packages);
    }

    root
}

fn declare_modules_into_packages(tree_root: &TreeNode, modules: &HashSet<String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    let mut queue: VecDeque<(&TreeNode, u32)> = VecDeque::from([(tree_root, 0)]);

    while !queue.is_empty() {
        let (node, level) = queue.pop_back().unwrap();
        let node_name = node.get_value();

        if modules.contains(node_name) {
            result.push(String::from(&format!("[\"{}\"]", node_name)));
        } else {
            result.push(String::from(&format!("package \"{}\" {{", node_name)));
        }

        for child in node.get_children().iter() {
            queue.push_back((&child, level + 1));
        }
    }
    result
}
