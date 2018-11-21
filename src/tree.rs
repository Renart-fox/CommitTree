use super::data::*;
use super::node::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

pub struct Tree{
    branches : HashMap<String, Rc<RefCell<Node>>>,
    last_working_branch : String,
    size : u32
}

impl Tree{
    pub fn new(tree_head : Rc<RefCell<Node>>) -> Tree{
        tree_head.borrow_mut().set_head(true);
        let mut tree = Tree{
            branches : HashMap::new(),
            last_working_branch : "master".to_string(),
            size : 1
        };
        tree.branches.insert("master".to_string(), tree_head);
        tree
    }

    pub fn push(&mut self, node : Rc<RefCell<Node>>, branch : String) -> String{
        if let Some(branch_head) = self.branches.clone().get(&branch){
            node.borrow_mut().add_child(branch_head.clone());
            node.borrow_mut().compute_hash();
            self.branches.insert("master".to_string(), node);
            self.last_working_branch = branch;
            self.size += 1;
            "Commit pushed".to_string()
        }
        else {
            "[error] branch does not exist".to_string()
        }
    }

    pub fn new_branch(&mut self, node : Rc<RefCell<Node>>, branch : String) -> String{
        if self.branches.clone().contains_key(&branch){
            "[error] branch with the same name already exist".to_string()
        }
        else {
            if let Some(branch_head) = self.branches.clone().get(&self.last_working_branch){
                node.borrow_mut().add_child(branch_head.clone());
            }
            self.branches.insert(branch.clone(), node);
            self.last_working_branch = branch;
            self.size += 1;
            "branch created".to_string()
        }
    }

    pub fn print_all_data(&self, branch : String) -> String{
        if let Some(branch_head) = self.branches.get(&branch){
            println!("Branch {}", branch);
            branch_head.borrow_mut().print_all_data();
            "Printed".to_string()
        }
        else {
            "[error] branch does not exist".to_string()
        }
    }
}