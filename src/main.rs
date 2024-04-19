mod brute_force;
mod original;
mod vertex_cover;
mod file_handling;

pub use crate::brute_force::brute_force_algorithm;
pub use crate::original::original_algorithm;
pub use crate::vertex_cover::vertex_cover_algorithm;
pub use crate::file_handling::file_operations;

use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect(); // get command line arguments

    if args.len() != 3 { // if there are the wrong number of arguments
        println!("Wrong Arguments: Expected usage is a command \"BF, OG, VC\" to
        call the brute force, orginal, and vertex cover algorithims followed by name of the
        graph file");
        return;
    }

    let file_path = &args[1];
    let command = &args[2];

    if command == "BF" {
        brute_force_algorithm::perform();
    }

    else if command == "OG" {
        original_algorithm::perform();
    }

    else if command == "VC" {
        vertex_cover_algorithm::perform();
    }

    else {
        println!("Wrong Arguments: Expected usage is a command \"BF, OG, VC\" to
        call the brute force, orginal, and vertex cover algorithims followed by name of the
        graph file");
    }
}
