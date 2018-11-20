mod node;
mod data;

use data::*;
use node::*;

fn main() {

    let first_node = Node::new(Data::new(String::from("My age"), DataType::Integer(21)));
    let twin_node = Node::new(Data::new(String::from("My height"), DataType::Integer(178)));
    let mut second_node = Node::new(Data::new(String::from("A float"), DataType::Float(3.14)));
    let mut third_node = Node::new(Data::new(String::from("This is a"), DataType::String(String::from("Working tree."))));

    second_node.add_child(first_node);
    second_node.add_child(twin_node);
    third_node.add_child(second_node);

    third_node.get_all_data();
}
