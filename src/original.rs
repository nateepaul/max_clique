pub mod original_algorithm {
    use std::collections::HashSet;

    pub fn perform(graph: &Vec<Vec<usize>>) -> Vec<usize> {
        let size = graph.len();
        let mut clique = Vec::new();
        let mut remaining: HashSet<usize> = (0..size).collect();

        let mut ordered_vertices: Vec<usize> = (0..size).collect();
        ordered_vertices.sort_by_key(|&v| graph[v].iter().sum::<usize>());

        for &v in ordered_vertices.iter().rev() {
            if remaining.contains(&v) {
                if clique.iter().all(|&u| graph[v][u] == 1) {
                    clique.push(v);

                    remaining.retain(|&u| graph[v][u] == 1);
                    remaining.remove(&v);
                }
            }
        }

        clique
    }
}