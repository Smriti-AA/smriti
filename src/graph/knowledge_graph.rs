use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::models::{GraphData, GraphEdge, GraphNode, GraphStats, Link};

/// In-memory knowledge graph built from note links
pub struct KnowledgeGraph {
    graph: DiGraph<NodeInfo, EdgeInfo>,
    id_to_index: HashMap<String, NodeIndex>,
}

#[derive(Debug, Clone)]
struct NodeInfo {
    id: String,
    title: String,
    tag_count: usize,
}

#[derive(Debug, Clone)]
struct EdgeInfo {
    link_type: String,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            id_to_index: HashMap::new(),
        }
    }

    /// Build graph from database links and note metadata
    pub fn from_links(
        links: &[Link],
        note_titles: &HashMap<String, String>,
        note_tag_counts: &HashMap<String, usize>,
    ) -> Self {
        let mut kg = Self::new();

        // Add all nodes referenced in links
        let mut all_ids: HashSet<&str> = HashSet::new();
        for link in links {
            all_ids.insert(&link.source_note_id);
            all_ids.insert(&link.target_note_id);
        }

        for id in &all_ids {
            let title = note_titles
                .get(*id)
                .cloned()
                .unwrap_or_else(|| id.to_string());
            let tag_count = note_tag_counts.get(*id).copied().unwrap_or(0);
            kg.add_node(id.to_string(), title, tag_count);
        }

        // Add edges
        for link in links {
            kg.add_edge(
                &link.source_note_id,
                &link.target_note_id,
                link.link_type.as_str(),
            );
        }

        kg
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, id: String, title: String, tag_count: usize) -> NodeIndex {
        if let Some(&idx) = self.id_to_index.get(&id) {
            return idx;
        }

        let info = NodeInfo {
            id: id.clone(),
            title,
            tag_count,
        };
        let idx = self.graph.add_node(info);
        self.id_to_index.insert(id, idx);
        idx
    }

    /// Add a directed edge between two nodes
    pub fn add_edge(&mut self, source_id: &str, target_id: &str, link_type: &str) {
        if let (Some(&src), Some(&tgt)) =
            (self.id_to_index.get(source_id), self.id_to_index.get(target_id))
        {
            self.graph.add_edge(
                src,
                tgt,
                EdgeInfo {
                    link_type: link_type.to_string(),
                },
            );
        }
    }

    /// Get all notes that link TO this note (backlinks)
    pub fn get_backlinks(&self, note_id: &str) -> Vec<String> {
        if let Some(&idx) = self.id_to_index.get(note_id) {
            self.graph
                .edges_directed(idx, Direction::Incoming)
                .map(|e| self.graph[e.source()].id.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all notes this note links TO (forward links)
    pub fn get_forward_links(&self, note_id: &str) -> Vec<String> {
        if let Some(&idx) = self.id_to_index.get(note_id) {
            self.graph
                .edges_directed(idx, Direction::Outgoing)
                .map(|e| self.graph[e.target()].id.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Find related notes within N hops using BFS
    pub fn get_related_notes(&self, note_id: &str, max_depth: usize) -> Vec<(String, usize)> {
        let Some(&start_idx) = self.id_to_index.get(note_id) else {
            return Vec::new();
        };

        let mut visited: HashSet<NodeIndex> = HashSet::new();
        let mut queue: VecDeque<(NodeIndex, usize)> = VecDeque::new();
        let mut results: Vec<(String, usize)> = Vec::new();

        visited.insert(start_idx);
        queue.push_back((start_idx, 0));

        while let Some((current, depth)) = queue.pop_front() {
            if depth > 0 {
                results.push((self.graph[current].id.clone(), depth));
            }

            if depth >= max_depth {
                continue;
            }

            // Traverse both incoming and outgoing edges
            for edge in self.graph.edges_directed(current, Direction::Outgoing) {
                let neighbor = edge.target();
                if visited.insert(neighbor) {
                    queue.push_back((neighbor, depth + 1));
                }
            }
            for edge in self.graph.edges_directed(current, Direction::Incoming) {
                let neighbor = edge.source();
                if visited.insert(neighbor) {
                    queue.push_back((neighbor, depth + 1));
                }
            }
        }

        results
    }

    /// Get nodes with no connections (orphan notes)
    pub fn get_orphans(&self) -> Vec<String> {
        self.graph
            .node_indices()
            .filter(|&idx| {
                self.graph.edges_directed(idx, Direction::Incoming).count() == 0
                    && self.graph.edges_directed(idx, Direction::Outgoing).count() == 0
            })
            .map(|idx| self.graph[idx].id.clone())
            .collect()
    }

    /// Export the full graph as a JSON-friendly data structure
    pub fn export(&self) -> GraphData {
        let nodes: Vec<GraphNode> = self
            .graph
            .node_indices()
            .map(|idx| {
                let info = &self.graph[idx];
                let link_count = self.graph.edges_directed(idx, Direction::Incoming).count()
                    + self.graph.edges_directed(idx, Direction::Outgoing).count();
                GraphNode {
                    id: info.id.clone(),
                    title: info.title.clone(),
                    tag_count: info.tag_count,
                    link_count,
                }
            })
            .collect();

        let edges: Vec<GraphEdge> = self
            .graph
            .edge_indices()
            .filter_map(|idx| {
                let (src, tgt) = self.graph.edge_endpoints(idx)?;
                let edge_info = &self.graph[idx];
                Some(GraphEdge {
                    source: self.graph[src].id.clone(),
                    target: self.graph[tgt].id.clone(),
                    link_type: edge_info.link_type.clone(),
                })
            })
            .collect();

        let orphan_notes = self.get_orphans().len();
        let most_linked = nodes
            .iter()
            .max_by_key(|n| n.link_count)
            .map(|n| n.title.clone());

        GraphData {
            stats: GraphStats {
                total_notes: nodes.len(),
                total_links: edges.len(),
                total_tags: 0, // populated by caller
                orphan_notes,
                most_linked,
            },
            nodes,
            edges,
        }
    }

    /// Export a subgraph centered on a specific note
    pub fn export_subgraph(&self, center_id: &str, depth: usize) -> GraphData {
        let related = self.get_related_notes(center_id, depth);
        let mut relevant_ids: HashSet<&str> = HashSet::new();
        relevant_ids.insert(center_id);
        for (id, _) in &related {
            relevant_ids.insert(id);
        }

        let nodes: Vec<GraphNode> = self
            .graph
            .node_indices()
            .filter(|&idx| relevant_ids.contains(self.graph[idx].id.as_str()))
            .map(|idx| {
                let info = &self.graph[idx];
                let link_count = self.graph.edges_directed(idx, Direction::Incoming).count()
                    + self.graph.edges_directed(idx, Direction::Outgoing).count();
                GraphNode {
                    id: info.id.clone(),
                    title: info.title.clone(),
                    tag_count: info.tag_count,
                    link_count,
                }
            })
            .collect();

        let edges: Vec<GraphEdge> = self
            .graph
            .edge_indices()
            .filter_map(|idx| {
                let (src, tgt) = self.graph.edge_endpoints(idx)?;
                let src_id = &self.graph[src].id;
                let tgt_id = &self.graph[tgt].id;
                if relevant_ids.contains(src_id.as_str()) && relevant_ids.contains(tgt_id.as_str())
                {
                    let edge_info = &self.graph[idx];
                    Some(GraphEdge {
                        source: src_id.clone(),
                        target: tgt_id.clone(),
                        link_type: edge_info.link_type.clone(),
                    })
                } else {
                    None
                }
            })
            .collect();

        GraphData {
            stats: GraphStats {
                total_notes: nodes.len(),
                total_links: edges.len(),
                total_tags: 0,
                orphan_notes: 0,
                most_linked: None,
            },
            nodes,
            edges,
        }
    }

    /// Get total node and edge counts
    pub fn stats(&self) -> (usize, usize) {
        (self.graph.node_count(), self.graph.edge_count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_graph() {
        let mut kg = KnowledgeGraph::new();
        kg.add_node("a".into(), "Note A".into(), 0);
        kg.add_node("b".into(), "Note B".into(), 0);
        kg.add_edge("a", "b", "wikilink");

        assert_eq!(kg.get_forward_links("a"), vec!["b"]);
        assert_eq!(kg.get_backlinks("b"), vec!["a"]);
    }

    #[test]
    fn test_related_notes() {
        let mut kg = KnowledgeGraph::new();
        kg.add_node("a".into(), "A".into(), 0);
        kg.add_node("b".into(), "B".into(), 0);
        kg.add_node("c".into(), "C".into(), 0);
        kg.add_edge("a", "b", "wikilink");
        kg.add_edge("b", "c", "wikilink");

        let related = kg.get_related_notes("a", 2);
        assert_eq!(related.len(), 2);
    }

    #[test]
    fn test_orphan_detection() {
        let mut kg = KnowledgeGraph::new();
        kg.add_node("a".into(), "A".into(), 0);
        kg.add_node("b".into(), "B".into(), 0);
        kg.add_node("orphan".into(), "Orphan".into(), 0);
        kg.add_edge("a", "b", "wikilink");

        let orphans = kg.get_orphans();
        assert_eq!(orphans, vec!["orphan"]);
    }
}
