use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

use crate::api::server::AppState;
use crate::errors::AppError;
use crate::models::*;
use crate::parser;

/// POST /api/v1/notes
pub async fn create_note(
    State(state): State<AppState>,
    Json(mut req): Json<CreateNoteRequest>,
) -> Result<(StatusCode, Json<Note>), AppError> {
    // Auto-extract tags from content
    let content_tags = parser::extract_tags(&req.content);
    for tag in content_tags {
        if !req.tags.contains(&tag) {
            req.tags.push(tag);
        }
    }

    // Extract frontmatter tags
    if let Some((fm, _)) = parser::parse_frontmatter(&req.content) {
        for tag in fm.tags {
            if !req.tags.contains(&tag) {
                req.tags.push(tag);
            }
        }
    }

    let note = state.db.create_note(req)?;

    // Process wiki-links and create link records
    let wikilinks = parser::extract_wikilinks(&note.content);
    for wl in &wikilinks {
        // Try to find target note by title
        if let Ok(Some(target)) = state.db.get_note_by_title(&wl.target) {
            let _ = state
                .db
                .create_link(&note.id, &target.id, LinkType::WikiLink);
        }
    }

    Ok((StatusCode::CREATED, Json(note)))
}

/// GET /api/v1/notes
pub async fn list_notes(
    State(state): State<AppState>,
    Query(query): Query<NoteListQuery>,
) -> Result<Json<Vec<NoteSummary>>, AppError> {
    let notes = state.db.list_notes(&query)?;
    Ok(Json(notes))
}

/// GET /api/v1/notes/:id
pub async fn get_note(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Note>, AppError> {
    let note = state.db.get_note(&id)?;
    Ok(Json(note))
}

/// PUT /api/v1/notes/:id
pub async fn update_note(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateNoteRequest>,
) -> Result<Json<Note>, AppError> {
    let note = state.db.update_note(&id, req)?;

    // Re-process wiki-links
    let wikilinks = parser::extract_wikilinks(&note.content);
    for wl in &wikilinks {
        if let Ok(Some(target)) = state.db.get_note_by_title(&wl.target) {
            let _ = state
                .db
                .create_link(&note.id, &target.id, LinkType::WikiLink);
        }
    }

    Ok(Json(note))
}

/// DELETE /api/v1/notes/:id
pub async fn delete_note(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    state.db.delete_note(&id)?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/v1/notes/search?q=...
pub async fn search_notes(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<NoteSummary>>, AppError> {
    if query.q.is_empty() {
        return Err(AppError::BadRequest("Search query cannot be empty".into()));
    }
    let results = state.db.search_notes(&query)?;
    Ok(Json(results))
}

/// GET /api/v1/notes/:id/backlinks
pub async fn get_backlinks(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<NoteSummary>>, AppError> {
    let backlinks = state.db.get_backlinks(&id)?;
    Ok(Json(backlinks))
}

/// GET /api/v1/notes/:id/links
pub async fn get_forward_links(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<NoteSummary>>, AppError> {
    let links = state.db.get_forward_links(&id)?;
    Ok(Json(links))
}

/// POST /api/v1/notes/:id/tags
pub async fn add_tags(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(tags): Json<Vec<String>>,
) -> Result<Json<Note>, AppError> {
    // Get existing note
    let existing = state.db.get_note(&id)?;
    let mut all_tags = existing.tags.clone();
    for tag in tags {
        if !all_tags.contains(&tag) {
            all_tags.push(tag);
        }
    }

    let note = state.db.update_note(
        &id,
        UpdateNoteRequest {
            title: None,
            content: None,
            tags: Some(all_tags),
        },
    )?;

    Ok(Json(note))
}
