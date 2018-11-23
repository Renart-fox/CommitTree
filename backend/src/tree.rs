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
        tree_head.borrow_mut().set_owner("master".to_string());
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
            branch_head.borrow_mut().set_head(false);
            node.borrow_mut().add_child(branch_head.clone());
            node.borrow_mut().compute_hash();
            node.borrow_mut().set_head(true);
            node.borrow_mut().set_owner(branch.clone());
            self.branches.insert("master".to_string(), node);
            self.last_working_branch = branch;
            self.size += 1;
            "Commit pushed".to_string()
        }
        else {
            "[error] branch does not exist".to_string()
        }
    }

    // Returns if the deletion was successful
    pub fn remove(&mut self, hash : String) -> bool{
        let mut result = false;
        for (branch_name, head) in self.branches.clone(){
            if !result{
                result = head.borrow_mut().remove(hash.clone());
            }
            if !result{ // Hash was not among children, maybe it's the head itself
                if head.borrow_mut().get_hash() == hash.clone(){
                    if head.borrow_mut().is_leaf(){
                        drop(head);
                        self.branches.remove_entry(&branch_name);
                    }
                    else if head.borrow_mut().get_children().len() == 1{
                        let mut child = head.borrow_mut().get_children()[0].clone();
                        child.borrow_mut().set_head(true);
                        self.branches.insert(branch_name.clone(), child);
                        drop(head);
                    } // Don't remove if children equals 2, you can't delete merge commits
                }
            }
        }
        result
    }

    pub fn new_branch(&mut self, node : Rc<RefCell<Node>>, branch : String) -> String{
        if self.branches.clone().contains_key(&branch){
            "[error] branch with the same name already exist".to_string()
        }
        else {
            if let Some(branch_head) = self.branches.clone().get(&self.last_working_branch){
                let mut head_clone = branch_head.borrow_mut().clone();
                head_clone.change_owner(branch.clone());
                let ref_to_clone = Rc::new(RefCell::new(head_clone));
                node.borrow_mut().add_child(ref_to_clone);
                node.borrow_mut().set_head(true);
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