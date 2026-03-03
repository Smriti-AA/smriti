use chrono::Utc;
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::models::*;
use crate::storage::Database;

// ─── Note Operations ────────────────────────────────────────────────

impl Database {
    pub fn create_note(&self, req: CreateNoteRequest) -> AppResult<Note> {
        let note = Note::new(req.title, req.content, req.tags.clone());
        self.execute(|conn| {
            conn.execute(
                "INSERT INTO notes (id, title, content, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    note.id,
                    note.title,
                    note.content,
                    note.created_at.to_rfc3339(),
                    note.updated_at.to_rfc3339(),
                ],
            )?;

            // Insert tags
            for tag_name in &req.tags {
                ensure_tag(conn, tag_name)?;
                let tag_id = get_tag_id(conn, tag_name)?;
                conn.execute(
                    "INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
                    params![note.id, tag_id],
                )?;
            }

            Ok(note)
        })
    }

    pub fn get_note(&self, id: &str) -> AppResult<Note> {
        self.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, title, content, created_at, updated_at FROM notes WHERE id = ?1",
            )?;

            let note = stmt
                .query_row(params![id], |row| {
                    Ok(Note {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        content: row.get(2)?,
                        created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                            .unwrap_or_default()
                            .with_timezone(&Utc),
                        updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                            .unwrap_or_default()
                            .with_timezone(&Utc),
                        tags: Vec::new(),
                        backlink_count: 0,
                        wikilink_count: 0,
                    })
                })
                .map_err(|_| AppError::NoteNotFound(id.to_string()))?;

            // Fetch tags
            let tags = get_note_tags(conn, &note.id)?;
            let backlink_count = count_backlinks(conn, &note.id)?;
            let wikilink_count = count_wikilinks(conn, &note.id)?;

            Ok(Note {
                tags,
                backlink_count,
                wikilink_count,
                ..note
            })
        })
    }

    pub fn update_note(&self, id: &str, req: UpdateNoteRequest) -> AppResult<Note> {
        self.execute(|conn| {
            // Check exists
            let exists: bool = conn
                .query_row(
                    "SELECT COUNT(*) > 0 FROM notes WHERE id = ?1",
                    params![id],
                    |row| row.get(0),
                )
                .unwrap_or(false);

            if !exists {
                return Err(AppError::NoteNotFound(id.to_string()));
            }

            let now = Utc::now().to_rfc3339();

            if let Some(title) = &req.title {
                conn.execute(
                    "UPDATE notes SET title = ?1, updated_at = ?2 WHERE id = ?3",
                    params![title, now, id],
                )?;
            }

            if let Some(content) = &req.content {
                conn.execute(
                    "UPDATE notes SET content = ?1, updated_at = ?2 WHERE id = ?3",
                    params![content, now, id],
                )?;
            }

            if let Some(tags) = &req.tags {
                // Clear existing tags
                conn.execute("DELETE FROM note_tags WHERE note_id = ?1", params![id])?;
                for tag_name in tags {
                    ensure_tag(conn, tag_name)?;
                    let tag_id = get_tag_id(conn, tag_name)?;
                    conn.execute(
                        "INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
                        params![id, tag_id],
                    )?;
                }
            }

            Ok(())
        })?;

        self.get_note(id)
    }

    pub fn delete_note(&self, id: &str) -> AppResult<()> {
        self.execute(|conn| {
            let affected = conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
            if affected == 0 {
                return Err(AppError::NoteNotFound(id.to_string()));
            }
            // Cascade handles note_tags and links
            Ok(())
        })
    }

    pub fn list_notes(&self, query: &NoteListQuery) -> AppResult<Vec<NoteSummary>> {
        self.execute(|conn| {
            let order_clause = match query.sort {
                SortOrder::UpdatedDesc => "updated_at DESC",
                SortOrder::UpdatedAsc => "updated_at ASC",
                SortOrder::CreatedDesc => "created_at DESC",
                SortOrder::CreatedAsc => "created_at ASC",
                SortOrder::TitleAsc => "title ASC",
            };

            let sql = if let Some(_tag) = &query.tag {
                format!(
                    "SELECT n.id, n.title, n.content, n.created_at, n.updated_at
                     FROM notes n
                     JOIN note_tags nt ON n.id = nt.note_id
                     JOIN tags t ON nt.tag_id = t.id
                     WHERE t.name = ?3
                     ORDER BY n.{} LIMIT ?1 OFFSET ?2",
                    order_clause
                )
            } else {
                format!(
                    "SELECT id, title, content, created_at, updated_at
                     FROM notes ORDER BY {} LIMIT ?1 OFFSET ?2",
                    order_clause
                )
            };

            let mut stmt = conn.prepare(&sql)?;

            let mut summaries = Vec::new();
            if let Some(tag) = &query.tag {
                let rows = stmt.query_map(params![query.limit, query.offset, tag], |row| {
                    build_note_summary(row)
                })?;
                for row in rows {
                    let mut summary = row?;
                    summary.tag_count = get_note_tags(conn, &summary.id).unwrap_or_default().len();
                    summary.backlink_count = count_backlinks(conn, &summary.id).unwrap_or(0);
                    summaries.push(summary);
                }
            } else {
                let rows = stmt.query_map(params![query.limit, query.offset], |row| {
                    build_note_summary(row)
                })?;
                for row in rows {
                    let mut summary = row?;
                    summary.tag_count = get_note_tags(conn, &summary.id).unwrap_or_default().len();
                    summary.backlink_count = count_backlinks(conn, &summary.id).unwrap_or(0);
                    summaries.push(summary);
                }
            };

            Ok(summaries)
        })
    }

    pub fn search_notes(&self, query: &SearchQuery) -> AppResult<Vec<NoteSummary>> {
        self.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT n.id, n.title, n.content, n.created_at, n.updated_at
                 FROM notes n
                 JOIN notes_fts fts ON n.rowid = fts.rowid
                 WHERE notes_fts MATCH ?1
                 ORDER BY rank
                 LIMIT ?2 OFFSET ?3",
            )?;

            let rows = stmt.query_map(params![query.q, query.limit, query.offset], |row| {
                build_note_summary(row)
            })?;

            let mut summaries = Vec::new();
            for row in rows {
                summaries.push(row?);
            }
            Ok(summaries)
        })
    }

    pub fn get_note_by_title(&self, title: &str) -> AppResult<Option<Note>> {
        self.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, title, content, created_at, updated_at FROM notes WHERE title = ?1",
            )?;

            let result = stmt.query_row(params![title], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                        .unwrap_or_default()
                        .with_timezone(&Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                        .unwrap_or_default()
                        .with_timezone(&Utc),
                    tags: Vec::new(),
                    backlink_count: 0,
                    wikilink_count: 0,
                })
            });

            match result {
                Ok(note) => Ok(Some(note)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(AppError::Database(e)),
            }
        })
    }

    // ─── Link Operations ────────────────────────────────────────────

    pub fn create_link(&self, source_id: &str, target_id: &str, link_type: LinkType) -> AppResult<Link> {
        let link = Link::new(source_id.to_string(), target_id.to_string(), link_type);
        self.execute(|conn| {
            conn.execute(
                "INSERT OR IGNORE INTO links (id, source_note_id, target_note_id, link_type, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    link.id,
                    link.source_note_id,
                    link.target_note_id,
                    link.link_type.as_str(),
                    link.created_at.to_rfc3339(),
                ],
            )?;
            Ok(link)
        })
    }

    pub fn get_backlinks(&self, note_id: &str) -> AppResult<Vec<NoteSummary>> {
        self.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT n.id, n.title, n.content, n.created_at, n.updated_at
                 FROM notes n
                 JOIN links l ON n.id = l.source_note_id
                 WHERE l.target_note_id = ?1",
            )?;

            let rows = stmt.query_map(params![note_id], |row| build_note_summary(row))?;
            let mut summaries = Vec::new();
            for row in rows {
                summaries.push(row?);
            }
            Ok(summaries)
        })
    }

    pub fn get_forward_links(&self, note_id: &str) -> AppResult<Vec<NoteSummary>> {
        self.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT n.id, n.title, n.content, n.created_at, n.updated_at
                 FROM notes n
                 JOIN links l ON n.id = l.target_note_id
                 WHERE l.source_note_id = ?1",
            )?;

            let rows = stmt.query_map(params![note_id], |row| build_note_summary(row))?;
            let mut summaries = Vec::new();
            for row in rows {
                summaries.push(row?);
            }
            Ok(summaries)
        })
    }

    pub fn get_all_links(&self) -> AppResult<Vec<Link>> {
        self.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, source_note_id, target_note_id, link_type, created_at FROM links",
            )?;

            let rows = stmt.query_map([], |row| {
                Ok(Link {
                    id: row.get(0)?,
                    source_note_id: row.get(1)?,
                    target_note_id: row.get(2)?,
                    link_type: LinkType::from_str(&row.get::<_, String>(3)?),
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                        .unwrap_or_default()
                        .with_timezone(&Utc),
                })
            })?;

            let mut links = Vec::new();
            for row in rows {
                links.push(row?);
            }
            Ok(links)
        })
    }

    // ─── Agent Memory Operations ────────────────────────────────────

    pub fn store_memory(&self, agent_id: &str, req: CreateMemoryRequest) -> AppResult<AgentMemory> {
        let ns = req.namespace.unwrap_or_else(|| "default".to_string());
        let memory = AgentMemory::new(
            agent_id.to_string(),
            ns,
            req.key,
            req.value,
            req.ttl_seconds,
        );

        self.execute(|conn| {
            conn.execute(
                "INSERT INTO agent_memory (id, agent_id, namespace, key, value, created_at, updated_at, ttl_seconds)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                 ON CONFLICT(agent_id, namespace, key) DO UPDATE SET
                    value = excluded.value,
                    updated_at = excluded.updated_at,
                    ttl_seconds = excluded.ttl_seconds",
                params![
                    memory.id,
                    memory.agent_id,
                    memory.namespace,
                    memory.key,
                    serde_json::to_string(&memory.value)?,
                    memory.created_at.to_rfc3339(),
                    memory.updated_at.to_rfc3339(),
                    memory.ttl_seconds,
                ],
            )?;
            Ok(memory)
        })
    }

    pub fn get_memory(&self, agent_id: &str, namespace: &str, key: &str) -> AppResult<AgentMemory> {
        self.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, agent_id, namespace, key, value, created_at, updated_at, ttl_seconds
                 FROM agent_memory
                 WHERE agent_id = ?1 AND namespace = ?2 AND key = ?3",
            )?;

            stmt.query_row(params![agent_id, namespace, key], |row| {
                Ok(AgentMemory {
                    id: row.get(0)?,
                    agent_id: row.get(1)?,
                    namespace: row.get(2)?,
                    key: row.get(3)?,
                    value: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                        .unwrap_or_default()
                        .with_timezone(&Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                        .unwrap_or_default()
                        .with_timezone(&Utc),
                    ttl_seconds: row.get(7)?,
                })
            })
            .map_err(|_| AppError::AgentNotFound(format!("{}/{}/{}", agent_id, namespace, key)))
        })
    }

    pub fn list_agent_memory(&self, agent_id: &str, namespace: Option<&str>) -> AppResult<Vec<AgentMemory>> {
        self.execute(|conn| {
            let (sql, ns) = if let Some(ns) = namespace {
                (
                    "SELECT id, agent_id, namespace, key, value, created_at, updated_at, ttl_seconds
                     FROM agent_memory WHERE agent_id = ?1 AND namespace = ?2
                     ORDER BY updated_at DESC",
                    Some(ns.to_string()),
                )
            } else {
                (
                    "SELECT id, agent_id, namespace, key, value, created_at, updated_at, ttl_seconds
                     FROM agent_memory WHERE agent_id = ?1
                     ORDER BY updated_at DESC",
                    None,
                )
            };

            let mut stmt = conn.prepare(sql)?;

            let mut memories = Vec::new();
            if let Some(ns) = &ns {
                let rows = stmt.query_map(params![agent_id, ns], |row| build_memory_row(row))?;
                for row in rows {
                    let mem: AgentMemory = row?;
                    if !mem.is_expired() {
                        memories.push(mem);
                    }
                }
            } else {
                let rows = stmt.query_map(params![agent_id], |row| build_memory_row(row))?;
                for row in rows {
                    let mem: AgentMemory = row?;
                    if !mem.is_expired() {
                        memories.push(mem);
                    }
                }
            };
            Ok(memories)
        })
    }

    pub fn log_tool_call(&self, agent_id: &str, req: CreateToolLogRequest) -> AppResult<ToolLog> {
        let log = ToolLog::new(
            agent_id.to_string(),
            req.tool_name,
            req.input,
            req.output,
            req.status,
            req.duration_ms,
        );

        self.execute(|conn| {
            conn.execute(
                "INSERT INTO tool_logs (id, agent_id, tool_name, input, output, status, duration_ms, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    log.id,
                    log.agent_id,
                    log.tool_name,
                    serde_json::to_string(&log.input)?,
                    serde_json::to_string(&log.output)?,
                    log.status.as_str(),
                    log.duration_ms,
                    log.created_at.to_rfc3339(),
                ],
            )?;
            Ok(log)
        })
    }

    pub fn get_tool_logs(&self, agent_id: &str, limit: usize) -> AppResult<Vec<ToolLog>> {
        self.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, agent_id, tool_name, input, output, status, duration_ms, created_at
                 FROM tool_logs WHERE agent_id = ?1
                 ORDER BY created_at DESC LIMIT ?2",
            )?;

            let rows = stmt.query_map(params![agent_id, limit], |row| {
                Ok(ToolLog {
                    id: row.get(0)?,
                    agent_id: row.get(1)?,
                    tool_name: row.get(2)?,
                    input: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
                    output: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
                    status: ToolStatus::from_str(&row.get::<_, String>(5)?),
                    duration_ms: row.get(6)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                        .unwrap_or_default()
                        .with_timezone(&Utc),
                })
            })?;

            let mut logs = Vec::new();
            for row in rows {
                logs.push(row?);
            }
            Ok(logs)
        })
    }

    // ─── Stats ──────────────────────────────────────────────────────

    pub fn get_stats(&self) -> AppResult<GraphStats> {
        self.execute(|conn| {
            let total_notes: usize = conn.query_row("SELECT COUNT(*) FROM notes", [], |r| r.get(0))?;
            let total_links: usize = conn.query_row("SELECT COUNT(*) FROM links", [], |r| r.get(0))?;
            let total_tags: usize = conn.query_row("SELECT COUNT(*) FROM tags", [], |r| r.get(0))?;

            let orphan_notes: usize = conn.query_row(
                "SELECT COUNT(*) FROM notes n
                 WHERE NOT EXISTS (SELECT 1 FROM links l WHERE l.source_note_id = n.id OR l.target_note_id = n.id)",
                [],
                |r| r.get(0),
            )?;

            let most_linked: Option<String> = conn
                .query_row(
                    "SELECT n.title FROM notes n
                     JOIN links l ON n.id = l.target_note_id
                     GROUP BY n.id ORDER BY COUNT(*) DESC LIMIT 1",
                    [],
                    |r| r.get(0),
                )
                .ok();

            Ok(GraphStats {
                total_notes,
                total_links,
                total_tags,
                orphan_notes,
                most_linked,
            })
        })
    }
}

