# MerkleTree
This is an adaptation of Merkle Trees for a github commit-like implementation.
<img alt="not a merkle tree" src="https://github.com/KNaudin/MerkleTree/blob/master/images/merkle%20tree.png" width="500" height="400">
> Yes, I was bored

## What it does
Merkle trees are a type of binary tree that compute a hash to verify the integrity of a sequence.
In this project, each Tree contains branches made of nodes.
Branches are a HasmMap<String, Rc<RefCell<Node>>> (this is a smart pointer in Rust, don't be scared). The node linked
is always the head of the branch.
A node contains some data that can be a string, a float or an int. The node also knows its hash and its children if it has any.
If a node has any children, it will compute its hash according to their data. Else, it will compute from its own data.
<img alt="not a merkle tree" src="https://github.com/KNaudin/MerkleTree/blob/master/images/merkletreehash.png">

## Example
The following code
```
let first_node = Node::new(Data::new(String::from("My age"), DataType::Integer(21)), String::from("Added my age."));
let second_node = Node::new(Data::new(String::from("A float"), DataType::Float(3.14)), String::from("Defined PI."));
let third_node = Node::new(Data::new(String::from("This is a"), DataType::String(String::from("Working tree."))), String::from("Commented line"));

let mut merkle_tree = Tree::new(first_node);
merkle_tree.push(second_node, String::from("master"));
merkle_tree.push(third_node, String::from("master"));

merkle_tree.print_all_data(String::from("master"));
```
Will output:
```
Branch master
Commit 'Commented line'
Data name: 'This is a' Held information -> Working tree. hash: 8339961e5dc9397c0316f8e60a3f5663
Commit 'Defined PI.'
Data name: 'A float' Held information -> 3.14 hash: 4beed3b9c4a886067de0e3a094246f78
Commit 'Added my age.'
Data name: 'My age' Held information -> 21 hash: 3c59dc048e8850243be8079a5c74d079
```
