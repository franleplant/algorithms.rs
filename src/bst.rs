use std::fmt::Debug;

pub trait Keyable {
    fn get_key(&self) -> i32;
}


impl Keyable for i32 {
    fn get_key(&self) -> i32 {
        *self
    }
}

#[derive(Debug)]
pub struct Node<T: Keyable + Debug > {
    data: T,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

#[derive(Debug)]
pub struct Bst<T: Keyable + Debug > {
    root: Option<usize>,
    nodes: Vec<Node<T>>,
}

impl<T: Keyable + Debug> Bst<T> {
    pub fn new() -> Bst<T> {
        Bst {
            root: None,
            nodes: vec![],
        }
    }

    pub fn insert(&mut self, data: T) {
        // New
        let node_index = self.nodes.len();

        // Previous ptr
        let mut y = None;
        // Current ptr
        let mut x = self.root;

        while let Some(x_index) = x {
            y = x;
            if data.get_key() < self.nodes[x_index].data.get_key() {
                x = self.nodes[x_index].left;
            } else {
                x = self.nodes[x_index].right;
            }
        }

        match y {
            None => {
                self.root = Some(0);
            }

            Some(y_index) => {
                if data.get_key() < self.nodes[y_index].data.get_key() {
                    self.nodes[y_index].left = Some(node_index);
                } else {
                    self.nodes[y_index].right = Some(node_index);
                }
            }
        }

        let node = Node {
            data: data,
            parent: y,
            left: None,
            right: None,
        };
        self.nodes.push(node);
    }


    fn inorder_walk_by_index(&self, x: usize) {
        if let Some(x_node) = self.nodes.get(x) {

            if let Some(left_index) = x_node.left {
                self.inorder_walk_by_index(left_index);
            }

            println!("{:?}", x_node.data);

            if let Some(right_index) = x_node.right {
                self.inorder_walk_by_index(right_index);
            }
        }
    }

    pub fn inorder_walk(&self) {
        self.inorder_walk_by_index(0);
    }





}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut bst = Bst::new();
        bst.insert(12);
        bst.insert(5);
        bst.insert(2);
        bst.insert(9);
        println!("{:?}", bst);

        bst.inorder_walk();
    }
}
