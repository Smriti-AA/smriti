use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LinkType {
    WikiLink,
    Backlink,
    Tag,
    AiSuggested,
}

impl LinkType {
    pub fn as_str(&self) -> &str {
        match self {
            LinkType::WikiLink => "wikilink",
            LinkType::Backlink => "backlink",
            LinkType::Tag => "tag",
            LinkType::AiSuggested => "ai_suggested",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "wikilink" => LinkType::WikiLink,
            "backlink" => LinkType::Backlink,
            "tag" => LinkType::Tag,
            "ai_suggested" => LinkType::AiSuggested,
            _ => LinkType::WikiLink,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub id: String,
    pub source_note_id: String,
    pub target_note_id: String,
    pub link_type: LinkType,
    pub created_at: DateTime<Utc>,
}

impl Link {
    pub fn new(source: String, target: String, link_type: LinkType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            source_note_id: source,
            target_note_id: target,
            link_type,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub stats: GraphStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub title: String,
    pub tag_count: usize,
    pub link_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub link_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStats {
    pub total_notes: usize,
    pub total_links: usize,
    pub total_tags: usize,
    pub orphan_notes: usize,
    pub most_linked: Option<String>,
}
