use csv::Reader;
use petgraph::algo::{connected_components, dijkstra};
use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::visit::IntoNodeIdentifiers;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Player {
    #[serde(rename = "PLAYER")]
    player_name: String,
    #[serde(rename = "TEAM_pie")]
    team: String,
}

fn read_data(path: &str) -> Result<Vec<Player>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut players = Vec::new();
    for result in rdr.deserialize() {
        let player: Player = result?;
        players.push(player);
    }
    Ok(players)
}

fn create_graph(players: &[Player]) -> UnGraph<String, &'static str> {
    let mut graph = UnGraph::new_undirected();
    let mut indices: HashMap<String, NodeIndex> = HashMap::new();

    // Create nodes for each player
    for player in players {
        indices.entry(player.player_name.clone())
            .or_insert_with(|| graph.add_node(player.player_name.clone()));
    }

    // Create edges based on team affiliation
    let mut team_map: HashMap<String, Vec<NodeIndex>> = HashMap::new();
    for player in players {
        let node = indices[&player.player_name];
        team_map.entry(player.team.clone())
            .or_default()
            .push(node);
    }

    for teammates in team_map.values() {
        for (i, &teammate1) in teammates.iter().enumerate() {
            for &teammate2 in &teammates[i + 1..] {
                graph.add_edge(teammate1, teammate2, "teammate");
            }
        }
    }

    graph
}

// Find connected components in the graph
fn find_connected_components(graph: &UnGraph<String, &'static str>) -> usize {
    connected_components(graph)
}

// Compute closeness centrality for each node
fn compute_closeness_centrality(graph: &UnGraph<String, &'static str>) -> HashMap<NodeIndex, f64> {
    let mut centrality = HashMap::new();
    let node_count = graph.node_count() as f64;

    for node in graph.node_identifiers() {
        let path_lengths = dijkstra(graph, node, None, |_| 1);
        let total_path_length: usize = path_lengths.values().map(|&d| d).sum();
        let closeness = if total_path_length > 0 {
            (node_count - 1.0) / total_path_length as f64
        } else {
            0.0
        };
        centrality.insert(node, closeness);
    }

    centrality
}

fn main() -> Result<(), Box<dyn Error>> {
    let players = read_data("nba.csv")?;
    let graph = create_graph(&players);

    let num_components = find_connected_components(&graph);
    println!("Number of connected components: {}", num_components);

    let centrality = compute_closeness_centrality(&graph);
    for (node, value) in centrality {
        println!("Node {}: Closeness Centrality = {}", graph[node], value);
    }

    Ok(())
}


