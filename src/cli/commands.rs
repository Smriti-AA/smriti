use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "smriti",
    version,
    about = "Smriti — A lightning-fast knowledge store and memory layer for AI agents",
    long_about = "Smriti (Sanskrit: memory) — Self-hosted knowledge store built in Rust for agentic AI.\n\
                   Features: MCP server, agent memory, knowledge graph, wiki-links, full-text search, sync."
)]
pub struct Cli {
    /// Path to the database file
    #[arg(long, default_value = "notes.db", global = true)]
    pub db: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new note
    Create {
        /// Note title
        title: String,

        /// Note content (markdown)
        #[arg(long, short)]
        content: Option<String>,

        /// Read content from a file
        #[arg(long)]
        file: Option<String>,

        /// Tags to add (comma-separated)
        #[arg(long, short, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },

    /// Read a note by ID or title
    Read {
        /// Note ID or title
        id: String,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// List all notes
    List {
        /// Maximum number of notes to show
        #[arg(long, short, default_value = "20")]
        limit: usize,

        /// Filter by tag
        #[arg(long, short)]
        tag: Option<String>,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Search notes using full-text search
    Search {
        /// Search query
        query: String,

        /// Maximum results
        #[arg(long, short, default_value = "10")]
        limit: usize,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Show the knowledge graph
    Graph {
        /// Output format: json, dot, or text
        #[arg(long, short, default_value = "text")]
        format: String,

        /// Center on a specific note ID
        #[arg(long)]
        center: Option<String>,

        /// Depth for subgraph (with --center)
        #[arg(long, default_value = "2")]
        depth: usize,
    },

    /// Show database statistics
    Stats,

    /// Start the REST API server
    Serve {
        /// Host to bind to
        #[arg(long, default_value = "0.0.0.0")]
        host: String,

        /// Port to listen on
        #[arg(long, short, default_value = "3000")]
        port: u16,
    },

    /// Start the MCP server (JSON-RPC over stdio)
    Mcp,

    /// Sync notes with a remote server (Synology WebDAV or custom)
    Sync {
        /// Remote URL (e.g., https://nas.local:5006/notes)
        #[arg(long)]
        remote: String,

        /// Sync direction: push, pull, or both
        #[arg(long, default_value = "both")]
        direction: String,
    },

    /// Import notes from a directory of markdown files
    Import {
        /// Directory containing .md files
        path: String,

        /// Recursively import subdirectories
        #[arg(long, short)]
        recursive: bool,
    },

    /// Export notes to a directory of markdown files
    Export {
        /// Output directory
        path: String,

        /// Include frontmatter with metadata
        #[arg(long)]
        frontmatter: bool,
    },
}
