use crate::common::to_num;
use crate::day7::NodeType::{DIR, FILE};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Tree {
    pub root: RefCell<Rc<Node>>,
}

#[derive(Debug)]
pub struct Node {
    pub children: RefCell<Vec<Rc<Node>>>,
    pub parent: RefCell<Weak<Node>>,
    pub node_type: NodeType,
    pub size: usize,
    pub name: String,
}

#[derive(PartialEq, Debug)]
pub enum NodeType {
    DIR,
    FILE,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            root: RefCell::new(Rc::new(Node {
                children: RefCell::new(vec![]),
                parent: RefCell::new(Default::default()),
                node_type: DIR,
                size: 0,
                name: Default::default(),
            })),
        }
    }
    pub fn make_dir(&self, name: &str) -> Rc<Node> {
        Rc::new(Node {
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
            node_type: DIR,
            size: 0,
            name: String::from(name),
        })
    }
    pub fn make_file(&self, name: &str, size: usize) -> Rc<Node> {
        Rc::new(Node {
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
            node_type: FILE,
            size,
            name: String::from(name),
        })
    }
    pub fn set_root(&self, node: Rc<Node>) {
        self.root.replace(node);
    }
    pub fn append(&self, parent: &Rc<Node>, child: &Rc<Node>) {
        parent.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Rc::downgrade(parent);
    }
    pub fn get_size(&self, parent: &Rc<Node>) -> usize {
        if parent.node_type == FILE {
            return parent.size;
        } else {
            parent
                .children
                .borrow()
                .iter()
                .map(|node| self.get_size(node))
                .sum()
        }
    }
    pub fn traverse_tree(&self) -> Vec<Rc<Node>> {
        let mut stack: Vec<Rc<Node>> = Vec::from([self.root.borrow().clone()]);
        let mut collected: Vec<Rc<Node>> = Vec::from([self.root.borrow().clone()]);

        while let Some(node) = stack.pop() {
            for child in node.children.borrow().iter() {
                collected.push(child.clone());
                stack.push(child.clone());
            }
        }

        return collected;
    }
}

pub fn parse_string(input: &str) -> Tree {
    let tree = Tree::new();
    tree.set_root(tree.make_dir("/"));
    let mut current_dir: Rc<Node> = tree.root.borrow_mut().clone();
    for line in input.lines() {
        match line {
            _ if line.contains("$ cd") => match line {
                _ if line.contains("/") => {
                    current_dir = tree.root.borrow_mut().clone();
                }
                _ if line.contains("..") => {
                    let a = current_dir.parent.borrow().upgrade().unwrap();
                    current_dir = a;
                }
                _ => {
                    let name = line.split_once("cd ").unwrap().1;
                    let children = current_dir.children.borrow().clone();
                    let existing_child = children.iter().find(|node| node.name == name);
                    if existing_child.is_some() {
                        current_dir = existing_child.unwrap().clone();
                    } else {
                        let new_dir = tree.make_dir(name);
                        tree.append(&current_dir, &new_dir);
                        current_dir = new_dir;
                    }
                }
            },
            _ if line.contains("$ ls") => {}
            _ => {
                if line.contains("dir") {
                    let name = line.split_once(" ").unwrap().1;
                    let new_dir = tree.make_dir(name);
                    tree.append(&current_dir, &new_dir);
                } else {
                    let parsed = line.split_once(" ").unwrap();
                    let new_dir = tree.make_file(parsed.1, to_num(parsed.0));
                    tree.append(&current_dir, &new_dir);
                }
            }
        }
    }
    return tree;
}

pub fn day7a(input: &str) -> usize {
    let a = parse_string(input);
    let b = a.traverse_tree();
    let mut sum = 0;
    for node in b {
        let size = a.get_size(&node);
        if node.node_type == DIR && size <= 100000 {
            sum += size
        }
    }
    return sum;
}

pub fn day7b(input: &str) -> usize {
    let tree = parse_string(input);
    let all_nodes = tree.traverse_tree();

    let space_needed = 30000000 - (70000000 - tree.get_size(&tree.root.borrow()));
    let mut sorted_nodes = all_nodes
        .iter()
        .filter(|node| node.node_type == DIR)
        .map(|node| (node, tree.get_size(node)))
        .filter(|(_, size)| *size >= space_needed)
        .collect::<Vec<(&Rc<Node>, usize)>>();

    sorted_nodes.sort_by(|a, b| (*a).1.partial_cmp(&(*b).1).unwrap());
    return (*sorted_nodes.first().unwrap()).1;
}

pub fn day7() {
    println!("{}", day7b(include_str!("../inputs/day7")));
}

#[cfg(test)]
mod test {
    use crate::day7::NodeType::{DIR, FILE};
    use crate::day7::Tree;

    #[test]
    fn testTree() {
        let tree = Tree::new();
        let a = Tree::make_dir(&tree, "a");
        let e = Tree::make_dir(&tree, "e");
        let i = Tree::make_file(&tree, "i.txt", 10);
        let f = Tree::make_file(&tree, "f", 1000);
        let g = Tree::make_file(&tree, "g", 10000);
        tree.set_root(a.clone());
        tree.append(&a, &e);
        tree.append(&e, &i);
        tree.append(&a, &g);
        tree.append(&a, &f);
        assert_eq!(tree.get_size(&a), 11010);
        assert_eq!(tree.get_size(&e), 10);
    }
}
