use std::fs::{File, read};
use std::io::{self, BufRead, Read};
use std::str::FromStr;
use petgraph::graph::{UnGraph};
use regex::Regex;
use crate::constants::{IMPORT};

use ruff_python_parser;

pub fn build_dependency_graph(files: Vec<String>) {

    let filepath=  &files[0];

    get_dependencies(filepath);
    
    // for item in get_dependencies(&files[0]).unwrap(){
    //     //println!("{}", item);
    //     println!("{} -> {:?}", item, extract_module(&item).unwrap());
    // }
}


fn get_dependencies(filepath: &String) -> Result<Vec<String>, String> {

    let content = String::from_utf8(read(filepath).unwrap()).unwrap();

    let result = ruff_python_parser::parse_module(&content).unwrap();

    for item in result.syntax().body.clone() {
        if item.is_import_from_stmt(){
            println!("{:?}", item);
        }
        else if item.is_import_stmt() {
            println!("{:?}", item);
        }
    }

    
    Ok(vec![])
}

fn is_line_import(line: &String) -> bool {
    // TODO - This doesn't work for multiline imports
    line.contains(IMPORT) && !line.contains("(")
}

fn extract_module(import_line: &String) -> Result<String, String> {
    let re = Regex::new(r"(from (.*))?import ([^\d\W][\w.]*)").unwrap();

    let Some(captures) = re.captures(&import_line) else {
        return Err(String::from("Can't search line"));
    };

    let module = if import_line.contains("from") {
        String::from_str(&captures[2].trim_end()).unwrap()
    }
    else {
        String::from_str(&captures[3].trim_end()).unwrap()
    };

    Ok(module)
}