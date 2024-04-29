use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::algo::dijkstra;
use petgraph::dot::{Dot, Config};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
pub fn build_graph(players: &[crate::models::Player]) -> UnGraph<String, ()> {
    let mut graph = UnGraph::new_undirected();
    let mut team_map: HashMap<String, Vec<NodeIndex>> = HashMap::new();
    for player in players {
        let player_node = graph.add_node(player.Player.clone());
        if let Some(peers) = team_map.get_mut(&player.Tm) {
            for &peer in peers.iter() {
                graph.add_edge(peer, player_node, ());
            }
            peers.push(player_node);
        } else {
            team_map.insert(player.Tm.clone(), vec![player_node]);
        }
    }
    graph
}
//This function builds an undirected graph (UnGraph) with nodes representing players.
//Each player is associated with a team, and all players from the same team are connected in the graph. 
//The players and their teams are taken from the players slice. The team_map hashmap tracks which nodes (players) belong to which team.
//For each player, a node is added to the graph.
//If other players are already in the graph from the same team, edges are added between the new player and all previous team members.
//The function returns the populated graph.
pub fn calculate_average_distance(graph: &UnGraph<String, ()>) -> f64 {
    let mut total_distance = 0.0;
    let mut count: f64 = 0.0; 
    for node in graph.node_indices() {
        let path_lengths = dijkstra(graph, node, None, |_| 1);
        for &length in path_lengths.values() {
            if length < usize::MAX {
                total_distance += length as f64;
                count += 1.0;
            }
        }
    }
    total_distance / count.max(1.0) 
}
//This function calculates the average shortest path distance between all pairs of nodes in the graph. It uses Dijkstra's algorithm for finding the shortest paths.
//The function iterates over each node, computes the shortest path to all other nodes, and aggregates these distances.
//It handles the possibility of infinite distances (i.e., disconnected nodes) by checking if the length is less than usize::MAX.
//The average distance is calculated by dividing the total distance by the number of valid paths.
pub fn export_graph(graph: &UnGraph<String, ()>, filename: &str) {
    let file = File::create(filename).expect("Unable to create file");
    let mut writer = std::io::BufWriter::new(file);
    write!(writer, "{:?}", Dot::with_config(graph, &[Config::EdgeNoLabel])).expect("Failed to write graph");
}
//This function writes the graph structure to a file in DOT format.
//A file is created with the specified filename.
//A buffered writer is used for efficient file writing.
//The graph is exported without labeling the edges, using Dot::with_config and Config::EdgeNoLabel to format the graph.
// After that, Use Graphviz  and the dot file to create a graph

//Overall , This module are calculate the average distance of the nodes and get the data graph of nodes.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Player;

    #[test]
    fn test_build_graph_empty() {
        let players: Vec<Player> = vec![];
        let graph = build_graph(&players);
        assert!(graph.node_count() == 0);
        assert!(graph.edge_count() == 0);
    }

    #[test]
    fn test_build_graph_single_team() {
        let players = vec![
            Player { Player: "Player A".to_string(), Tm: "Team 1".to_string() },
            Player { Player: "Player B".to_string(), Tm: "Team 1".to_string() },
        ];
        let graph = build_graph(&players);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_build_graph_multiple_teams() {
        let players = vec![
            Player { Player: "Player A".to_string(), Tm: "Team 1".to_string() },
            Player { Player: "Player B".to_string(), Tm: "Team 2".to_string() },
        ];
        let graph = build_graph(&players);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 0);
    }
}

#[test]
fn test_average_distance_empty_graph() {
    let graph: UnGraph<String, ()> = UnGraph::new_undirected();
    assert_eq!(calculate_average_distance(&graph), 0.0);
}

#[test]
fn test_average_distance_single_node() {
    let mut graph = UnGraph::new_undirected();
    graph.add_node("Solo Node".to_string());
    assert_eq!(calculate_average_distance(&graph), 0.0);
}

#[test]
fn test_average_distance_connected_nodes() {
    let mut graph = UnGraph::<String, ()>::new_undirected();
    let a = graph.add_node("Node A".to_string());
    let b = graph.add_node("Node B".to_string());
    graph.add_edge(a, b, ());

    // Calculate the average distance
    let avg_distance = calculate_average_distance(&graph);
    assert_eq!(avg_distance, 0.5, "The average distance should be 1.0");
}




