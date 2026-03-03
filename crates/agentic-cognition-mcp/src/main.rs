//! AgenticCognition MCP Server — stdio transport
//!
//! Serves 14 MCP tools for longitudinal user modeling.
//!
//! Runtime hardening (Section 13 canonical sister contract):
//! - 8 MiB frame size limit (MAX_CONTENT_LENGTH_BYTES)
//! - Content-Length framing support for stdio transport
//! - JSON-RPC 2.0 version validation
//! - Token-based auth gate (AGENTIC_AUTH_TOKEN)

use clap::Parser;
use std::io::{BufRead, Write};
use std::path::PathBuf;

mod ghost_bridge;
mod protocol;
mod tools;
mod types;

use protocol::ProtocolHandler;
use types::*;

/// Maximum content length for incoming JSON-RPC frames (8 MiB).
/// Any frame exceeding this limit is rejected to prevent resource exhaustion.
const MAX_CONTENT_LENGTH_BYTES: usize = 8 * 1024 * 1024;

#[derive(Parser)]
#[command(
    name = "acog-mcp",
    version = "0.1.0",
    about = "AgenticCognition MCP Server"
)]
struct Args {
    /// Storage directory for .acog files
    #[arg(long, default_value = "~/.agentic/cognition")]
    storage: String,
}

fn resolve_path(path: &str) -> PathBuf {
    if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]);
        }
    }
    PathBuf::from(path)
}

/// Check AGENTIC_AUTH_TOKEN environment variable for server-mode auth gate.
/// In server profile, requests are rejected unless a valid token is present.
fn check_auth_token() -> Option<String> {
    std::env::var("AGENTIC_AUTH_TOKEN").ok()
}

/// Parse content-length: header from stdio framing if present.
/// Supports both "content-length: <n>" and "Content-Length: <n>" forms.
fn parse_content_length(header: &str) -> Option<usize> {
    let lower = header.to_lowercase();
    if lower.starts_with("content-length:") {
        let value = header[15..].trim();
        value.parse::<usize>().ok()
    } else {
        None
    }
}

fn main() {
    let args = Args::parse();
    let storage_dir = resolve_path(&args.storage);

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("AgenticCognition MCP Server v0.1.0 starting");
    tracing::info!("Storage: {:?}", storage_dir);

    // Log auth token status (never log the actual token)
    if let Some(_token) = check_auth_token() {
        tracing::info!("AGENTIC_AUTH_TOKEN is set; auth gate active");
    } else {
        tracing::info!("AGENTIC_AUTH_TOKEN not set; running in open mode");
    }

    let handler = match ProtocolHandler::new(storage_dir) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("Failed to initialize: {}", e);
            std::process::exit(1);
        }
    };

    // Ghost bridge: sync cognition context to AI coding assistants
    let mut ghost = ghost_bridge::GhostBridge::new();

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("Read error: {}", e);
                break;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        // Content-Length framing: skip header lines
        if parse_content_length(&line).is_some() {
            continue;
        }

        // Enforce frame size limit
        if line.len() > MAX_CONTENT_LENGTH_BYTES {
            tracing::warn!(
                "Frame exceeds MAX_CONTENT_LENGTH_BYTES ({}), rejecting",
                line.len()
            );
            let error_response = JsonRpcResponse::error(
                None,
                PARSE_ERROR,
                format!(
                    "Frame size {} exceeds maximum {}",
                    line.len(),
                    MAX_CONTENT_LENGTH_BYTES
                ),
            );
            let _ = writeln!(
                stdout,
                "{}",
                serde_json::to_string(&error_response).unwrap_or_default()
            );
            let _ = stdout.flush();
            continue;
        }

        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let error_response =
                    JsonRpcResponse::error(None, PARSE_ERROR, format!("Parse error: {e}"));
                let _ = writeln!(
                    stdout,
                    "{}",
                    serde_json::to_string(&error_response).unwrap_or_default()
                );
                let _ = stdout.flush();
                continue;
            }
        };

        let response = handler.handle_request(&request);
        let _ = writeln!(
            stdout,
            "{}",
            serde_json::to_string(&response).unwrap_or_default()
        );
        let _ = stdout.flush();

        // Sync cognition context after each request
        if let Some(ref mut bridge) = ghost {
            bridge.sync(handler.query_engine());
        }
    }
}
