use regex::Regex;
use std::sync::LazyLock;

/// Represents a parsed wiki-link from markdown content
#[derive(Debug, Clone, PartialEq)]
pub struct WikiLink {
    /// The target note title
    pub target: String,
    /// Optional display alias (from [[target|alias]] syntax)
    pub alias: Option<String>,
    /// Optional section anchor (from [[target#section]] syntax)
    pub section: Option<String>,
    /// The full raw text matched
    pub raw: String,
}

// Precompiled regex patterns for performance
static WIKILINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[\[([^\]\|#]+)(?:#([^\]\|]+))?(?:\|([^\]]+))?\]\]").unwrap());

static TAG_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:^|\s)#([\w][\w/-]*)").unwrap());

/// Extract all wiki-links from markdown content
///
/// Supports:
/// - `[[Note Title]]` — basic link
/// - `[[Note Title|Display Text]]` — aliased link
/// - `[[Note Title#Section]]` — section link
/// - `[[Note Title#Section|Display]]` — section + alias
pub fn extract_wikilinks(content: &str) -> Vec<WikiLink> {
    WIKILINK_RE
        .captures_iter(content)
        .map(|cap| {
            let target = cap[1].trim().to_string();
            let section = cap.get(2).map(|m| m.as_str().trim().to_string());
            let alias = cap.get(3).map(|m| m.as_str().trim().to_string());

            WikiLink {
                target: target.clone(),
                alias,
                section,
                raw: cap[0].to_string(),
            }
        })
        .collect()
}

/// Extract all hashtag-style tags from content
///
/// Matches: #tag, #my-tag, #nested/tag
/// Does NOT match: #123 (must start with a letter)
pub fn extract_tags(content: &str) -> Vec<String> {
    TAG_RE
        .captures_iter(content)
        .map(|cap| cap[1].to_string())
        .collect()
}

/// Replace wiki-links in content with a custom format
/// Useful for rendering or converting to other formats
pub fn replace_wikilinks<F>(content: &str, replacer: F) -> String
where
    F: Fn(&WikiLink) -> String,
{
    let links = extract_wikilinks(content);
    let mut result = content.to_string();
    for link in links.iter().rev() {
        let replacement = replacer(link);
        result = result.replace(&link.raw, &replacement);
    }
    result
}

/// Convert wiki-links to standard markdown links
pub fn wikilinks_to_markdown(content: &str) -> String {
    replace_wikilinks(content, |link| {
        let display = link.alias.as_ref().unwrap_or(&link.target);
        let target_slug = link.target.to_lowercase().replace(' ', "-");
        format!("[{}](/notes/{})", display, target_slug)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_wikilink() {
        let links = extract_wikilinks("Check out [[My Note]] for details");
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "My Note");
        assert_eq!(links[0].alias, None);
    }

    #[test]
    fn test_aliased_wikilink() {
        let links = extract_wikilinks("See [[Project Plan|the plan]] here");
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "Project Plan");
        assert_eq!(links[0].alias, Some("the plan".to_string()));
    }

    #[test]
    fn test_section_wikilink() {
        let links = extract_wikilinks("Jump to [[Guide#Installation]]");
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "Guide");
        assert_eq!(links[0].section, Some("Installation".to_string()));
    }

    #[test]
    fn test_multiple_wikilinks() {
        let links = extract_wikilinks("Link to [[A]] and [[B]] and [[C|see C]]");
        assert_eq!(links.len(), 3);
    }

    #[test]
    fn test_extract_tags() {
        let tags = extract_tags("This is #rust and #ai-agents with #nested/path");
        assert_eq!(tags, vec!["rust", "ai-agents", "nested/path"]);
    }

    #[test]
    fn test_no_numeric_tags() {
        let tags = extract_tags("Issue #123 is not a tag but #valid is");
        assert_eq!(tags, vec!["valid"]);
    }

    #[test]
    fn test_wikilinks_to_markdown() {
        let result = wikilinks_to_markdown("See [[My Note]] for info");
        assert_eq!(result, "See [My Note](/notes/my-note) for info");
    }
}
