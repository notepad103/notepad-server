use axum::{
    Router,
    routing::{post, put},
};

use crate::handlers;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/sections",
            post(handlers::create_section).get(handlers::list_sections),
        )
        .route(
            "/sections/:id",
            put(handlers::update_section).delete(handlers::delete_section),
        )
}
