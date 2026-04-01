use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/notes", post(handlers::create_note).get(handlers::list_notes))
        .route(
            "/notes/:id",
            get(handlers::get_note)
                .put(handlers::update_note)
                .delete(handlers::delete_note),
        )
}
