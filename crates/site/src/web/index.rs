//! The index

use crate::web::index::get::index;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new().route("/", get(index))
}

mod get {
    use crate::Yew;
    use axum::response::IntoResponse;
    use pages::Index;

    pub async fn index() -> impl IntoResponse {
        Yew::<Index>::new()
    }
}
