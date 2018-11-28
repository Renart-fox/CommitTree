mod structs;

use structs::data::*;
use structs::node::*;
use structs::tree::*;

fn main() {
    // Creating 4 nodes
    let first_node = Node::new(Data::new(String::from("My age"), DataType::Integer(21)), String::from("Added my age."));
    let second_node = Node::new(Data::new(String::from("A float"), DataType::Float(3.14)), String::from("Defined PI."));
    let third_node = Node::new(Data::new(String::from("This is a"), DataType::String(String::from("Working tree."))), String::from("Commented line"));
    let fourth_node = Node::new(Data::new(String::from("Random number"), DataType::Integer(21)), String::from("New branch!"));
    let fifth_node = Node::new(Data::new(String::from("My age"), DataType::Integer(22)), String::from("New version is out."));
    let sixth_node = Node::new(Data::new(String::from("This is the number 5"), DataType::Integer(5)), String::from("Number 5 baybee"));
    let seventh_node = Node::new(Data::new(String::from("A good movie"), DataType::String("7".to_string())), String::from("I love Freeman"));


    // Create the tree with the first node as the 'master' head
    let mut merkle_tree = Tree::new(first_node);
    merkle_tree.push(second_node, String::from("master"));
    merkle_tree.push(third_node, String::from("master")); // At this point, the third node is the 'master' head

    // Error because the dev branch doesn't exist [1]
    let error = merkle_tree.push(fourth_node.clone(), String::from("dev"));
    println!("{}", error);

    // Last used branch was 'master' so the 'dev' branch will be set from the 'master' branch
    merkle_tree.new_branch(fourth_node, String::from("dev"));
    merkle_tree.new_branch(fifth_node, String::from("v0.2.0"));

    merkle_tree.push(sixth_node, String::from("master"));
    merkle_tree.new_branch(seventh_node, String::from("useless_test_branch"));

    // Display the 'dev' branch [2]
    merkle_tree.print_all_data(String::from("dev"));

    // Any commit with this hash will be removed.
    // All parent commits will recompute their hash !
    merkle_tree.remove(String::from("3c59dc048e8850243be8079a5c74d079"));

    // Note that the hash are different
    // Note that more than 1 commit can be a head
    merkle_tree.print_all_data(String::from("master"));
    merkle_tree.print_all_data(String::from("dev"));
    merkle_tree.display_branches();
}
