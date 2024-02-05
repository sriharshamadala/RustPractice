use std::cmp::Ordering;

/// Currently we do not implement any error handling.
pub struct UnionFind {
    root: Vec<usize>,
    rank: Vec<usize>,
    nf_disjoint_sets: usize
}

impl UnionFind {
    pub fn new(nf_vertices: usize) -> Self {
        Self {
            root: vec![0; nf_vertices].iter().enumerate().map(|(index, _)| index).collect(),
            rank: vec![0; nf_vertices],
            nf_disjoint_sets: nf_vertices
        }
    }

    /// Returns the root of the vertex, not necessarily the parent
    /// Using path compression technique, hence the recursive call
    pub fn find(&mut self, vertex: usize) -> usize {
        let curr_root = self.root[vertex];
        if  curr_root != vertex {
            self.root[vertex] = self.find(curr_root);
        }

        self.root[vertex]
    }

    /// Performs quick union
    /// Additional optimization is union by rank
    pub fn union(&mut self, vertex_1: usize, vertex_2: usize) {
        let root_1 = self.find(vertex_1);
        let root_2 = self.find(vertex_2);

        if root_1 != root_2 {
            // select the new root based on the rank
            match (self.rank[root_1]).cmp(&self.rank[root_2]) {
                Ordering::Equal => {
                    self.root[root_2] = root_1;
                    self.rank[root_1] += 1;
                },
                Ordering::Greater => self.root[root_2] = root_1,
                Ordering::Less => self.root[root_1] = root_2,
            }

            self.nf_disjoint_sets -= 1;
        }
    } 

    pub fn is_connected(&mut self, vertex_1: usize, vertex_2: usize) -> bool {
        self.find(vertex_1) == self.find(vertex_2)
    }

    pub fn get_number_of_disjoint_sets(&self) -> usize {
        self.nf_disjoint_sets
    }

}

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn initialize() {
        let mut disjoint_set = UnionFind::new(1);
        assert!(disjoint_set.is_connected(0, 0));
        assert_eq!(disjoint_set.get_number_of_disjoint_sets(), 1);
    }

    /// This allows creation of disjoint set from a list of edges
    fn disjoint_set_from_vec(nf_vertices: usize, edges: Vec<Vec<usize>>) -> UnionFind {
        let mut disjoint_set = UnionFind::new(nf_vertices);

        for edge in edges {
            disjoint_set.union(edge[0], edge[1]);
        }

        disjoint_set
    }

    #[test]
    fn create_from_list() {
        let edges = vec![ vec![0, 1], vec![0, 2], vec![1, 6], vec![6, 7], vec![3, 4]];

        let mut disjoint_set = disjoint_set_from_vec(8, edges);

        assert!(disjoint_set.is_connected(2, 7));
        assert_eq!(disjoint_set.get_number_of_disjoint_sets(), 3);
    }

    #[test]
    fn join() {
        let edges = vec![ vec![0, 1], vec![0, 2], vec![1, 6], vec![6, 7], vec![3, 4]];

        let mut disjoint_set = disjoint_set_from_vec(8, edges);
        
        // Add a new edge between 2, 3, that merges the two disjoint sets
        disjoint_set.union(2, 3);

        assert!(disjoint_set.is_connected(4, 7));
        assert!(!disjoint_set.is_connected(5, 7));
        assert_eq!(disjoint_set.get_number_of_disjoint_sets(), 2);
    }
}