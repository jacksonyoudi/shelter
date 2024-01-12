use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::api::handlers;
use crate::state::ApplicationState;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route("/hello", get(handlers::hello::hello).with_state(state))
}