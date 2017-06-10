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
pub struct Node<T: Keyable + Debug> {
    data: T,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

#[derive(Debug)]
pub struct Bst<T: Keyable + Debug> {
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

    pub fn insert(&mut self, new_data: T) {
        let new = self.nodes.len();
        let mut maybe_previous = None;
        let mut maybe_current = self.root;

        while let Some(current) = maybe_current {
            maybe_previous = maybe_current;
            maybe_current = if new_data.get_key() < self.nodes[current].data.get_key() {
                self.nodes[current].left
            } else {
                self.nodes[current].right
            };
        }

        match maybe_previous {
            None => {
                self.root = Some(0);
            }

            Some(previous) => {
                if new_data.get_key() < self.nodes[previous].data.get_key() {
                    self.nodes[previous].left = Some(new);
                } else {
                    self.nodes[previous].right = Some(new);
                }
            }
        }

        let new_node = Node {
            data: new_data,
            parent: maybe_previous,
            left: None,
            right: None,
        };

        self.nodes.push(new_node);
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
