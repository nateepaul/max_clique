pub mod file_operations {
    use std::fs::File;
    use std::io::{self, BufRead, Write};

    pub fn read_graph_file(file_path: &str) -> io::Result<Vec<Vec<usize>>> {
        let file = File::open(file_path)?; // open file with given path
        let reader = io::BufReader::new(file); // create file reader
        
        let mut matrix: Vec<Vec<usize>> = Vec::new(); // initialize matrix
        
        for line in reader.lines() { // loop through lines in file using lines()
            let row: Vec<usize> = line? // put lines into row of vector
                .split_whitespace() // split lines by whitespace
                .skip(1) // skip the first number (node number)
                .map(|s| s.parse().unwrap()) // parse each substring into a usize
                .collect(); // collect values into row
            matrix.push(row); // add row into matrix
        }
  
        Ok(generate_symmetric_matrix(&matrix)) // return matrix if successful
    }

    pub fn write_clique_to_file(clique: &Vec<usize>) -> io::Result<()> {
        let filename = format!("S{}_nepaul.sol", clique.len());
        
        let mut file = File::create(filename)?; // create file with file name
        for node in clique { // loop through nodes in best path
            writeln!(&mut file, "{}", node)?; // write the node to the file
        }
        Ok(()) // check for error
    }
    
    fn generate_symmetric_matrix(lower_half: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let size = lower_half.len(); // set size to length of given matrix
        let mut symmetric_matrix = vec![vec![0; size]; size]; // create new matrix with values initialized as zero
    
        for i in 0..size {
            for j in 0..i {
                symmetric_matrix[i][j] = lower_half[i][j]; // copy lower half value to lower half of new
                symmetric_matrix[j][i] = lower_half[i][j]; // copy lower half value to upper half of new
            }
        }
    
        symmetric_matrix // return symmetric matrix
    }
}