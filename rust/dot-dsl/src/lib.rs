pub mod graph {
    pub mod graph_items {
        pub mod edge {
            use std::str;

            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: Vec<(String, String)>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: vec![],
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    self
                }
                pub fn attr(&self, key: &str) -> Option<&str> {
                    let result = self.attrs.iter().find(|(k, _)| k == key);
                    match result {
                        Some((_, value)) => Some(value.as_str()),
                        None => None,
                    }
                }
            }
        }

        pub mod node {
            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: Vec<(String, String)>,
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: vec![],
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    self
                }
                pub fn attr(&self, key: &str) -> Option<&str> {
                    let result = self.attrs.iter().find(|(k, _)| k == key);
                    match result {
                        Some((_, value)) => Some(value.as_str()),
                        None => None,
                    }
                }
            }
        }
    }

    use maplit::hashmap;
    use std::collections::HashMap;

    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: hashmap! {},
            }
        }
        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            self
        }
        pub fn node(&self, key: &str) -> Option<Node> {
            self.nodes.iter().find(|n| n.name == key).cloned()
        }
    }
}
