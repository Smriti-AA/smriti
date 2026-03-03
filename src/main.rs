use clap::Parser;
use std::sync::Arc;

use smriti::cli::commands::{Cli, Commands};
use smriti::cli::handlers;
use smriti::features::smart_links::SmartLinker;
use smriti::mcp::server::McpServer;
use smriti::storage::Database;
use smriti::sync::engine::{SyncDirection, SyncEngine};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let cli = Cli::parse();
    let db = Arc::new(Database::new(&cli.db)?);

    match cli.command {
        Commands::Create {
            title,
            content,
            file,
            tags,
        } => {
            handlers::handle_create(&db, title, content, file, tags)?;
        }

        Commands::Read { id, json } => {
            handlers::handle_read(&db, id, json)?;
        }

        Commands::List { limit, tag, json } => {
            handlers::handle_list(&db, limit, tag, json)?;
        }

        Commands::Search { query, limit, json } => {
            handlers::handle_search(&db, query, limit, json)?;
        }

        Commands::Graph {
            format,
            center,
            depth,
        } => {
            handlers::handle_graph(&db, format, center, depth)?;
        }

        Commands::Stats => {
            handlers::handle_stats(&db)?;

            // Also show smart link suggestions
            println!();
            let suggestions = SmartLinker::find_suggestions(&db, 5)?;
            if !suggestions.is_empty() {
                println!("Suggested Connections:");
                for s in &suggestions {
                    println!(
                        "  {} ↔ {} ({:.0}% - {})",
                        s.source_title,
                        s.target_title,
                        s.confidence * 100.0,
                        s.reason
                    );
                }
            }
        }

        Commands::Serve { host, port } => {
            println!(
                "\n  Smriti API Server v{}\n",
                env!("CARGO_PKG_VERSION")
            );
            println!("  Endpoints:");
            println!("    GET    /health");
            println!("    POST   /api/v1/notes");
            println!("    GET    /api/v1/notes");
            println!("    GET    /api/v1/notes/:id");
            println!("    PUT    /api/v1/notes/:id");
            println!("    DELETE /api/v1/notes/:id");
            println!("    GET    /api/v1/notes/search?q=...");
            println!("    GET    /api/v1/notes/:id/backlinks");
            println!("    GET    /api/v1/notes/:id/links");
            println!("    GET    /api/v1/graph");
            println!("    GET    /api/v1/graph/:id");
            println!("    GET    /api/v1/stats");
            println!("    POST   /api/v1/agent/:id/memory");
            println!("    GET    /api/v1/agent/:id/memory");
            println!("    POST   /api/v1/agent/:id/tool-logs");
            println!("    GET    /api/v1/agent/:id/tool-logs");
            println!();

            smriti::api::server::start_server(db, &host, port).await?;
        }

        Commands::Mcp => {
            let mcp = McpServer::new(db);
            mcp.run()?;
        }

        Commands::Sync {
            remote,
            direction,
        } => {
            let device_id = hostname::get()
                .map(|h| h.to_string_lossy().to_string())
                .unwrap_or_else(|_| "unknown".into());

            let engine = SyncEngine::new(device_id, db.clone());
            let dir = SyncDirection::from_str(&direction);

            // Detect if remote is a file path or URL
            if remote.starts_with("http://") || remote.starts_with("https://") {
                println!("WebDAV sync with {} ({})...", remote, direction);
                println!("Note: Set SYNC_USER and SYNC_PASS environment variables for auth");

                let username =
                    std::env::var("SYNC_USER").unwrap_or_else(|_| "admin".into());
                let password =
                    std::env::var("SYNC_PASS").unwrap_or_else(|_| "".into());

                let result = engine
                    .sync_webdav(&remote, &username, &password, dir)
                    .await?;

                println!("Sync complete:");
                println!("  Pushed: {}", result.pushed);
                println!("  Pulled: {}", result.pulled);
                if !result.errors.is_empty() {
                    println!("  Errors:");
                    for err in &result.errors {
                        println!("    - {}", err);
                    }
                }
            } else {
                println!("Filesystem sync with {} ({})...", remote, direction);
                let result = engine.sync_filesystem(&remote, dir)?;
                println!("Sync complete:");
                println!("  Pushed: {}", result.pushed);
                println!("  Pulled: {}", result.pulled);
            }
        }

        Commands::Import { path, recursive } => {
            handlers::handle_import(&db, path, recursive)?;
        }

        Commands::Export { path, frontmatter } => {
            handlers::handle_export(&db, path, frontmatter)?;
        }
    }

    Ok(())
}
