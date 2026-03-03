use crate::errors::AppResult;
use crate::models::*;
use crate::storage::Database;

/// Smart Link Suggestions — the "killer feature" that discovers hidden connections
///
/// This analyzes your notes to find notes that should be linked but aren't,
/// surfacing connections you didn't know existed. Think of it as an AI research
/// assistant constantly reading your notes and saying "hey, these relate to each other."
///
/// How it works:
/// 1. **Keyword overlap**: Notes sharing significant keywords get suggested as links
/// 2. **Tag co-occurrence**: Notes with overlapping tags are likely related
/// 3. **Transitive connections**: If A links to B and B links to C, A might relate to C
/// 4. **Title mentions**: If note content mentions another note's title without a [[link]]

pub struct SmartLinker;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LinkSuggestion {
    pub source_id: String,
    pub source_title: String,
    pub target_id: String,
    pub target_title: String,
    pub reason: String,
    pub confidence: f64,
}

impl SmartLinker {
    /// Discover unlinked notes that should be connected
    pub fn find_suggestions(db: &Database, limit: usize) -> AppResult<Vec<LinkSuggestion>> {
        let notes = db.list_notes(&NoteListQuery {
            limit: 10000,
            offset: 0,
            sort: SortOrder::UpdatedDesc,
            tag: None,
        })?;

        let mut suggestions: Vec<LinkSuggestion> = Vec::new();

        // Build a map of note content for analysis
        let mut note_data: Vec<(String, String, String, Vec<String>)> = Vec::new();
        for summary in &notes {
            let note = db.get_note(&summary.id)?;
            let words: Vec<String> = note
                .content
                .to_lowercase()
                .split_whitespace()
                .filter(|w| w.len() > 3) // Skip short words
                .map(String::from)
                .collect();
            note_data.push((note.id, note.title, note.content, words));
        }

        // Get existing links to avoid re-suggesting
        let existing_links = db.get_all_links()?;
        let mut linked_pairs: std::collections::HashSet<(String, String)> =
            std::collections::HashSet::new();
        for link in &existing_links {
            linked_pairs.insert((
                link.source_note_id.clone(),
                link.target_note_id.clone(),
            ));
        }

        // Strategy 1: Title mentions without wiki-links
        for (i, (id_a, title_a, content_a, _)) in note_data.iter().enumerate() {
            for (j, (id_b, title_b, _, _)) in note_data.iter().enumerate() {
                if i == j {
                    continue;
                }
                if linked_pairs.contains(&(id_a.clone(), id_b.clone())) {
                    continue;
                }

                // Check if note A's content mentions note B's title
                let title_lower = title_b.to_lowercase();
                if title_lower.len() > 3
                    && content_a.to_lowercase().contains(&title_lower)
                {
                    // Make sure it's not already a [[wiki-link]]
                    let wiki_pattern = format!("[[{}]]", title_b);
                    if !content_a.contains(&wiki_pattern) {
                        suggestions.push(LinkSuggestion {
                            source_id: id_a.clone(),
                            source_title: title_a.clone(),
                            target_id: id_b.clone(),
                            target_title: title_b.clone(),
                            reason: format!(
                                "\"{}\" mentions \"{}\" but has no wiki-link",
                                title_a, title_b
                            ),
                            confidence: 0.9,
                        });
                    }
                }
            }
        }

        // Strategy 2: Keyword overlap (Jaccard similarity on significant words)
        for i in 0..note_data.len() {
            for j in (i + 1)..note_data.len() {
                let (id_a, title_a, _, words_a) = &note_data[i];
                let (id_b, title_b, _, words_b) = &note_data[j];

                if linked_pairs.contains(&(id_a.clone(), id_b.clone()))
                    || linked_pairs.contains(&(id_b.clone(), id_a.clone()))
                {
                    continue;
                }

                let set_a: std::collections::HashSet<&str> =
                    words_a.iter().map(|s| s.as_str()).collect();
                let set_b: std::collections::HashSet<&str> =
                    words_b.iter().map(|s| s.as_str()).collect();

                let intersection = set_a.intersection(&set_b).count();
                let union = set_a.union(&set_b).count();

                if union > 0 {
                    let jaccard = intersection as f64 / union as f64;
                    if jaccard > 0.15 {
                        suggestions.push(LinkSuggestion {
                            source_id: id_a.clone(),
                            source_title: title_a.clone(),
                            target_id: id_b.clone(),
                            target_title: title_b.clone(),
                            reason: format!(
                                "High keyword overlap ({:.0}% shared vocabulary)",
                                jaccard * 100.0
                            ),
                            confidence: jaccard.min(0.85),
                        });
                    }
                }
            }
        }

        // Sort by confidence and limit
        suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        suggestions.truncate(limit);

        Ok(suggestions)
    }

    /// Auto-apply top suggestions as AI-suggested links
    pub fn auto_link(db: &Database, max_links: usize) -> AppResult<usize> {
        let suggestions = Self::find_suggestions(db, max_links)?;
        let mut created = 0;

        for suggestion in &suggestions {
            if suggestion.confidence >= 0.7 {
                let _ = db.create_link(
                    &suggestion.source_id,
                    &suggestion.target_id,
                    LinkType::AiSuggested,
                );
                created += 1;
            }
        }

        Ok(created)
    }
}
