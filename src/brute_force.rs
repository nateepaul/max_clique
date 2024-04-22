pub mod brute_force_algorithm {
    pub fn perform(graph: &Vec<Vec<usize>>) -> Vec<usize> {
        let size = graph.len();
        let mut max_clique: Vec<usize> = Vec::new();
    
        for i in 0..(1 << size) {
            let mut clique: Vec<usize> = Vec::new();
    
            for j in 0..size {
                if i & (1 << j) != 0 {
                    clique.push(j);
                }
            }

            if is_clique(graph, &clique) && clique.len() > max_clique.len() {
                max_clique = clique;
            }
        }
    
        max_clique
    }

    fn is_clique(graph: &Vec<Vec<usize>>, vertices: &Vec<usize>) -> bool {
        for &i in vertices {
            for &j in vertices {
                if i != j && graph[i][j] == 0 {
                    return false;
                }
            }
        }
        
        true
    }
}