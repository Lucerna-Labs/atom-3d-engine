//! weighted_route: directed edges carry integer weights. A route walks the
//! first outgoing edge (in insertion order) from a start node, accumulating the
//! total edge weight, until it reaches a node with no outgoing edge or revisits
//! a node it has already been to. On a dead end it emits "end:<node>"; on a
//! revisit it emits "cycle:<node>" naming the node it would have re-entered.

/// Result of walking a route: the path of visited nodes, the summed weight of
/// the edges traversed, and any emitted signals.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteTrace {
    pub path: Vec<String>,
    pub total: i64,
    pub signals: Vec<String>,
}

/// A directed, integer-weighted edge, stored in insertion order.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Edge {
    from: String,
    to: String,
    weight: i64,
}

/// Runtime that holds the weighted edges and walks routes over them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WeightRouter {
    edges: Vec<Edge>,
}

impl WeightRouter {
    pub fn new() -> WeightRouter {
        WeightRouter { edges: Vec::new() }
    }

    /// Record a directed edge from `from` to `to` carrying `weight`, preserving
    /// insertion order.
    pub fn connect(&mut self, from: &str, to: &str, weight: i64) {
        self.edges.push(Edge {
            from: from.to_string(),
            to: to.to_string(),
            weight,
        });
    }

    /// Find the first outgoing edge (in insertion order) for `node`.
    fn first_outgoing(&self, node: &str) -> Option<&Edge> {
        self.edges.iter().find(|e| e.from == node)
    }

    /// Walk from `start`, taking the first outgoing edge per node and summing
    /// its weight, until a node has no outgoing edge (emits "end:<node>") or the
    /// next node was already visited (emits "cycle:<node>"). The cycle edge's
    /// weight is NOT added and the revisited node is NOT re-appended.
    pub fn route(&self, start: &str) -> RouteTrace {
        let mut path: Vec<String> = vec![start.to_string()];
        let mut signals: Vec<String> = Vec::new();
        let mut total: i64 = 0;
        let mut current = start.to_string();

        loop {
            match self.first_outgoing(&current) {
                Some(edge) => {
                    if path.contains(&edge.to) {
                        signals.push(format!("cycle:{}", edge.to));
                        break;
                    }
                    total += edge.weight;
                    path.push(edge.to.clone());
                    current = edge.to.clone();
                }
                None => {
                    signals.push(format!("end:{}", current));
                    break;
                }
            }
        }

        RouteTrace {
            path,
            total,
            signals,
        }
    }
}

impl Default for WeightRouter {
    fn default() -> WeightRouter {
        WeightRouter::new()
    }
}
