use serde_json::Value;
use std::collections::HashMap;

use crate::graph::KnowledgeGraph;
use crate::models::*;
use crate::parser;
use crate::storage::Database;

pub fn handle_notes_create(db: &Database, args: &Value) -> Result<Value, String> {
    let title = args
        .get("title")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'title'")?
        .to_string();

    let content = args
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'content'")?
        .to_string();

    let tags: Vec<String> = args
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let mut all_tags = tags;
    for tag in parser::extract_tags(&content) {
        if !all_tags.contains(&tag) {
            all_tags.push(tag);
        }
    }

    let note = db
        .create_note(CreateNoteRequest {
            title,
            content: content.clone(),
            tags: all_tags,
        })
        .map_err(|e| e.to_string())?;

    // Process wiki-links
    let wikilinks = parser::extract_wikilinks(&content);
    for wl in &wikilinks {
        if let Ok(Some(target)) = db.get_note_by_title(&wl.target) {
            let _ = db.create_link(&note.id, &target.id, LinkType::WikiLink);
        }
    }

    Ok(serde_json::to_value(&note).unwrap_or_default())
}

pub fn handle_notes_read(db: &Database, args: &Value) -> Result<Value, String> {
    let id = args
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'id'")?;

    // Try by ID first, then by title
    let note = match db.get_note(id) {
        Ok(n) => n,
        Err(_) => match db.get_note_by_title(id).map_err(|e| e.to_string())? {
            Some(n) => db.get_note(&n.id).map_err(|e| e.to_string())?,
            None => return Err(format!("Note not found: {}", id)),
        },
    };

    Ok(serde_json::to_value(&note).unwrap_or_default())
}

pub fn handle_notes_search(db: &Database, args: &Value) -> Result<Value, String> {
    let query = args
        .get("query")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'query'")?;

    let limit = args
        .get("limit")
        .and_then(|v| v.as_u64())
        .unwrap_or(10) as usize;

    let results = db
        .search_notes(&SearchQuery {
            q: query.to_string(),
            limit,
            offset: 0,
        })
        .map_err(|e| e.to_string())?;

    Ok(serde_json::to_value(&results).unwrap_or_default())
}

pub fn handle_notes_list(db: &Database, args: &Value) -> Result<Value, String> {
    let limit = args
        .get("limit")
        .and_then(|v| v.as_u64())
        .unwrap_or(20) as usize;

    let tag = args.get("tag").and_then(|v| v.as_str()).map(String::from);

    let notes = db
        .list_notes(&NoteListQuery {
            limit,
            offset: 0,
            sort: SortOrder::UpdatedDesc,
            tag,
        })
        .map_err(|e| e.to_string())?;

    Ok(serde_json::to_value(&notes).unwrap_or_default())
}

pub fn handle_notes_graph(db: &Database, args: &Value) -> Result<Value, String> {
    let center_id = args.get("center_id").and_then(|v| v.as_str());
    let depth = args
        .get("depth")
        .and_then(|v| v.as_u64())
        .unwrap_or(2) as usize;

    let links = db.get_all_links().map_err(|e| e.to_string())?;
    let notes = db
        .list_notes(&NoteListQuery {
            limit: 10000,
            offset: 0,
            sort: SortOrder::UpdatedDesc,
            tag: None,
        })
        .map_err(|e| e.to_string())?;

    let mut titles: HashMap<String, String> = HashMap::new();
    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    for note in &notes {
        titles.insert(note.id.clone(), note.title.clone());
        tag_counts.insert(note.id.clone(), note.tag_count);
    }

    let kg = KnowledgeGraph::from_links(&links, &titles, &tag_counts);

    let graph_data = if let Some(id) = center_id {
        kg.export_subgraph(id, depth)
    } else {
        kg.export()
    };

    Ok(serde_json::to_value(&graph_data).unwrap_or_default())
}

pub fn handle_memory_store(db: &Database, args: &Value) -> Result<Value, String> {
    let agent_id = args
        .get("agent_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'agent_id'")?;

    let key = args
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'key'")?;

    let value = args.get("value").cloned().ok_or("Missing 'value'")?;

    let namespace = args
        .get("namespace")
        .and_then(|v| v.as_str())
        .map(String::from);

    let ttl_seconds = args
        .get("ttl_seconds")
        .and_then(|v| v.as_i64());

    let memory = db
        .store_memory(
            agent_id,
            CreateMemoryRequest {
                namespace,
                key: key.to_string(),
                value,
                ttl_seconds,
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(serde_json::to_value(&memory).unwrap_or_default())
}

pub fn handle_memory_retrieve(db: &Database, args: &Value) -> Result<Value, String> {
    let agent_id = args
        .get("agent_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'agent_id'")?;

    let key = args
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'key'")?;

    let namespace = args
        .get("namespace")
        .and_then(|v| v.as_str())
        .unwrap_or("default");

    let memory = db
        .get_memory(agent_id, namespace, key)
        .map_err(|e| e.to_string())?;

    if memory.is_expired() {
        return Err("Memory entry has expired".to_string());
    }

    Ok(serde_json::to_value(&memory).unwrap_or_default())
}

pub fn handle_memory_list(db: &Database, args: &Value) -> Result<Value, String> {
    let agent_id = args
        .get("agent_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'agent_id'")?;

    let namespace = args.get("namespace").and_then(|v| v.as_str());

    let memories = db
        .list_agent_memory(agent_id, namespace)
        .map_err(|e| e.to_string())?;

    Ok(serde_json::to_value(&memories).unwrap_or_default())
}
