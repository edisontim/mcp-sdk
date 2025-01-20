use std::sync::{Arc, Mutex};

use mcp_sdk::{server::Server, transport::ServerStdioTransport, types::ServerCapabilities};
use serde_json::json;
use types::KnowledgeGraph;

use anyhow::Result;
mod tool_set;
mod types;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        // needs to be stderr due to stdio transport
        .with_writer(std::io::stderr)
        .init();
    let memory_file_path = "kb_memory.json";
    let kg = KnowledgeGraph::load_from_file(memory_file_path)?;
    let kg = Arc::new(Mutex::new(kg));
    let tools = tool_set::tool_set(kg, memory_file_path.to_string());

    let server = Server::builder(ServerStdioTransport)
        .capabilities(ServerCapabilities {
            tools: Some(json!({})),
            ..Default::default()
        })
        .tools(tools)
        .build();

    server
        .listen()
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;
    Ok(())
}
