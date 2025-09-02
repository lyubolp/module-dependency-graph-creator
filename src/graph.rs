use std::collections::{HashMap};

#[derive(Debug, Clone)]
pub struct Graph{
    node_to_id: HashMap<String, u32>,
    id_to_node: HashMap<u32, String>,
    edges: HashMap<u32, Vec<u32>>
}

impl Graph {
    pub fn new() -> Self {
        Graph { node_to_id: HashMap::new(), id_to_node: HashMap::new(), edges: HashMap::new() }
    }

    pub fn add_node(&mut self, node: &String) -> Result<&str, &str> {
        if !self.is_node_in_graph(&node) {
            let current_index = self.node_to_id.len() as u32;
            self.node_to_id.insert(node.clone(), current_index);
            self.id_to_node.insert(current_index, node.clone());
    
            Ok("Node added succesfully!")
        }
        else {
            Err("Node already in graph.")
        }
    }

    pub fn is_node_in_graph(&self, node: &String) -> bool {
        self.node_to_id.contains_key(node)
    }

    pub fn add_edge(&mut self, start: &String, end: &String) -> Result<&str, &str> {
        if !self.is_node_in_graph(start) {
            return Err("Start not in graph");
        }

        if !self.is_node_in_graph(end) {
            return Err("End not in graph");
        }

        if self.does_edge_exist(start, end) {
            return Err("Edge already exists");
        }

        let start_id = self.node_to_id[start];
        let end_id = self.node_to_id[end];

        if self.edges.contains_key(&start_id) {
            self.edges.get_mut(&start_id).unwrap().push(end_id);
        }
        else {
            self.edges.insert(start_id, vec![end_id]);
        }

        Ok("Edge added successfully")
    }

    pub fn does_edge_exist(&self, start: &String, end: &String) -> bool {
        if !self.is_node_in_graph(start) || !self.is_node_in_graph(end) {
            return false;
        }

        let start_id = self.node_to_id[start];
        let end_id = self.node_to_id[end];

        if !self.edges.contains_key(&start_id) {
            return false;
        }
        return self.edges[&start_id].contains(&end_id);
    }

    pub fn get_edges(&self, node: &String) -> Result<Vec<String>, &str> {
        if !self.is_node_in_graph(node) {
            return Err("Node not in graph");
        }
        let node_id = self.node_to_id[node];

        match self.edges.get(&node_id) {
            Some(edges) => {
                Ok(edges
                    .iter()
                    .map(|id| self.id_to_node[id].clone())
                    .collect())
            }
            None => Err("No edges found")
        }
    }

    pub fn get_nodes(&self) -> impl Iterator<Item = &String> {
        self.node_to_id.keys()
    }
}