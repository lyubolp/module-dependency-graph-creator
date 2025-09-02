use std::{collections::HashSet, fs::read};
// use petgraph::graph::{UnGraph};

use ruff_python_parser;
use ruff_python_ast::Stmt;

use crate::python_utils::{extract_module_name, split_import};

pub fn build_dependency_graph(files: Vec<String>, root_dir: &str) {

    for filepath in files {
        let deps = get_all_dependencies(&filepath, root_dir);

        println!("{} -> {:?}", filepath, deps)
    }
}


fn get_all_dependencies(filepath: &String, root_dir: &str) -> HashSet<String> {

    let content = String::from_utf8(read(filepath).unwrap()).unwrap();
    let result = ruff_python_parser::parse_module(&content).unwrap();

    let mut names: HashSet<String> = HashSet::new();

    for item in result.syntax().body.clone() {
        names.extend(extract_names(item.clone(), root_dir).into_iter());
    }

    names
}

fn extract_names(item: Stmt, root_dir: &str) -> HashSet<String> {
    let names = if item.is_import_from_stmt(){
        extract_names_from_import_from_statement(item)
    }
    else if item.is_import_stmt() {
        extract_names_from_import_statement(item)
    }
    else {
        HashSet::new()
    };

    names
        .iter()
        .map(|name|extract_module_name(name, root_dir))
        .map(|name| split_import(&name).pop().unwrap_or(String::new()))
        .filter(|name| name != "")
        .collect()
}

fn extract_names_from_import_statement(item: Stmt) -> HashSet<String> {
    // TODO - This could return just an iterator
    item
        .import_stmt()
        .unwrap()
        .names
        .iter()
        .map(|import| import.name.id.clone())
        .collect()
}

fn extract_names_from_import_from_statement(item: Stmt) -> HashSet<String> {
    // TODO - This could return just an iterator
    let statement = item.import_from_stmt().unwrap();

    let Some(module) = statement.module else {
        // TODO - This ignores imports from parent package
        return HashSet::new();
    };
    
    statement
        .names
        .iter()
        .map(|alias| module.id.clone() + "." + &alias.name.id)
        .collect()
}

