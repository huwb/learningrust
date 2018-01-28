use ::outline_print::OutlinePrint;

pub fn run() {
    "Advanced Traits".outline_print();

    let g = TestGraph {
        edges: vec![
            TestGraphEdge {
                n0: 0,
                n1: 1,
                d: 1.0,
            },
            TestGraphEdge {
                n0: 1,
                n1: 3,
                d: 1.0,
            },
            TestGraphEdge {
                n0: 0,
                n1: 2,
                d: 0.5,
            },
            TestGraphEdge {
                n0: 2,
                n1: 3,
                d: 0.75,
            },
        ],
    };

    let result = shortest_path_exhaustive(&g, 0, 3);
    assert_eq!(result, Some(1.25));

    println!("Graph: {:?}", g);
    println!("Shortest path: {:?}", result);
}

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

// simple graph trait. edges are not directed.
trait AGraph {
    type Node: Eq + Copy + Hash;
    type Edge: Eq + Copy;

    fn edge_length(e: Self::Edge) -> f32;
    fn incident_edges(&self, n: Self::Node) -> Vec<&Self::Edge>;

    fn n0(e: Self::Edge) -> Self::Node;
    fn n1(e: Self::Edge) -> Self::Node;
}

// find shortest path in graph g between n0 and n1. edges are non-directed. cycles are
// not an issue.
fn shortest_path_exhaustive<G: AGraph>(g: &G, n0: G::Node, n1: G::Node) -> Option<f32> {
    // start searching at provided node (distance = 0 at start)
    let mut queue = vec![(n0, 0.0)];

    // keep track of visited nodes and shortest distance to start node
    let mut visited: HashMap<G::Node, f32> = HashMap::new();
    visited.insert(n0, 0.0);

    while let Some((n, d)) = queue.pop() {
        // for each node in the work queue, look at neighbours
        for &e in g.incident_edges(n) {
            let n_other = if G::n0(e) == n { G::n1(e) } else { G::n0(e) };
            let e_len = G::edge_length(e);

            // has this neighbouring node already been visited
            if let Some(&d_other) = visited.get(&n_other) {
                // node already visited
                if d_other > d + e_len {
                    visited.insert(n_other, d + e_len);
                }
            } else {
                // node encountered for first time
                visited.insert(n_other, d + e_len);
                queue.push((n_other, d + e_len));
            }
        }
    }

    // to finish, return shortest distance in visited list
    match visited.get(&n1) {
        Some(&dist) => Some(dist),
        _ => None,
    }
}

// Make a simple graph implementation with i32 nodes
type TestGraphNode = i32;

#[derive(PartialEq, Copy, Clone, Debug)]
struct TestGraphEdge {
    n0: TestGraphNode,
    n1: TestGraphNode,
    d: f32,
}

impl Eq for TestGraphEdge {}

#[derive(Debug)]
struct TestGraph {
    edges: Vec<TestGraphEdge>,
}

impl AGraph for TestGraph {
    type Node = TestGraphNode;
    type Edge = TestGraphEdge;

    fn edge_length(e: Self::Edge) -> f32 {
        e.d
    }

    fn incident_edges(&self, n: Self::Node) -> Vec<&Self::Edge> {
        self.edges
            .iter()
            .filter(|&edge| edge.n0 == n || edge.n1 == n)
            .collect()
    }

    fn n0(e: Self::Edge) -> Self::Node {
        e.n0
    }
    fn n1(e: Self::Edge) -> Self::Node {
        e.n1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dist_no_edges() {
        let g = TestGraph { edges: vec![] };

        assert_eq!(shortest_path_exhaustive(&g, 0, 1), None);
    }

    #[test]
    fn dist_one_edge() {
        let g = TestGraph {
            edges: vec![
                TestGraphEdge {
                    n0: 0,
                    n1: 1,
                    d: 1.0,
                },
            ],
        };

        assert_eq!(shortest_path_exhaustive(&g, 0, 1), Some(1.0));
    }

    #[test]
    fn dist_one_edge_switched_nodes() {
        let g = TestGraph {
            edges: vec![
                TestGraphEdge {
                    n0: 1,
                    n1: 0,
                    d: 1.0,
                },
            ],
        };

        assert_eq!(shortest_path_exhaustive(&g, 0, 1), Some(1.0));
    }

    #[test]
    fn dist_one_edge_bad_node() {
        let g = TestGraph {
            edges: vec![
                TestGraphEdge {
                    n0: 0,
                    n1: 1,
                    d: 1.0,
                },
            ],
        };

        assert_eq!(shortest_path_exhaustive(&g, 0, 2), None);
    }

    #[test]
    fn dist_two_edges() {
        let g = TestGraph {
            edges: vec![
                TestGraphEdge {
                    n0: 0,
                    n1: 3,
                    d: 1.0,
                },
                TestGraphEdge {
                    n0: 3,
                    n1: 2,
                    d: 2.0,
                },
            ],
        };

        assert_eq!(shortest_path_exhaustive(&g, 0, 2), Some(3.0));
    }

    #[test]
    fn dist_multiple_first_shortest() {
        let g = TestGraph {
            edges: vec![
                TestGraphEdge {
                    n0: 0,
                    n1: 1,
                    d: 1.0,
                },
                TestGraphEdge {
                    n0: 1,
                    n1: 3,
                    d: 1.0,
                },
                TestGraphEdge {
                    n0: 0,
                    n1: 2,
                    d: 0.5,
                },
                TestGraphEdge {
                    n0: 2,
                    n1: 3,
                    d: 0.75,
                },
            ],
        };

        assert_eq!(shortest_path_exhaustive(&g, 0, 3), Some(1.25));
    }

    #[test]
    fn dist_multiple_second_shortest() {
        let g = TestGraph {
            edges: vec![
                TestGraphEdge {
                    n0: 0,
                    n1: 2,
                    d: 0.5,
                },
                TestGraphEdge {
                    n0: 2,
                    n1: 3,
                    d: 0.75,
                },
                TestGraphEdge {
                    n0: 0,
                    n1: 1,
                    d: 1.0,
                },
                TestGraphEdge {
                    n0: 1,
                    n1: 3,
                    d: 1.0,
                },
            ],
        };

        assert_eq!(shortest_path_exhaustive(&g, 0, 3), Some(1.25));
    }
}
