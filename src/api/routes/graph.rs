use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use std::collections::HashMap;

use crate::api::server::AppState;
use crate::errors::AppError;
use crate::graph::KnowledgeGraph;
use crate::models::{GraphData, GraphStats};

#[derive(Debug, Deserialize)]
pub struct SubgraphQuery {
    #[serde(default = "default_depth")]
    pub depth: usize,
}

fn default_depth() -> usize {
    2
}

/// GET /api/v1/graph — Full knowledge graph
pub async fn get_full_graph(
    State(state): State<AppState>,
) -> Result<Json<GraphData>, AppError> {
    let links = state.db.get_all_links()?;

    // Build title + tag count maps
    let notes = state.db.list_notes(&crate::models::NoteListQuery {
        limit: 10000,
        offset: 0,
        sort: crate::models::SortOrder::UpdatedDesc,
        tag: None,
    })?;

    let mut titles: HashMap<String, String> = HashMap::new();
    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    for note in &notes {
        titles.insert(note.id.clone(), note.title.clone());
        tag_counts.insert(note.id.clone(), note.tag_count);
    }

    let kg = KnowledgeGraph::from_links(&links, &titles, &tag_counts);
    let mut graph_data = kg.export();

    // Fill in total tags from stats
    let stats = state.db.get_stats()?;
    graph_data.stats.total_tags = stats.total_tags;

    Ok(Json(graph_data))
}

/// GET /api/v1/graph/:id — Subgraph around a note
pub async fn get_subgraph(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<SubgraphQuery>,
) -> Result<Json<GraphData>, AppError> {
    // Verify note exists
    let _ = state.db.get_note(&id)?;

    let links = state.db.get_all_links()?;

    let notes = state.db.list_notes(&crate::models::NoteListQuery {
        limit: 10000,
        offset: 0,
        sort: crate::models::SortOrder::UpdatedDesc,
        tag: None,
    })?;

    let mut titles: HashMap<String, String> = HashMap::new();
    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    for note in &notes {
        titles.insert(note.id.clone(), note.title.clone());
        tag_counts.insert(note.id.clone(), note.tag_count);
    }

    let kg = KnowledgeGraph::from_links(&links, &titles, &tag_counts);
    let graph_data = kg.export_subgraph(&id, query.depth);

    Ok(Json(graph_data))
}

/// GET /api/v1/stats — Graph and database statistics
pub async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<GraphStats>, AppError> {
    let stats = state.db.get_stats()?;
    Ok(Json(stats))
}
