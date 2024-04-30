type Next = Option<Box<Node>>;

pub struct Node {
    value: Option<char>,
    left: Next,
    right: Next,
}

impl Node {
    pub fn new(val: Option<char>) -> Self {
        return Node {
            value: val,
            left: None,
            right: None,
        }
    }
}

pub struct Tree {
    root: Option<Next>
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }
}


pub fn from(tree: String) -> Tree {
    let decoder = Tree::new();

    decoder
}
