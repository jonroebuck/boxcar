use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use crate::{routes, state::AppState};

/// Build the Axum application with all routes and middleware.
/// Kept separate from main so it can be used in integration tests.
pub fn build(state: AppState) -> Router {
    Router::new()
        .route("/health", get(routes::health))
        .route("/tools", get(routes::list_tools))
        .route("/tools/call", post(routes::call_tool))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
