
mod first;

use first::List;

use std::fmt::Debug;


#[derive(Debug)]
struct BinaryTree<'a, T> {
    value: T,
    left: Option<&'a BinaryTree<'a, T>>,
    right: Option<&'a BinaryTree<'a, T>>
}

impl <'a, T> BinaryTree <'a, T> where T: Debug{
    fn new(a : T) -> Self {
        BinaryTree { value : a, left : None, right: None}
    }
    fn search(&self) {
        println!("Current value: {:?}", self.value);
        
        match self.left {
            Some(s) => s.search(),
            None => ()
        }
        
        match self.right {
            Some(s) => s.search(),
            None => ()
        }
    }
}

fn main() {
    let a = BinaryTree::new(20);
    let b = BinaryTree::new(10);
    let c = BinaryTree{ value : 30, left: Some(&a), right : Some(&b) };

    c.search();
}
