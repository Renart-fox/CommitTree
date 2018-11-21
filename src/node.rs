extern crate md5;

use super::data::*;
use std::cell::RefCell;
use std::rc::Rc;
use self::md5::*;

pub struct Node{
    data : Data,
    hash : Option<String>,
    is_head : bool,
    is_leaf : bool,
    message : String,
    children : Vec<Rc<RefCell<Node>>>
}

impl Node{
    pub fn new(node_data : Data, commit_message : String) -> Rc<RefCell<Node>> {
        let node = Rc::new(RefCell::new(Node{
            data : node_data,
            hash : None,
            is_head : false,
            is_leaf : false,
            message : commit_message,
            children : Vec::new()
        }));
        node.borrow_mut().compute_hash();
        node
    }

    pub fn add_child(&mut self, node : Rc<RefCell<Node>>) -> bool {
        let mut added_node = true;

        if self.children.len() < 2 as usize {
            self.children.push(node);
        }
        else {
            added_node = false;
        }
        added_node
    }

    pub fn get_data(&self) -> &Data{
        &self.data
    }

    pub fn get_message(&self) -> &String{
        &self.message
    }

    pub fn set_message(&mut self, new_message : String){
        self.message = new_message;
    }

    pub fn print_all_data(&self){
        println!("Commit '{}'", &self.message);
        print!("Data name: '{}' Held information -> ", self.data.get_name());
        match self.data.get_value() {
            DataType::String(v) => print!("{}", v),
            DataType::Integer(v)  => print!("{}", v),
            DataType::Float(v)  => print!("{}", v)
        }
        if let Some(hash) = self.hash.clone(){
            println!(" hash: {}", hash);
        }
        &self.children.iter().for_each(|child| child.borrow_mut().print_all_data());
    }

    pub fn set_head(&mut self, is_head : bool){
        self.is_head = is_head;
    }

    pub fn get_hash(&self) -> String{
        let mut hash = String::new();
        if let Some(str) = self.hash.clone(){
            hash = str;
        }
        hash
    }

    pub fn compute_hash(&mut self){
        let mut value = String::new();
        // If we have children, we build our hash according to their hash
        if self.children.len() > 0{
            self.children.iter().for_each(|child|
                value.push_str(&*child.borrow_mut().get_hash())
            );
        }
        // If we don't have children, we build our hash according to our data
        else{
            match self.get_data().get_value(){
                    DataType::String(s) => value.push_str(s),
                    DataType::Integer(i) => value.push_str(&*i.to_string()),
                    DataType::Float(f) => value.push_str(&*f.to_string())
            }
        }
        self.hash.get_or_insert(format!("{:x}", md5::compute(value)));
    }
}
