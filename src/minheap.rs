use rayon::prelude::*;
use std::collections::HashMap;

#[cfg(test)]
use std::sync::{
    Arc,
    Mutex
};

type Next = Option<Box<Node>>;

#[derive(Debug)]
pub enum NodeType {
    Parent(u64),
    Leaf(char, u64),
}

#[derive(Debug)]
struct Node {
    t: NodeType,
    right: Next,
    left: Next,
}

impl Node {
    pub fn get_count(&self) -> u64 {
        match self.t {
            NodeType::Parent(i) => i,
            NodeType::Leaf(_, i) => i,
        }
    }
}

pub struct MinHeap{
    root: Next
}


impl MinHeap {
    // TODO: this will need fixed moving forward
    // doesn't produce tree safely or accurately
    pub fn new(chars: Vec<char>) -> Self {
        if chars.len() == 0 {
            return MinHeap { root: None }
        }

        let mut map: HashMap<char, u64> = HashMap::new();

        for c in chars.iter() {
            if let Some(value) = map.get_mut(c) {
                *value += 1;
            } else {
                map.insert(*c, 1);
            }
        }

        // loop and make nodes for each item in map
        let mut nodes: Vec<Box<Node>> = map.par_iter()
            .map(|(c, i)| {
                Box::new(Node { t: NodeType::Leaf(*c, *i), right: None, left: None })
            })
        .collect();



        while nodes.len() > 1 {
            // FIX: figure out how to remove this later
            nodes.par_sort_by(|x, y| {
                y.get_count().cmp(&x.get_count())
            });

            // find two min
            // since min at end pop last 2
            let min1 = nodes.pop().unwrap();
            let min2 = nodes.pop().unwrap();

            // join under one node
            let new_node = Box::new(Node {
                t: NodeType::Parent(min1.get_count() + min2.get_count()),
                right: Some(min1),
                left: Some(min2),
            });

            // insert new node in correct spot
            nodes.push(new_node);
        };

        let root = nodes.pop().unwrap();

        MinHeap { root: Some(root) }
    }

    // FIX: temp idea
    // also convert string to vec of bits
    pub fn get_char_paths(&self) -> HashMap<char, String> {
        fn parse(node: &Next, chars: &mut HashMap<char, String>, path: String) {
            match node {
                Some(inner_node) => {
                    match &inner_node.t {
                        NodeType::Parent(_) => {
                            let mut left_path = path.clone();
                            left_path.push('0');
                            let mut right_path = path.clone();
                            right_path.push('1');
                            parse(&inner_node.left, chars, left_path);
                            parse(&inner_node.right, chars, right_path);
                        },
                        NodeType::Leaf(c, _) => {
                            if !chars.contains_key(c) {
                                chars.insert(c.clone(), path);
                            }
                        },
                    }
                },
                None => {}
            }
        }

        let mut chars: HashMap<char, String> = HashMap::new();
        let path = String::new();
        parse(&self.root, &mut chars, path);
        chars
    }

    #[cfg(test)]
    pub fn top(&self) -> u64 {
        match &self.root {
            Some(root) => {
                root.get_count()
            },
            None => 0
        }
    }

    #[cfg(test)]
    pub fn most_common(&self) -> Option<char> {
        fn dfs<'a>(node: &'a Next, best: Arc<Mutex<&'a Next>>) {
            match node {
                Some(node_inner) => {
                    match node_inner.t {
                        NodeType::Parent(_) => {
                            // TODO: add check here if further depth is required
                            dfs(&node_inner.left, best.clone());
                            dfs(&node_inner.right, best.clone());
                        },
                        NodeType::Leaf(_, i) => {
                            let mut unlocked_best = best.lock().unwrap();
                            if let Some(best_node) = *unlocked_best {
                                match best_node.t {
                                    NodeType::Parent(_) => {
                                        *unlocked_best = node
                                    },
                                    NodeType::Leaf(_, best_i) => {
                                        if best_i < i {
                                            *unlocked_best = node;
                                        }
                                    },
                                }
                            } else {
                                *unlocked_best = node;
                            }
                        },
                    }
                },
                None => {}
            }
        }
        
        let best = Arc::new(Mutex::new(&None));
        dfs(&self.root, best.clone());
        
        match *best.lock().unwrap() {
            Some(node) => {
                match node.t {
                    NodeType::Leaf(c,_) => {
                        return Some(c)
                    },
                    _ => {}
                }
            },
            _ => {},
        }

        return None
    }

    // mark parent node as a 0
    // leaf node as 1 followed by 8 bit char encoding
    // then move to left node then right
    pub fn encode(&self) -> String {
        fn parse(node: &Next, tree: &mut String) {
            match node {
                Some(inner_node) => {
                    match inner_node.t {
                        NodeType::Parent(_) => {
                            tree.push('0');
                            parse(&inner_node.left, tree);
                            parse(&inner_node.right, tree);
                        },
                        NodeType::Leaf(c, _) => {
                            tree.push('1');
                            tree.push(c);
                        },
                    }
                },
                None => {},
            }
        }

        let mut tree = String::new();
        parse(&self.root, &mut tree);
        tree
    }
}



#[test]
fn tester() {
    let y = String::from("aaabbc");
    let mh = MinHeap::new(y.chars().collect());
    println!("{:#?}", mh.root);
    assert_eq!(mh.top(), y.len() as u64);

    let x = String::from("a;sldkfja;vbalskdjfl;kajdsbjakshdlkfjalksdjv\nasdadsfasdf");
    let mh = MinHeap::new(x.chars().collect());
    println!("{:#?}", mh.root);
    assert_eq!(mh.top(), x.len() as u64);
}

#[test]
fn most(){
    let x = String::from("aaaa");
    let mh = MinHeap::new(x.chars().collect());
    assert_eq!(Some('a'), mh.most_common());

    let x = String::from("aaaaaasdfadsfadsfadsfasdfdsf");
    let mh = MinHeap::new(x.chars().collect());
    assert_eq!(Some('a'), mh.most_common());
}

#[test]
fn does_it_run(){
    let x = String::from("aaaabbbccd");
    let mh = MinHeap::new(x.chars().collect());

    let paths = mh.get_char_paths();
    for (key, val) in paths.iter() {
        println!("{}: {}", key, val);
    }

    // assert_eq!(1, 0); // uncomment to print map
}
