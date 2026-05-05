use boxcar_core::registry::ServerRegistry;
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

    // Build the registry. MCP server adapters will be registered here
    // once boxcar-github (and others) exist.
    let registry = ServerRegistry::new();
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
