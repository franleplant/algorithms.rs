use std::fmt::Debug;
use std::collections::HashMap;


// TODO probably this is redundat since we are using PartialOrd
// for comparing data directly, explore more
pub trait Keyable {
    fn get_key(&self) -> i32;
}


impl Keyable for i32 {
    fn get_key(&self) -> i32 {
        *self
    }
}

#[derive(Debug, PartialEq)]
pub struct Node<T: Keyable + Debug + PartialOrd> {
    data: T,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    index: usize,
}

#[derive(Debug)]
pub struct Bst<T: Keyable + Debug + PartialOrd> {
    root: Option<usize>,
    nodes: HashMap<usize, Node<T>>,
    last_index: usize,
}

impl<T: Keyable + Debug + PartialOrd> Bst<T> {
    pub fn new() -> Bst<T> {
        Bst {
            root: None,
            nodes: HashMap::new(),
            last_index: 0,
        }
    }

    pub fn insert(&mut self, new_data: T) {
        debug_assert!(self.bst_invariant());
        let new_index = self.last_index;
        let mut previous = None;
        let mut current = self.root;

        while current != None {
            let current_node = self.nodes.get(&current.unwrap()).unwrap();
            previous = current;
            current = if new_data < current_node.data {
                current_node.left
            } else {
                current_node.right
            };
        }

        if previous == None {
            self.root = Some(0);
        } else {
            let previous_node = self.nodes.get_mut(&previous.unwrap()).unwrap();
            if new_data < previous_node.data {
                previous_node.left = Some(new_index);
            } else {
                previous_node.right = Some(new_index);
            }
        }

        let new_node = Node {
            data: new_data,
            parent: previous,
            left: None,
            right: None,
            index: new_index,
        };

        self.nodes.insert(new_index, new_node);
        self.last_index += 1;
        debug_assert!(self.bst_invariant());
    }

    fn bst_invariant(&self) -> bool {
        for (_, node) in &self.nodes {
            if node.left != None {
                let left = self.nodes.get(&node.left.unwrap()).unwrap();
                if !(node.data > left.data) {
                    return false;
                }
            }

            if node.right != None {
                let right = self.nodes.get(&node.right.unwrap()).unwrap();
                if !(node.data < right.data) {
                    return false;
                }
            }
        }

        true
    }


    fn inorder_walk_by_index(&self, x: usize) {
        let node = self.nodes.get(&x);
        if node == None {
            return
        }

        let node = node.unwrap();


        if node.left != None {
            self.inorder_walk_by_index(node.left.unwrap());
        }

        println!("{:?}", node.data);

        if node.right != None {
            self.inorder_walk_by_index(node.right.unwrap());
        }
    }

    pub fn inorder_walk(&self) {
        self.inorder_walk_by_index(0);
    }

    fn preorder_walk_by_index(&self, x: usize, level: usize, connect: Vec<bool>) {
        if let Some(x_node) = self.nodes.get(&x) {

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

    pub fn search(&self, key: T) -> Option<&Node<T>> {
        let mut x = self.root;

        while x != None {
            let node = self.nodes.get(&x.unwrap()).expect("WTF");

            if key == node.data {
                break;
            }

            x = if key < node.data {
                node.left
            } else {
                node.right
            }
        }


        if x != None {
            self.nodes.get(&x.unwrap())
        } else {
            None
        }
    }

    pub fn min(&self, index: usize) -> Option<&Node<T>> {
        let mut x = Some(index);

        while x != None {
            let left = self.nodes.get(&x.unwrap()).unwrap().left;
            if left == None {
                break;
            }

            x = left
        }

        if x != None {
            self.nodes.get(&x.unwrap())
        } else {
            None
        }
    }

    pub fn max(&self, index: usize) -> Option<&Node<T>> {
        let mut x = Some(index);

        while x != None {
            let right = self.nodes.get(&x.unwrap()).unwrap().right;
            if right == None {
                break;
            }

            x = right
        }

        if x != None {
            self.nodes.get(&x.unwrap())
        } else {
            None
        }
    }

    pub fn successor(&self, index: usize) -> Option<&Node<T>> {
        let mut x = self.nodes.get(&index).expect("Wrong index");

        if let Some(right_index) = x.right {
            return self.min(right_index);
        }

        if x.parent != None {
            let mut parent = self.nodes
                .get(&x.parent.unwrap())
                .expect("Wrong parent index");

            loop {
                if parent.left != None {
                    if parent.left.unwrap() == x.index {
                        return Some(parent);
                    }
                }

                if parent.parent != None {
                    x = parent;
                    parent = self.nodes
                        .get(&parent.parent.unwrap())
                        .expect("Wrong parent index");
                } else {
                    break;
                }
            }
        }

        None
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
        for (i, n) in &bst.nodes {
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
        for (i, n) in &bst.nodes {
            println!("{}, {:?}", i, n);
        }
        println!("");

        bst.preorder_walk();
    }

    #[test]
    fn bst_search() {
        let mut bst = Bst::new();
        bst.insert(15);
        bst.insert(18);
        bst.insert(17);
        bst.insert(20);
        bst.insert(6);
        bst.insert(3);
        bst.insert(2);
        bst.insert(4);
        bst.insert(7);
        bst.insert(13);
        bst.insert(9);
        //bst.preorder_walk();

        assert_eq!(15, bst.search(15).unwrap().data);
        assert_eq!(13, bst.search(13).unwrap().data);
        assert_eq!(None, bst.search(55));
    }

    #[test]
    fn bst_min_max() {
        let mut bst = Bst::new();
        bst.insert(15);
        bst.insert(18);
        bst.insert(17);
        bst.insert(20);
        bst.insert(6);
        bst.insert(3);
        bst.insert(2);
        bst.insert(4);
        bst.insert(7);
        bst.insert(13);
        bst.insert(9);
        //bst.preorder_walk();

        assert_eq!(2, bst.min(bst.root.unwrap()).unwrap().data);
        assert_eq!(20, bst.max(bst.root.unwrap()).unwrap().data);
    }

    #[test]
    fn bst_successor() {
        let mut bst = Bst::new();
        bst.insert(15);
        bst.insert(18);
        bst.insert(17);
        bst.insert(20);
        bst.insert(6);
        bst.insert(3);
        bst.insert(2);
        bst.insert(4);
        bst.insert(7);
        bst.insert(13);
        bst.insert(9);
        println!("root {:?}", bst.root);
        for (i, n) in &bst.nodes {
            println!("{}, {:?}", i, n);
        }
        println!("");


        assert_eq!(bst.successor(0).unwrap().data, 17);
        assert_eq!(bst.successor(9).unwrap().data, 15);
    }
}
