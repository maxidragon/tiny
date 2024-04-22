use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{create_url_handler, get_url_handler},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(create_url_handler))
        .route("/:code", get(get_url_handler))
        .with_state(app_state)
}
