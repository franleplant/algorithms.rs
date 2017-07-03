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

    fn preorder_walk_by_index(&self, x: usize, level: usize, connect: Vec<bool>) {
        if let Some(x_node) = self.nodes.get(x) {

            let separator = "|-- ";
            let s = format!("{}{:?}", separator, x_node.data);
            let space_n = level * separator.len();

            let mut space = String::new();
            for i in 0..space_n {
                if i % separator.len() == 0 && connect[i / separator.len()] {
                    space.push_str("|");
                } else {
                    space.push_str(" ");
                }
            }
            println!("{}{}", space, s);

            if let Some(left_index) = x_node.left {
                let mut connect = connect.clone();
                connect.push(true);
                self.preorder_walk_by_index(left_index, level + 1, connect);
            }

            if let Some(right_index) = x_node.right {
                let mut connect = connect.clone();
                connect.push(false);
                self.preorder_walk_by_index(right_index, level + 1, connect);
            }
        }
    }

    pub fn preorder_walk(&self) {
        self.preorder_walk_by_index(0, 0, vec![false]);
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
        println!("root {:?}", bst.root);
        for (i, n) in bst.nodes.iter().enumerate() {
            println!("{}, {:?}", i, n);
        }
        println!("");

        bst.inorder_walk();
    }

    #[test]
    fn preorder() {
        let mut bst = Bst::new();
        bst.insert(12);
        bst.insert(5);
        bst.insert(2);
        bst.insert(9);
        bst.insert(3);
        bst.insert(4);
        bst.insert(51);
        bst.insert(0);
        bst.insert(1);
        println!("root {:?}", bst.root);
        for (i, n) in bst.nodes.iter().enumerate() {
            println!("{}, {:?}", i, n);
        }
        println!("");

        bst.preorder_walk();
    }
}
