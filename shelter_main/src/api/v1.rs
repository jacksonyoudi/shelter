use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use crate::api::handlers;
use crate::state::ApplicationState;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route("/hello", get(handlers::hello::hello).with_state(state))

        .route("/login", post(handlers::login::login).with_state(state))
}