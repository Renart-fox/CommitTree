use super::data::*;
use std::io::Write;
use std::io::stdout;

pub struct Node{
    data : Data,
    hash : Option<String>,
    is_head : bool,
    is_leaf : bool,
    children : Vec<Node>
}

impl Node{
    pub fn new(node_data : Data) -> Node {
        Node{
            data : node_data,
            hash : None,
            is_head : false,
            is_leaf : false,
            children : Vec::new()
        }
    }

    pub fn add_child(&mut self, node : Node) -> bool {
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

    pub fn get_all_data(&self){
        print!("Data name: '{}' Held information -> ", self.data.get_name());
        match self.data.get_value() {
            DataType::String(v) => println!("{}", v),
            DataType::Integer(v)  => println!("{}", v),
            DataType::Float(v)  => println!("{}", v)
        }
        stdout().flush();
        &self.children.iter().for_each(|child| child.get_all_data());
    }
}
