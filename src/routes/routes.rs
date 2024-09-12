use axum::{
    routing::{get},
    Router,
};
use std::sync::Arc;
use crate::handlers::order::{get_order};
use crate::state::state::AppState;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/orders/:order_uid", get(get_order))
        .with_state(state)
}
