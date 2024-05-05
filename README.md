**Project Write-Up**  
*Project Overview*  
- This project focuses on analyzing social network connections, specifically the relationships between Twitter users and their friends. The dataset consists of user data, including lists of friends (other users they follow), and is used to construct a graph representing this network. The primary objective is to analyze this graph structure to uncover valuable insights into the relationships and overall connectivity among Twitter users.  
  
**How the Project Works**  
*Graph Construction*: The data is read from a CSV file and organized into a graph structure using an adjacency list (represented as a HashMap). Each node represents a Twitter user, and each connection represents a "follows" relationship.  
*Graph Analysis*: Several analysis functions are implemented to calculate various metrics:  
- *Breadth-First Search (BFS)*: To compute distances between nodes.
- *Degree Distribution*: To determine how many followers each user has and identify patterns.  
- *Average Degree*: To measure the average number of connections per user, directly and within two steps.  
- *Output Metrics*: The project calculates and outputs key metrics for the graph:  
  - *Max Path Length*: The longest distance between any two connected users.
  - *Min Path Length*: The shortest non-zero distance between two connected users.
  - *Median Path Length*: The median distance across all pairs of connected users.
  - *Standard Deviation*: The variability in the path lengths.
  - *Average Distance*: The average distance between all connected users.
  - *Degree Distribution*: A mapping of how many users have a given number of followers.
  - *Degree Distribution at Distance 2*: A mapping of how many users can reach a given number of others within two steps.
**How to Run the Project**  
*Setup:*  
 - Install Rust programming environment.  
 - Place your Twitter dataset in the project's root directory as data.csv. You can download the file here: https://www.kaggle.com/datasets/hwassner/TwitterFriends
   - Place the file in 'finalproj'   
 - *Compilation*:  
  - Compile the project using cargo build.  
 - *Execution*:  
  - Run the project with cargo run.  
  - The program will read the graph data from data.csv and compute all the metrics.
  
**Example Output**  
Here's what the output looks like after running the project:  
Number of nodes in the graph: 73671  
Max path length: 5  
Min path length: 1  
Median path length: 2.00  
Standard deviation: 0.52  
Average distance: 1.51  
Degree Distribution: {40: 1, 78: 1, 27: 1, 9: 22, 20: 3, 4: 169, 5: 102, 30: 1, 46: 1, 17: 4, 2: 1824, 8: 29, 51: 2, 36: 1, 7: 37, 3: 444, 1: 70884, 10: 9, 47: 2, 39: 1, 38: 1, 23: 3, 6: 76, 24: 1, 16: 3, 22: 1, 54: 1, 96: 1, 18: 1, 86: 1, 11: 13, 75: 1, 26: 2, 12: 9, 13: 9, 15: 5, 14: 5}  
Degree Distribution at Distance 2: {3: 678, 54: 1, 0: 64634, 15: 48, 7: 232, 14: 75, 13: 70, 38: 39, 25: 52, 74: 75, 23: 24, 5: 456, 4: 510, 95: 96, 35: 36, 53: 53, 46: 94, 16: 68, 2: 1326, 6: 259, 19: 60, 12: 117, 10: 143, 39: 40, 26: 27, 8: 198, 45: 46, 77: 78, 37: 38, 29: 30, 17: 18, 22: 69, 85: 86, 1: 3573, 21: 22, 9: 90, 50: 102, 11: 108}  
Average degree at distance 1: 1.09  
Average degree at distance 2: 1.10  

**Conclusions**  
*Graph Size and Connectivity*: With over 73,000 nodes and an average distance of 1.51 between users, most Twitter users are closely connected. The graph is tightly connected, with the median distance being only 2.  
*Max and Min Path Lengths*: The maximum path length is 5, indicating that even the most distantly connected users can still reach each other within a few hops. The minimum path length of 1 confirms that most users are directly connected.  
*Degree Distributions*:  
 - *Direct Connections*: The distribution shows a strong skew, with a significant majority of users having only one follower. Only a few users have a high number of followers, reflecting the influence and popularity disparities often found on social media.  
 - *Connections at Distance 2*: A large number of users can connect to others within two steps, providing a broader but still constrained network. This reflects the “small-world phenomenon,” where a few users serve as bridges, expanding reach.  
 - *Average Degree*: The similarity in the average degree at distance 1 and 2 (1.09 vs. 1.10) suggests that most connections do not expand the network significantly beyond the direct friend circle.  
These insights provide valuable information on the network dynamics of Twitter, revealing the presence of influential users, a strong core-periphery structure, and the small-world phenomenon.  
