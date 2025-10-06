use std::{collections::VecDeque, fmt::{Display, Formatter}};

pub struct Node {
    value: String,
    children: Vec<Box<Node>>,
}

impl Node {
    pub fn new(value: String) -> Self {
        Node {
            value,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(Box::new(child));
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn get_children(&self) -> &Vec<Box<Node>> {
        &self.children
    }

    
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let children_names: Vec<String> = self.children.iter().map(|item| item.value.clone()).collect();
        write!(
            f,
            "Node '{}' with children: [{}]",
            self.value,
            children_names.join(", ")
        )
    }
}

pub fn insert(root: &mut Node, parts: Vec<String>) {
    let Some(top) = parts.first() else {
        // No more stuff to insert, we are done
        return;
    };

    for child in root.children.iter_mut() {
        if child.value == top.clone() {
            // We have to insert here
            return insert(child, parts[1..].to_vec());
        }
    }

    // New node, let's add it
    let new_node = Node::new(top.clone());
    root.add_child(new_node);

}

pub fn bfs(root: &Node) {
    let mut queue: VecDeque<(&Node, u32)> = VecDeque::from([(root, 0)]);


    while !queue.is_empty() {
        let (node, level) = queue.pop_front().unwrap();

        println!("level {}: {}", level, node.value);

        for child in node.children.iter() {
            queue.push_back((&child, level + 1));
        }
    }

}