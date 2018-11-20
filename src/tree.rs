use super::data::*;
use super::node::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Tree{
    head : Rc<RefCell<Node>>,
    size : u32
}

impl Tree{
    pub fn new(tree_head : Rc<RefCell<Node>>) -> Tree{
        tree_head.borrow_mut().set_head(true);
        Tree{
            head : tree_head,
            size : 1
        }
    }

    pub fn add(&mut self, node : Rc<RefCell<Node>>){
        node.borrow_mut().add_child(self.head.clone());
        node.borrow_mut().compute_hash();
        self.head = node;
    }

    pub fn print_all_data(&self){
        self.head.borrow_mut().print_all_data();
    }
}