use std::collections::{HashMap, VecDeque, HashSet};
use std::error::Error;
use std::fs::File;
use csv::{ReaderBuilder, Trim};

// Define the Graph type as a HashMap where each key is a String representing a node ID,
// and the value is a Vec<String> of node IDs representing connections to other nodes.
pub type Graph = HashMap<String, Vec<String>>;

// Function to read graph data from a CSV file and construct a Graph structure.
pub fn read_data(filename: &str) -> Result<Graph, Box<dyn Error>> {
    // Open the specified file.
    let file = File::open(filename)?;
    // Set up a CSV reader with appropriate settings for headers and trimming.
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .trim(Trim::All)
        .from_reader(file);

    // Initialize an empty graph.
    let mut graph = Graph::new();

    // Iterate over each record in the CSV file.
    for result in rdr.records() {
        let record = result?; // If there's an error reading a record, it will return early.
        let id = record[0].trim().to_string(); // Extract the node ID.
        let friends_str = record[9].trim(); // Extract the string of friends from the last field.

        // Clean the friends string by removing unwanted characters and splitting into individual IDs.
        let friends_cleaned = friends_str
            .trim_matches(|c: char| c == '[' || c == ']' || c == '"')
            .replace("\"", "");

        let friends: Vec<String> = friends_cleaned
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        // Ensure every node appears in the graph, even if it has no friends.
        graph.entry(id.clone()).or_default();
        // Add edges both ways since this is an undirected graph.
        for friend in friends {
            let friend_clone = friend.clone(); // Clone to use in multiple entries.
            graph.entry(friend_clone).or_default().push(id.clone());
            graph.entry(id.clone()).or_default().push(friend);
        }
    }

    // Return the graph if all went well.
    Ok(graph)
}

// Function to perform a breadth-first search (BFS) starting from a specified node.
pub fn bfs(graph: &Graph, start: &str) -> HashMap<String, usize> {
    let mut distances = HashMap::new(); // Store distances from the start node.
    let mut queue = VecDeque::new(); // Use a queue to manage the nodes to visit next.
    let mut visited = HashSet::new(); // Track visited nodes to avoid revisiting.

    // Initialize the BFS with the start node.
    visited.insert(start.to_string());
    queue.push_back((start.to_string(), 0)); // Start with distance 0.

    // Process the queue until empty.
    while let Some((current, dist)) = queue.pop_front() {
        distances.insert(current.clone(), dist); // Record the distance for the current node.

        // Get all neighbors of the current node.
        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                // Only consider unvisited neighbors to avoid cycles.
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.to_string());
                    queue.push_back((neighbor.to_string(), dist + 1)); // Enqueue with incremented distance.
                }
            }
        }
    }

    // Return all computed distances.
    distances
}
