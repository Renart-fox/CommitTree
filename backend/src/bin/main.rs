extern crate MerkleTree;

mod structs;

use MerkleTree::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::collections::HashMap;
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

    // Server implementation on port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // GET requests
    let get = b"GET / HTTP/1.1\r\n";

    // POST requests
    let add_node = b"POST /add_node HTTP/1.1\r\n";
    let get_all_nodes = b"POST /get_all_nodes HTTP/1.1\r\n";

    // This deals with GET requests but we don't use them
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // If we're dealing with post requests, this will recover the data
    // Else it will recover some HTTP data we don't care about
    let request_content = String::from_utf8_lossy(&buffer[..]);
    let mut request_split = request_content.split("\r\n").collect::<Vec<_>>();
    let mut post_data = request_split[request_split.len()-1];
    // Now we can deal with post requests
    let (status_line, serialized_answer) = if buffer.starts_with(add_node) {
        // This needs a branch name, a data, the data name and a commit message to work

        ("HTTP/1.1 200 OK\r\n\r\n", "Trust me this is a serialized JSON bro\n")
    }else if buffer.starts_with(get_all_nodes){
        ("HTTP/1.1 200 OK\r\n\r\n", "Displaying all nodes\n")
    }else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut contents = String::new();

    if buffer.starts_with(get) {
        let mut file = File::open(filename).unwrap();

        file.read_to_string(&mut contents).unwrap();
    }
    else{
        contents = serialized_answer.to_string();
    }

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_data(raw_data: String) -> HashMap<String, String>{
    let mut data = HashMap::new();
    /* Regex to find parameters in the data
     /([a-zA-Z0-9]+=(.)+?)(?=\&[a-zA-Z0-9]+=)/g
    */
}