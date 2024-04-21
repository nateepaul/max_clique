mod brute_force;
mod original;
mod vertex_cover;
mod file_handling;

pub use crate::brute_force::brute_force_algorithm;
pub use crate::original::original_algorithm;
pub use crate::vertex_cover::vertex_cover_algorithm;
pub use crate::file_handling::file_operations;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // get command line arguments

    if args.len() != 3 { // if there are the wrong number of arguments
        println!("Wrong Arguments: Expected usage is a command \"BF, OG, VC\" to
        call the brute force, orginal, and vertex cover algorithims followed by name of the
        graph file");
        return;
    }

    let file_path = &args[1]; // file path is first argument
    let command = &args[2]; // command is second argument

    let graph: Vec<Vec<usize>>; // create graph vector

    if let Err(graph) = file_operations::read_graph_file(file_path) {
        println!("{}", file_path);
        println!("Error reading file. Check file name and contents.");
        return;
    }

    if command == "BF" { // If brute force command perform brute force algorithm
        brute_force_algorithm::perform();
    }

    else if command == "OG" { // If original command perform original algorithm
        original_algorithm::perform();
    }

    else if command == "VC" { // If brute force command perform brute force algorithm
        vertex_cover_algorithm::perform();
    }

    else { // Else print correct usage
        println!("Wrong Arguments: Expected usage is a command \"BF, OG, VC\" to
        call the brute force, orginal, and vertex cover algorithims followed by name of the
        graph file");
        return;
    }
}
