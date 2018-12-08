const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("small_input.txt");

#[derive(Debug, Clone)]
struct Node {
    children_count: usize,
    children: Vec<Rc<RefCell<Node>>>,
    metadata_count: usize,
    metadata: Vec<usize>,
}

impl Node {
    fn total(&self) -> usize {
        if self.children_count == 0 {
            let v = self.metadata.clone().into_iter().fold(0, |acc, m| acc + m);
            return v;
        } else {
            return self
                .metadata
                .clone()
                .into_iter()
                .map(|m| match self.children.get(m - 1) {
                    Some(node) => node.borrow().total(),
                    None => 0,
                }).fold(0, |acc, m| acc + m);
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
pub fn part_1() {
    let mut inputs = INPUT
        .split(' ')
        .map(|c| c.parse::<usize>().unwrap())
        .peekable();
    let root = Node {
        children_count: inputs.next().unwrap(),
        children: vec![],
        metadata_count: inputs.next().unwrap(),
        metadata: vec![],
    };
    // let mut current_node = Box::new(root);
    let mut node_stack = vec![Rc::new(RefCell::new(root))];
    let mut tree = None;
    let mut checksum = 0;
    while node_stack.len() > 0 {
        let top_node = node_stack.last_mut().cloned().unwrap();
        let mut current_node = top_node.borrow_mut();
        if current_node.children_count > current_node.children.len() {
            //Add a child node and parse it
            let child = Node {
                children_count: inputs.next().unwrap(),
                children: vec![],
                metadata_count: inputs.next().unwrap(),
                metadata: vec![],
            };

            let boxed_child = Rc::new(RefCell::new(child));
            node_stack.push(Rc::clone(&boxed_child));
            current_node.children.push(boxed_child);
        } else {
            //Parse metadata and pop up to parent
            for _i in 0..current_node.metadata_count {
                current_node.metadata.push(inputs.next().unwrap());
            }
            checksum += current_node
                .metadata
                .clone()
                .into_iter()
                .fold(0, |acc, m| acc + m);

            tree = Some(node_stack.pop());
        }
    }

    println!("Tree: {:?}", tree.unwrap());
    println!("Checksum {}", checksum);
}
pub fn part_2() {
    let mut inputs = INPUT
        .split(' ')
        .map(|c| c.parse::<usize>().unwrap())
        .peekable();
    let root = Node {
        children_count: inputs.next().unwrap(),
        children: vec![],
        metadata_count: inputs.next().unwrap(),
        metadata: vec![],
    };
    // let mut current_node = Box::new(root);
    let mut node_stack = vec![Rc::new(RefCell::new(root))];
    let mut tree = None;

    while node_stack.len() > 0 {
        let top_node = node_stack.last_mut().cloned().unwrap();
        let mut current_node = top_node.borrow_mut();
        if current_node.children_count > current_node.children.len() {
            //Add a child node and parse it
            let child = Node {
                children_count: inputs.next().unwrap(),
                children: vec![],
                metadata_count: inputs.next().unwrap(),
                metadata: vec![],
            };

            let boxed_child = Rc::new(RefCell::new(child));
            node_stack.push(Rc::clone(&boxed_child));
            current_node.children.push(boxed_child);
        } else {
            //Parse metadata and pop up to parent
            for _i in 0..current_node.metadata_count {
                current_node.metadata.push(inputs.next().unwrap());
            }

            tree = node_stack.pop();
        }
    }

    let finished_tree = &*tree.unwrap();
    println!("Tree: {:?}", finished_tree);
    println!("Checksum {}", finished_tree.clone().into_inner().total());
}
