use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::IntoNeighbors;
use std::collections::{HashMap, HashSet};
fn collect_neighbors(graph: &UnGraph<String, ()>) -> HashMap<NodeIndex, HashSet<NodeIndex>> {
    graph.node_indices()
        .map(|node| (node, graph.neighbors(node).collect()))
        .collect()
}
//This function collects and returns a mapping from each node in the graph to a set of its immediate neighbors. 
//This structure facilitates quick lookup of neighbors for any node in the graph and is crucial for calculating node similarity.
fn calculate_similarity(neighbors_u: &HashSet<NodeIndex>, neighbors_v: &HashSet<NodeIndex>) -> f64 {
    let intersection_count = neighbors_u.intersection(neighbors_v).count() as f64;
    let union_count = neighbors_u.union(neighbors_v).count() as f64;
    if union_count == 0.0 {
        0.0
    } else {
        intersection_count / union_count
    }
}
//This function computes the Jaccard similarity coefficient between two sets of neighbors, which is the ratio of the intersection to the union of the sets. 
//This metric is commonly used to measure the similarity between two data sets, and in this context, it quantifies the similarity between two nodes based on their shared neighbors
fn update_extremes(u: NodeIndex, v: NodeIndex, similarity: f64, most_similar: &mut (NodeIndex, NodeIndex, f64), most_dissimilar: &mut (NodeIndex, NodeIndex, f64)) {
    if similarity > most_similar.2 {
        *most_similar = (u, v, similarity);
    }
    if similarity < most_dissimilar.2 {
        *most_dissimilar = (u, v, similarity);
    }
}
//This function updates the most similar and most dissimilar node pairs based on the computed similarity. 
//It checks if the current pair’s similarity is higher or lower than the recorded extremes and updates them accordingly.
pub fn find_extreme_similarity(graph: &UnGraph<String, ()>) -> ((NodeIndex, NodeIndex, f64), (NodeIndex, NodeIndex, f64)) {
    if graph.node_count() < 2 {
        panic!("Graph must contain at least two nodes to compare similarities.");
    } 
    let neighbor_sets = collect_neighbors(graph);
    let mut most_similar = (NodeIndex::new(0), NodeIndex::new(1), 0.0);
    let mut most_dissimilar = (NodeIndex::new(0), NodeIndex::new(1), 1.0);

    for (&u, neighbors_u) in &neighbor_sets {
        for (&v, neighbors_v) in &neighbor_sets {
            if u != v {
                let similarity = calculate_similarity(neighbors_u, neighbors_v);
                update_extremes(u, v, similarity, &mut most_similar, &mut most_dissimilar);
            }
        }
    }  
    (most_similar, most_dissimilar)
}
//This pub function orchestrates the process to find the most similar and most dissimilar node pairs in the entire graph.
//First, it ensures the graph has at least two nodes to perform comparisons.
//It then collects neighbor sets for all nodes.
//It initializes the most similar and most dissimilar records with dummy values (nodes 0 and 1 with similarities of 0.0 and 1.0 respectively).
//The function iterates over all pairs of different nodes, calculates their similarity, and updates the records for the most extreme similarities.
//Finally, it returns these extremes.


#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::node_index;

    #[test]
    fn test_calculate_similarity_empty_sets() {
        let set1: HashSet<NodeIndex> = HashSet::new();
        let set2: HashSet<NodeIndex> = HashSet::new();
        assert_eq!(calculate_similarity(&set1, &set2), 0.0);
    }

    #[test]
    fn test_calculate_similarity_no_overlap() {
        let set1: HashSet<NodeIndex> = [node_index(1)].iter().cloned().collect();
        let set2: HashSet<NodeIndex> = [node_index(2)].iter().cloned().collect();
        assert_eq!(calculate_similarity(&set1, &set2), 0.0);
    }

    #[test]
    fn test_calculate_similarity_complete_overlap() {
        let set1: HashSet<NodeIndex> = [node_index(1), node_index(2)].iter().cloned().collect();
        let set2: HashSet<NodeIndex> = [node_index(1), node_index(2)].iter().cloned().collect();
        assert_eq!(calculate_similarity(&set1, &set2), 1.0);
    }

    #[test]
    fn test_calculate_similarity_partial_overlap() {
        let set1: HashSet<NodeIndex> = [node_index(1), node_index(2)].iter().cloned().collect();
        let set2: HashSet<NodeIndex> = [node_index(2), node_index(3)].iter().cloned().collect();
        assert_eq!(calculate_similarity(&set1, &set2), 1.0 / 3.0);
    }

    #[test]
    fn test_find_extreme_similarity_simple() {
        let mut graph = UnGraph::<String, ()>::new_undirected();
        let n1 = graph.add_node("Node 1".to_string());
        let n2 = graph.add_node("Node 2".to_string());
        let n3 = graph.add_node("Node 3".to_string());
        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());

        let (most_similar, most_dissimilar) = find_extreme_similarity(&graph);
        assert!(most_similar.2 > 0.5);  
        assert!(most_dissimilar.2 < 0.5);  
    }
}




