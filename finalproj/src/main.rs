// Import the necessary modules that manage graph structures and their analysis.
mod graph;
mod analysis;

fn main() {
    // Attempt to read the graph data from a CSV file.
    let graph = match graph::read_data("data.csv") {
        Ok(graph) => graph,  // If successful, use the graph for further processing.
        Err(e) => {
            // If there is an error during file reading, report it and exit.
            println!("Failed to read data: {}", e);
            return;
        }
    };
    //The above variable stores the entire graph data.
    // Print the total number of nodes present in the graph.
    println!("Number of nodes in the graph: {}", graph.len());

    // Calculate and display various metrics about the graph.
    let (max, min, median, std_dev, avg) = analysis::graph_metrics(&graph);
    println!("Max path length: {}", max);
    println!("Min path length: {}", min);
    println!("Median path length: {:.2}", median);
    println!("Standard deviation: {:.2}", std_dev);
    println!("Average distance: {:.2}", avg);

    // Calculate and display the degree distribution for each node.
    let degrees = analysis::degree_distribution(&graph);
    println!("Degree Distribution: {:?}", degrees);

    // Calculate and display the degree distribution for nodes within two steps.
    let degrees_at_2 = analysis::degree_distribution_at_distance_2(&graph);
    println!("Degree Distribution at Distance 2: {:?}", degrees_at_2);

    // Calculate and display the average degree for nodes at distance 1 and 2.
    let (average_degree, average_degree_2) = analysis::calculate_average_degrees(&graph);
    println!("Average degree at distance 1: {:.2}", average_degree);
    println!("Average degree at distance 2: {:.2}", average_degree_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    // I've set up basic tests to ensure each function behaves as expected under controlled scenarios.
    #[test]
    fn test_read_data() {
        // This test checks if the function can read data successfully, assuming no file I/O issues.
        assert!(read_data().is_ok());
    }

    #[test]
    fn test_bfs() {
        // Testing BFS to ensure it correctly calculates distances from a start node.
        let mut graph = Graph::new();
        graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
        graph.insert("B".to_string(), vec!["A".to_string(), "D".to_string()]);
        graph.insert("C".to_string(), vec!["A".to_string()]);
        graph.insert("D".to_string(), vec!["B".to_string()]);

        let distances = bfs(&graph, "A");
        assert_eq!(distances.get("A"), Some(&0));
        assert_eq!(distances.get("B"), Some(&1));
        assert_eq!(distances.get("C"), Some(&1));
        assert_eq!(distances.get("D"), Some(&2));
    }

    #[test]
    fn test_graph_metrics() {
        // Ensure the graph metrics function returns correct values for a simple connected graph.
        let mut graph = Graph::new();
        graph.insert("A".to_string(), vec!["B".to_string()]);
        graph.insert("B".to_string(), vec!["A".to_string(), "C".to_string()]);
        graph.insert("C".to_string(), vec!["B".to_string()]);

        let (max, min, median, std_dev, avg) = graph_metrics(&graph);
        assert_eq!(max, 1);
        assert_eq!(min, 1);
        assert_eq!(median, 1.0);
        assert_eq!(std_dev, 0.0);
        assert!(avg > 0.0);
    }

    #[test]
    fn test_degree_distribution() {
        // Check if the degree distribution correctly identifies the number of connections for each node.
        let mut graph = Graph::new();
        graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
        graph.insert("B".to_string(), vec!["A".to_string()]);
        graph.insert("C".to_string(), vec!["A".to_string()]);

        let distribution = degree_distribution(&graph);
        assert_eq!(distribution.get(&2), Some(&1)); // A has 2 connections
        assert_eq!(distribution.get(&1), Some(&2)); // B and C have 1 connection each
    }

    #[test]
    fn test_degree_distribution_at_distance_2() {
        // Verify that the degree at distance 2 is calculated correctly, accounting for unique connections.
        let mut graph = Graph::new();
        graph.insert("A".to_string(), vec!["B".to_string()]);
        graph.insert("B".to_string(), vec!["A".to_string(), "C".to_string()]);
        graph.insert("C".to_string(), vec!["B".to_string()]);

        let distribution = degree_distribution_at_distance_2(&graph);
        assert_eq!(distribution.get(&0), Some(&2)); // A and C can only reach B within 2 steps
        assert_eq!(distribution.get(&2), Some(&1)); // B can reach both A and C
    }

    #[test]
    fn test_calculate_average_degrees() {
        // Ensure that the function correctly computes the average degrees at distance 1 and 2.
        let mut graph = Graph::new();
        graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
        graph.insert("B".to_string(), vec!["A".to_string()]);
        graph.insert("C".to_string(), vec!["A".to_string()]);

        let (average_degree, average_degree_2) = calculate_average_degrees(&graph);
        assert!(average_degree > 0.0);
        assert!(average_degree_2 > 0.0);
    }
}
