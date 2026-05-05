use std::sync::Arc;
use boxcar_core::registry::ServerRegistry;
use boxcar_github::GitHubMcpServer;
use boxcar_server::{app, state::AppState};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    // Initialise tracing. Set RUST_LOG=boxcar=debug for verbose output.
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "boxcar=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut registry = ServerRegistry::new();

    // Register GitHub MCP adapter if GITHUB_TOKEN is set.
    match GitHubMcpServer::from_env() {
        Ok(server) => {
            registry.register(Arc::new(server));
            info!("Registered GitHub MCP server");
        }
        Err(e) => {
            tracing::warn!("GitHub MCP server not registered: {e}");
        }
    }

    let state = AppState::new(registry);
    let app = app::build(state);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");

    info!("Boxcar listening on {addr}");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}
