use std::collections::HashMap;
use std::path::Path;

use crate::errors::AppResult;
use crate::graph::KnowledgeGraph;
use crate::models::*;
use crate::parser;
use crate::storage::Database;

/// Handle `notes create`
pub fn handle_create(
    db: &Database,
    title: String,
    content: Option<String>,
    file: Option<String>,
    tags: Option<Vec<String>>,
) -> AppResult<()> {
    let content = if let Some(file_path) = file {
        std::fs::read_to_string(&file_path)?
    } else {
        content.unwrap_or_default()
    };

    let mut all_tags = tags.unwrap_or_default();
    // Auto-extract tags from content
    for tag in parser::extract_tags(&content) {
        if !all_tags.contains(&tag) {
            all_tags.push(tag);
        }
    }

    let note = db.create_note(CreateNoteRequest {
        title,
        content: content.clone(),
        tags: all_tags,
    })?;

    // Process wiki-links
    let wikilinks = parser::extract_wikilinks(&content);
    for wl in &wikilinks {
        if let Ok(Some(target)) = db.get_note_by_title(&wl.target) {
            let _ = db.create_link(&note.id, &target.id, LinkType::WikiLink);
        }
    }

    println!("Created note: {} ({})", note.title, note.id);
    if !note.tags.is_empty() {
        println!("  Tags: {}", note.tags.join(", "));
    }
    if !wikilinks.is_empty() {
        println!(
            "  Wiki-links: {}",
            wikilinks
                .iter()
                .map(|w| w.target.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    Ok(())
}

/// Handle `notes read`
pub fn handle_read(db: &Database, id: String, json: bool) -> AppResult<()> {
    // Try by ID first, then by title
    let note = match db.get_note(&id) {
        Ok(n) => n,
        Err(_) => match db.get_note_by_title(&id)? {
            Some(n) => db.get_note(&n.id)?,
            None => return Err(crate::errors::AppError::NoteNotFound(id)),
        },
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&note)?);
    } else {
        println!("━━━ {} ━━━", note.title);
        println!("ID: {}", note.id);
        println!(
            "Created: {} | Updated: {}",
            note.created_at.format("%Y-%m-%d %H:%M"),
            note.updated_at.format("%Y-%m-%d %H:%M")
        );
        if !note.tags.is_empty() {
            println!(
                "Tags: {}",
                note.tags.iter().map(|t| format!("#{}", t)).collect::<Vec<_>>().join(" ")
            );
        }
        println!(
            "Links: {} outgoing, {} incoming",
            note.wikilink_count, note.backlink_count
        );
        println!("───────────────────────────");
        println!("{}", note.content);
    }

    Ok(())
}

/// Handle `notes list`
pub fn handle_list(
    db: &Database,
    limit: usize,
    tag: Option<String>,
    json: bool,
) -> AppResult<()> {
    let notes = db.list_notes(&NoteListQuery {
        limit,
        offset: 0,
        sort: SortOrder::UpdatedDesc,
        tag,
    })?;

    if json {
        println!("{}", serde_json::to_string_pretty(&notes)?);
        return Ok(());
    }

    if notes.is_empty() {
        println!("No notes found.");
        return Ok(());
    }

    println!("{:<38} {:<30} {:<12} {}", "ID", "TITLE", "UPDATED", "TAGS");
    println!("{}", "─".repeat(90));
    for note in &notes {
        println!(
            "{:<38} {:<30} {:<12} {}",
            &note.id[..8],
            truncate(&note.title, 28),
            note.updated_at.format("%Y-%m-%d"),
            note.tag_count,
        );
    }
    println!("\n{} notes shown", notes.len());

    Ok(())
}

/// Handle `notes search`
pub fn handle_search(db: &Database, query: String, limit: usize, json: bool) -> AppResult<()> {
    let results = db.search_notes(&SearchQuery {
        q: query.clone(),
        limit,
        offset: 0,
    })?;

    if json {
        println!("{}", serde_json::to_string_pretty(&results)?);
        return Ok(());
    }

    if results.is_empty() {
        println!("No results for \"{}\"", query);
        return Ok(());
    }

    println!("Search results for \"{}\":\n", query);
    for (i, note) in results.iter().enumerate() {
        println!(
            "  {}. {} ({})",
            i + 1,
            note.title,
            &note.id[..8]
        );
        println!("     {}", truncate(&note.preview, 80));
        println!();
    }

    Ok(())
}

/// Handle `notes graph`
pub fn handle_graph(
    db: &Database,
    format: String,
    center: Option<String>,
    depth: usize,
) -> AppResult<()> {
    let links = db.get_all_links()?;
    let notes = db.list_notes(&NoteListQuery {
        limit: 10000,
        offset: 0,
        sort: SortOrder::UpdatedDesc,
        tag: None,
    })?;

    let mut titles: HashMap<String, String> = HashMap::new();
    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    for note in &notes {
        titles.insert(note.id.clone(), note.title.clone());
        tag_counts.insert(note.id.clone(), note.tag_count);
    }

    let kg = KnowledgeGraph::from_links(&links, &titles, &tag_counts);

    let graph_data = if let Some(center_id) = center {
        kg.export_subgraph(&center_id, depth)
    } else {
        kg.export()
    };

    match format.as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&graph_data)?);
        }
        "dot" => {
            println!("digraph knowledge_graph {{");
            println!("  rankdir=LR;");
            println!("  node [shape=box, style=rounded];");
            for node in &graph_data.nodes {
                println!(
                    "  \"{}\" [label=\"{}\"];",
                    node.id,
                    node.title.replace('"', "\\\"")
                );
            }
            for edge in &graph_data.edges {
                println!(
                    "  \"{}\" -> \"{}\" [label=\"{}\"];",
                    edge.source, edge.target, edge.link_type
                );
            }
            println!("}}");
        }
        _ => {
            // Text format
            println!("Knowledge Graph");
            println!("═══════════════");
            println!(
                "Nodes: {} | Edges: {} | Orphans: {}",
                graph_data.stats.total_notes,
                graph_data.stats.total_links,
                graph_data.stats.orphan_notes
            );
            if let Some(most) = &graph_data.stats.most_linked {
                println!("Most linked: {}", most);
            }
            println!();

            for edge in &graph_data.edges {
                let src_title = titles.get(&edge.source).cloned().unwrap_or_default();
                let tgt_title = titles.get(&edge.target).cloned().unwrap_or_default();
                println!("  {} ──[{}]──> {}", src_title, edge.link_type, tgt_title);
            }
        }
    }

    Ok(())
}