// ─── Helper Functions ───────────────────────────────────────────────

fn ensure_tag(conn: &Connection, name: &str) -> AppResult<()> {
    conn.execute(
        "INSERT OR IGNORE INTO tags (id, name, created_at) VALUES (?1, ?2, ?3)",
        params![Uuid::new_v4().to_string(), name, Utc::now().to_rfc3339()],
    )?;
    Ok(())
}

fn get_tag_id(conn: &Connection, name: &str) -> AppResult<String> {
    let id: String = conn.query_row("SELECT id FROM tags WHERE name = ?1", params![name], |r| {
        r.get(0)
    })?;
    Ok(id)
}

fn get_note_tags(conn: &Connection, note_id: &str) -> AppResult<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT t.name FROM tags t JOIN note_tags nt ON t.id = nt.tag_id WHERE nt.note_id = ?1",
    )?;
    let rows = stmt.query_map(params![note_id], |row| row.get(0))?;
    let mut tags = Vec::new();
    for row in rows {
        tags.push(row?);
    }
    Ok(tags)
}

fn count_backlinks(conn: &Connection, note_id: &str) -> AppResult<usize> {
    let count: usize = conn.query_row(
        "SELECT COUNT(*) FROM links WHERE target_note_id = ?1",
        params![note_id],
        |r| r.get(0),
    )?;
    Ok(count)
}

