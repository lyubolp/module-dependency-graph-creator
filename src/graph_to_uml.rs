use crate::{graph::Graph, module::PythonModule};
use crate::tree::{Node, insert};

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

    let mut root = Node::new(String::from("pygrader"));

    for node in graph.get_nodes() {
        let mut packages: Vec<String> = node.get_packages().iter().filter(|item| *item != "").map(|item| item.clone()).collect();
        packages.push(node.get_name().clone());

        insert(&mut root, packages);
    }


    for node in graph.get_nodes() {
        let content = String::from(&format!("[\"{}\"]", node.get_name()));
        result.push(content);
    }
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
