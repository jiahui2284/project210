use petgraph::graph::NodeIndex;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use std::collections::BTreeMap;
pub fn calculate_closeness_centrality(graph: &UnGraph<String, ()>) -> BTreeMap<NodeIndex, f64> {
    let mut centralities = BTreeMap::new();
    let node_count = graph.node_count() as f64;
    for node in graph.node_indices() {
        let path_lengths = dijkstra(graph, node, None, |_| 1);
        let total_distance: usize = path_lengths.values().filter_map(|&d| if d < usize::MAX { Some(d) } else { None }).sum();
        if total_distance > 0 {
            let centrality = (node_count - 1.0) / total_distance as f64;
            centralities.insert(node, centrality);
        }
    }
    centralities
}
//The function “calculate_closeness_centrality calculates” the closeness centrality for each node in an undirected graph “ (UnGraph<String, ()>)”, where each node is identified by a NodeIndex and edges have no weight (()):
//Initialize the Centralities Map: A “BTreeMap” is created to store the centrality values of each node, indexed by their “NodeIndex”.
//Total Nodes: The total number of nodes “(node_count)” in the graph is recorded, which is used in the centrality calculation.
//Iterate Over Each Node: The function iterates over each node in the graph. For each node, it:
//Uses Dijkstra's algorithm to find the shortest path from the current node to all other nodes. The path cost is set uniformly to 1 for all edges.
//Filters and sums the distances to determine the total distance “(total_distance)” from the current node to all other reachable nodes.
//Calculate Centrality: If “total_distance” is greater than 0, it calculates the centrality for the node using the formula (node_count−1)/total_distance. This formula measures how close a node is to all other nodes, with a higher value indicating greater centrality.
//Store Centrality Value: The centrality value is then stored in the “centralities” map.

pub fn find_representatives(graph: &UnGraph<String, ()>, k: usize) -> Vec<NodeIndex> {
    let centralities = calculate_closeness_centrality(graph);
    let mut sorted_nodes: Vec<_> = centralities.iter().collect();
    sorted_nodes.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    sorted_nodes.iter().take(k).map(|(&node, _)| node).collect()
}
//The function “find_representatives” finds the “k” most central nodes from the graph:
//Calculate Centralities: It first calculates the closeness centrality of each node using the previously described function.
//Sort Nodes by Centrality: It collects these centrality values into a vector, sorts this vector in descending order based on the centrality values, ensuring the nodes with the highest centrality are at the front.
//Select Top k Nodes: Finally, it takes the first k elements from this sorted list, extracting just the “NodeIndex” for each, to get the list of nodes that are most representative of the graph's structure based on their centrality



#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::UnGraph;
    fn create_test_graph() -> UnGraph<String, ()> {
        let mut graph = UnGraph::<String, ()>::new_undirected();
        let n1 = graph.add_node("Node 1".to_string());
        let n2 = graph.add_node("Node 2".to_string());
        let n3 = graph.add_node("Node 3".to_string());

        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        graph
    }

    #[test]
    fn test_calculate_closeness_centrality() {
        let graph = create_test_graph();
        let centralities = calculate_closeness_centrality(&graph);

        assert_eq!(centralities.len(), graph.node_count());
    }

    #[test]
    fn test_find_representatives() {
        let graph = create_test_graph();
        let representatives = find_representatives(&graph, 2);

        assert_eq!(representatives.len(), 2);
    }
}

