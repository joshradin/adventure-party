//! The index

use crate::web::index::get::index;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
}

mod get {
    use axum::response::IntoResponse;
    use pages::Index;
    use crate::Yew;

    pub async fn index() -> impl IntoResponse {
        Yew::<Index>::new()
    }
}