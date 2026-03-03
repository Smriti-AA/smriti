use serde::{Deserialize, Serialize};

/// Frontmatter metadata parsed from YAML header in notes
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Frontmatter {
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    /// Catch-all for custom fields
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Parse YAML frontmatter from the top of a markdown document
///
/// Expects content starting with `---\n` and ending with `---\n`
pub fn parse_frontmatter(content: &str) -> Option<(Frontmatter, &str)> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }

    let after_first = &trimmed[3..].trim_start_matches(['\r', '\n']);
    if let Some(end_pos) = after_first.find("\n---") {
        let yaml_str = &after_first[..end_pos];
        let rest = &after_first[end_pos + 4..];
        let rest = rest.trim_start_matches(['\r', '\n']);

        // Parse YAML manually (simple key: value pairs)
        let fm = parse_simple_yaml(yaml_str);
        Some((fm, rest))
    } else {
        None
    }
}

/// Strip frontmatter and return just the content body
pub fn strip_frontmatter(content: &str) -> &str {
    match parse_frontmatter(content) {
        Some((_, body)) => body,
        None => content,
    }
}

/// Simple YAML parser for frontmatter (avoids adding a full YAML dependency)
fn parse_simple_yaml(yaml: &str) -> Frontmatter {
    let mut fm = Frontmatter::default();

    for line in yaml.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "tags" => {
                    fm.tags = parse_yaml_list(value);
                }
                "aliases" => {
                    fm.aliases = parse_yaml_list(value);
                }
                "date" => {
                    fm.date = Some(value.to_string());
                }
                "status" => {
                    fm.status = Some(value.to_string());
                }
                _ => {
                    fm.extra.insert(
                        key.to_string(),
                        serde_json::Value::String(value.to_string()),
                    );
                }
            }
        }
    }

    fm
}

/// Parse a YAML-style inline list: [item1, item2] or comma-separated values
fn parse_yaml_list(value: &str) -> Vec<String> {
    let value = value.trim();
    if value.starts_with('[') && value.ends_with(']') {
        value[1..value.len() - 1]
            .split(',')
            .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else if value.contains(',') {
        value
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else if !value.is_empty() {
        vec![value.to_string()]
    } else {
        Vec::new()
    }
}

/// Extract headings from markdown content for outline generation
pub fn extract_headings(content: &str) -> Vec<(usize, String)> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                let level = trimmed.chars().take_while(|&c| c == '#').count();
                let text = trimmed[level..].trim().to_string();
                if !text.is_empty() && level <= 6 {
                    return Some((level, text));
                }
            }
            None
        })
        .collect()
}

/// Count words in markdown content (excludes frontmatter)
pub fn word_count(content: &str) -> usize {
    let body = strip_frontmatter(content);
    body.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let content = "---\ntags: [rust, ai]\ndate: 2024-01-01\n---\n# Hello\nBody text";
        let (fm, body) = parse_frontmatter(content).unwrap();
        assert_eq!(fm.tags, vec!["rust", "ai"]);
        assert_eq!(fm.date, Some("2024-01-01".to_string()));
        assert!(body.starts_with("# Hello"));
    }

    #[test]
    fn test_no_frontmatter() {
        let content = "# Just a heading\nSome text";
        assert!(parse_frontmatter(content).is_none());
    }

    #[test]
    fn test_extract_headings() {
        let content = "# Title\n## Section 1\ntext\n### Sub\n## Section 2";
        let headings = extract_headings(content);
        assert_eq!(headings.len(), 4);
        assert_eq!(headings[0], (1, "Title".to_string()));
        assert_eq!(headings[1], (2, "Section 1".to_string()));
    }

    #[test]
    fn test_word_count() {
        let content = "---\ntags: [test]\n---\nHello world this is a test";
        assert_eq!(word_count(content), 6);
    }
}
