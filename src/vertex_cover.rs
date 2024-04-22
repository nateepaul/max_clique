pub mod vertex_cover_algorithm {
    pub fn perform(graph: &Vec<Vec<usize>>) -> Vec<usize> {
        let mut comp = vec![vec![0; graph.len()]; graph.len()];
        
        for i in 0..graph.len() {
            for j in 0..graph.len() {
                if i != j && graph[i][j] == 0 {
                    comp[i][j] = 1;
                }
            }
        }


        let size = graph.len(); // set size equal to graph length
        let mut cover: Vec<usize> = Vec::new(); // initilize empty cover
        let mut visited = vec![0; size]; // initlize vector with same size as graph of all false values

        for u in 0..graph.len() {
            if visited[u] == 0 {
                visited[u] = 1;
                for &v in &comp[u] {
                    if visited[v] == 0 {
                        visited[v] = 1;
                        visited[u] = 1;
                        if !cover.contains(&u) {
                            cover.push(u);
                        }
                        cover.push(v);
                        break;
                    } 
                }
            }
        }

        cover
    }
}