/// Handle `notes stats`
pub fn handle_stats(db: &Database) -> AppResult<()> {
    let stats = db.get_stats()?;
    println!("Database Statistics");
    println!("═══════════════════");
    println!("  Notes:        {}", stats.total_notes);
    println!("  Links:        {}", stats.total_links);
    println!("  Tags:         {}", stats.total_tags);
    println!("  Orphan notes: {}", stats.orphan_notes);
    if let Some(most) = &stats.most_linked {
        println!("  Most linked:  {}", most);
    }
    Ok(())
}

/// Handle `notes import`
pub fn handle_import(db: &Database, path: String, recursive: bool) -> AppResult<()> {
    let dir = Path::new(&path);
    if !dir.is_dir() {
        return Err(crate::errors::AppError::BadRequest(format!(
            "{} is not a directory",
            path
        )));
    }

    let mut count = 0;
    import_dir(db, dir, recursive, &mut count)?;
    println!("Imported {} notes from {}", count, path);
    Ok(())
}

fn import_dir(
    db: &Database,
    dir: &Path,
    recursive: bool,
    count: &mut usize,
) -> AppResult<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() && recursive {
            import_dir(db, &path, recursive, count)?;
        } else if path.extension().map_or(false, |ext| ext == "md") {
            let content = std::fs::read_to_string(&path)?;
            let title = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string();

            let mut tags = parser::extract_tags(&content);
            if let Some((fm, _)) = parser::parse_frontmatter(&content) {
                for tag in fm.tags {
                    if !tags.contains(&tag) {
                        tags.push(tag);
                    }
                }
            }

            db.create_note(CreateNoteRequest {
                title,
                content,
                tags,
            })?;
            *count += 1;
        }
    }
    Ok(())
}

/// Handle `notes export`
pub fn handle_export(db: &Database, path: String, frontmatter: bool) -> AppResult<()> {
    std::fs::create_dir_all(&path)?;

    let notes = db.list_notes(&NoteListQuery {
        limit: 100000,
        offset: 0,
        sort: SortOrder::TitleAsc,
        tag: None,
    })?;

    let mut count = 0;
    for summary in &notes {
        let note = db.get_note(&summary.id)?;
        let filename = format!("{}.md", sanitize_filename(&note.title));
        let filepath = Path::new(&path).join(&filename);

        let content = if frontmatter {
            let tags_str = note
                .tags
                .iter()
                .map(|t| format!("\"{}\"", t))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "---\ntags: [{}]\nid: {}\ncreated: {}\nupdated: {}\n---\n{}",
                tags_str,
                note.id,
                note.created_at.to_rfc3339(),
                note.updated_at.to_rfc3339(),
                note.content
            )
        } else {
            note.content.clone()
        };

        std::fs::write(&filepath, content)?;
        count += 1;
    }

    println!("Exported {} notes to {}", count, path);
    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max - 3])
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}
