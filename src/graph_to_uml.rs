use crate::{graph::Graph, module::PythonModule};

pub fn generate_plantuml(graph: &Graph<PythonModule>) -> Vec<String> {
    let colors: Vec<&str> = vec![
        "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd", "#8c564b", "#e377c2", "#7f7f7f",
        "#bcbd22", "#17becf",
    ];

    let mut result: Vec<String> = vec![];

    result.push(String::from("@startuml"));

    result.push(String::from("skinparam packageStyle rectangle"));
    result.push(String::from("skinparam linetype ortho"));
    result.push(String::from("skinparam class {"));
    result.push(String::from("    BackgroundColor White"));
    result.push(String::from("    BorderColor Black"));
    result.push(String::from("}"));
    result.push(String::from("left to right direction"));

    // TODO - Probably need a tree representation of the modules to properly nest them

    for node in graph.get_nodes() {
        if let Ok(edges) = graph.get_edges(node) {
            for edge in edges {
                let content = String::from(&format!(
                    "[\"{}\"] --> [\"{}\"]",
                    node.get_name(),
                    edge.get_name()
                ));
                result.push(content);
            }
        }
    }

    result
}
