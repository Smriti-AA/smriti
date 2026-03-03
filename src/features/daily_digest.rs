use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::AppResult;
use crate::features::smart_links::SmartLinker;
use crate::models::*;
use crate::storage::Database;

/// Daily Digest — an automated summary of your knowledge base activity
///
/// This is a "pull people back" feature: every day, it generates an
/// insight report showing what you wrote, what connections were discovered,
/// and what you might want to revisit. Think of it as your personal
/// research assistant giving you a morning briefing.
///
/// Features:
/// - Recently modified notes summary
/// - Newly discovered connections (from SmartLinker)
/// - Trending topics based on tag frequency
/// - Orphan notes that need attention
/// - "On this day" — notes from exactly N days/months/years ago

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyDigest {
    pub date: String,
    pub summary: DigestSummary,
    pub recent_notes: Vec<NoteSummary>,
    pub suggested_links: Vec<super::smart_links::LinkSuggestion>,
    pub trending_topics: Vec<TrendingTopic>,
    pub orphan_notes: Vec<NoteSummary>,
    pub on_this_day: Vec<NoteSummary>,
    pub stats: GraphStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestSummary {
    pub notes_created_today: usize,
    pub notes_modified_today: usize,
    pub new_links_today: usize,
    pub total_words_today: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingTopic {
    pub tag: String,
    pub count: usize,
    pub trend: String, // "rising", "stable", "declining"
}

pub struct DigestGenerator;

impl DigestGenerator {
    /// Generate today's daily digest
    pub fn generate(db: &Database) -> AppResult<DailyDigest> {
        let today = Utc::now().format("%Y-%m-%d").to_string();

        // Get recent notes (last 24 hours)
        let all_notes = db.list_notes(&NoteListQuery {
            limit: 1000,
            offset: 0,
            sort: SortOrder::UpdatedDesc,
            tag: None,
        })?;

        let yesterday = Utc::now() - Duration::hours(24);
        let recent: Vec<NoteSummary> = all_notes
            .iter()
            .filter(|n| n.updated_at > yesterday)
            .cloned()
            .collect();

        let created_today = all_notes
            .iter()
            .filter(|n| n.created_at > yesterday)
            .count();

        // Get smart link suggestions
        let suggestions = SmartLinker::find_suggestions(db, 5)?;

        // Calculate trending topics
        let trending = Self::calculate_trending_topics(db)?;

        // Find orphan notes (no links)
        let stats = db.get_stats()?;

        // "On this day" — notes created on this date in previous years
        let on_this_day = Self::find_on_this_day(db, &all_notes)?;

        // Count total words written today
        let total_words: usize = recent
            .iter()
            .map(|n| n.preview.split_whitespace().count())
            .sum();

        Ok(DailyDigest {
            date: today,
            summary: DigestSummary {
                notes_created_today: created_today,
                notes_modified_today: recent.len(),
                new_links_today: 0, // Would need link timestamps
                total_words_today: total_words,
            },
            recent_notes: recent.into_iter().take(10).collect(),
            suggested_links: suggestions,
            trending_topics: trending,
            orphan_notes: Vec::new(), // Would need a dedicated query
            on_this_day,
            stats,
        })
    }

    fn calculate_trending_topics(db: &Database) -> AppResult<Vec<TrendingTopic>> {
        // Get notes from the last 7 days and count tag frequency
        let all_notes = db.list_notes(&NoteListQuery {
            limit: 1000,
            offset: 0,
            sort: SortOrder::UpdatedDesc,
            tag: None,
        })?;

        let week_ago = Utc::now() - Duration::days(7);
        let mut tag_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for summary in &all_notes {
            if summary.updated_at > week_ago {
                let note = db.get_note(&summary.id)?;
                for tag in &note.tags {
                    *tag_counts.entry(tag.clone()).or_insert(0) += 1;
                }
            }
        }

        let mut topics: Vec<TrendingTopic> = tag_counts
            .into_iter()
            .map(|(tag, count)| TrendingTopic {
                tag,
                count,
                trend: if count > 3 {
                    "rising".into()
                } else {
                    "stable".into()
                },
            })
            .collect();

        topics.sort_by(|a, b| b.count.cmp(&a.count));
        topics.truncate(10);

        Ok(topics)
    }

    fn find_on_this_day(
        _db: &Database,
        all_notes: &[NoteSummary],
    ) -> AppResult<Vec<NoteSummary>> {
        let today = Utc::now();
        let month_day = today.format("%m-%d").to_string();

        let on_this_day: Vec<NoteSummary> = all_notes
            .iter()
            .filter(|n| {
                let note_md = n.created_at.format("%m-%d").to_string();
                note_md == month_day
                    && n.created_at.format("%Y").to_string()
                        != today.format("%Y").to_string()
            })
            .cloned()
            .collect();

        Ok(on_this_day)
    }

    /// Print digest to terminal in a nice format
    pub fn print_digest(digest: &DailyDigest) {
        println!("\n╔══════════════════════════════════════╗");
        println!("║   Daily Knowledge Digest — {}   ║", digest.date);
        println!("╚══════════════════════════════════════╝\n");

        println!("📊 Today's Activity");
        println!("   Notes created:  {}", digest.summary.notes_created_today);
        println!("   Notes modified: {}", digest.summary.notes_modified_today);
        println!("   Words written:  {}", digest.summary.total_words_today);
        println!();

        if !digest.recent_notes.is_empty() {
            println!("📝 Recently Updated");
            for note in &digest.recent_notes {
                println!(
                    "   • {} ({})",
                    note.title,
                    note.updated_at.format("%H:%M")
                );
            }
            println!();
        }

        if !digest.suggested_links.is_empty() {
            println!("🔗 Suggested Connections");
            for link in &digest.suggested_links {
                println!(
                    "   {} ↔ {} ({:.0}%)",
                    link.source_title,
                    link.target_title,
                    link.confidence * 100.0
                );
                println!("     Reason: {}", link.reason);
            }
            println!();
        }

        if !digest.trending_topics.is_empty() {
            println!("🔥 Trending Topics");
            for topic in &digest.trending_topics {
                println!(
                    "   #{} — {} notes ({})",
                    topic.tag, topic.count, topic.trend
                );
            }
            println!();
        }

        if !digest.on_this_day.is_empty() {
            println!("📅 On This Day");
            for note in &digest.on_this_day {
                println!(
                    "   {} ({})",
                    note.title,
                    note.created_at.format("%Y-%m-%d")
                );
            }
            println!();
        }

        println!("📈 Knowledge Base: {} notes, {} links, {} tags",
            digest.stats.total_notes,
            digest.stats.total_links,
            digest.stats.total_tags,
        );
        if let Some(most) = &digest.stats.most_linked {
            println!("   Most connected: {}", most);
        }
        println!();
    }
}
