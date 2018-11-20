mod data;
use data::*;

fn main() {
    let num_data = Data::new(5);
    println!("Numeric data contains {}", num_data.get_value());
}