fn count_wikilinks(conn: &Connection, note_id: &str) -> AppResult<usize> {
    let count: usize = conn.query_row(
        "SELECT COUNT(*) FROM links WHERE source_note_id = ?1",
        params![note_id],
        |r| r.get(0),
    )?;
    Ok(count)
}

fn build_note_summary(row: &rusqlite::Row) -> rusqlite::Result<NoteSummary> {
    let content: String = row.get(2)?;
    let preview = if content.len() > 200 {
        format!("{}...", &content[..200])
    } else {
        content
    };

    Ok(NoteSummary {
        id: row.get(0)?,
        title: row.get(1)?,
        preview,
        created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
            .unwrap_or_default()
            .with_timezone(&Utc),
        updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
            .unwrap_or_default()
            .with_timezone(&Utc),
        tag_count: 0,
        backlink_count: 0,
    })
}

fn build_memory_row(row: &rusqlite::Row) -> rusqlite::Result<AgentMemory> {
    Ok(AgentMemory {
        id: row.get(0)?,
        agent_id: row.get(1)?,
        namespace: row.get(2)?,
        key: row.get(3)?,
        value: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
        created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
            .unwrap_or_default()
            .with_timezone(&Utc),
        updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
            .unwrap_or_default()
            .with_timezone(&Utc),
        ttl_seconds: row.get(7)?,
    })
}
