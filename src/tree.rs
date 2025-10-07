use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

pub struct TreeNode {
    value: String,
    children: Vec<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(value: String) -> Self {
        TreeNode {
            value,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(Box::new(child));
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn get_children(&self) -> &Vec<Box<TreeNode>> {
        &self.children
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let children_names: Vec<String> = self
            .children
            .iter()
            .map(|item| item.value.clone())
            .collect();
        write!(
            f,
            "Node '{}' with children: [{}]",
            self.value,
            children_names.join(", ")
        )
    }
}

pub fn insert(root: &mut TreeNode, parts: Vec<String>) {
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
    let new_node = TreeNode::new(top.clone());
    root.add_child(new_node);
}

pub fn bfs(root: &TreeNode) {
    let mut queue: VecDeque<(&TreeNode, u32)> = VecDeque::from([(root, 0)]);

    while !queue.is_empty() {
        let (node, level) = queue.pop_front().unwrap();

        println!("level {}: {}", level, node.get_value());

        for child in node.get_children().iter() {
            queue.push_back((&child, level + 1));
        }
    }
}
