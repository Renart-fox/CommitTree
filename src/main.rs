mod node;
mod data;
mod tree;

use data::*;
use node::*;
use tree::*;

fn main() {

    let first_node = Node::new(Data::new(String::from("My age"), DataType::Integer(21)));
    let second_node = Node::new(Data::new(String::from("A float"), DataType::Float(3.14)));
    let third_node = Node::new(Data::new(String::from("This is a"), DataType::String(String::from("Working tree."))));

    let mut merkle_tree = Tree::new(first_node);
    merkle_tree.add(second_node);
    merkle_tree.add(third_node);

    merkle_tree.print_all_data();
}
