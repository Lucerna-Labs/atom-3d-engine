// Ordo app: report whether a directed path exists from a start node to a target node.
// Plain-data public struct with String fields and an integer hop count (no f64, derives Eq).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reachability {
    pub reachable: String,
    pub route: Vec<String>,
    pub hops: i64,
}

// Private helper: a single directed edge, kept in insertion order.
struct Edge {
    from: String,
    to: String,
}

pub struct Graph {
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Graph { edges: Vec::new() }
    }

    // Mutating setup method returns () so the private Edge type stays private.
    pub fn link(&mut self, from: &str, to: &str) {
        self.edges.push(Edge { from: from.to_string(), to: to.to_string() });
    }

    // Query method returns the public plain-data Reachability. Depth-first walk
    // following outgoing edges in insertion order, owning Strings, cycle-safe via
    // a visited list. The route is the first path found to the target; hops is the
    // number of edges along it. When unreachable, route is empty and hops is 0.
    pub fn reach(&self, start: &str, target: &str) -> Reachability {
        let mut visited: Vec<String> = Vec::new();
        let mut route: Vec<String> = Vec::new();
        let found = self.walk(start, target, &mut visited, &mut route);
        if found {
            let hops = (route.len() as i64) - 1;
            Reachability { reachable: "yes".to_string(), route, hops }
        } else {
            Reachability { reachable: "no".to_string(), route: Vec::new(), hops: 0 }
        }
    }

    fn walk(&self, current: &str, target: &str, visited: &mut Vec<String>, route: &mut Vec<String>) -> bool {
        if visited.iter().any(|v| v == current) {
            return false;
        }
        visited.push(current.to_string());
        route.push(current.to_string());
        if current == target {
            return true;
        }
        for edge in self.edges.iter() {
            if edge.from == current {
                if self.walk(&edge.to, target, visited, route) {
                    return true;
                }
            }
        }
        route.pop();
        false
    }
}
