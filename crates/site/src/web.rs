//! Contains web component

use crate::web::get::public;
use axum::handler::Handler;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use relative_path::{PathExt, RelativePath, RelativePathBuf};
use std::collections::VecDeque;
use std::env::current_dir;
use std::fs::FileType;
use std::path::{Path, PathBuf};
use tracing::{debug, instrument, trace};

mod index;

/// Creates the main router for the entire website
pub async fn router() -> Router {
    debug!("serving public/** at /** as fallback.");
    Router::new()
        .merge(index::router())
        .fallback(get(public))
        .merge(static_router())
}

/// Creates the static router, which creates routes for every entry within the 'static' dir
#[instrument]
fn static_router() -> Router {
    let mut router = Router::new();
    let static_dir = current_dir().unwrap().join("static");
    if static_dir.exists() && static_dir.is_dir() {
        let mut dir_queue = VecDeque::<RelativePathBuf>::new();
        dir_queue.push_back(RelativePathBuf::new());
        while let Some(dir_p) = dir_queue.pop_front() {
            let dir = dir_p.to_path(&static_dir);
            for entry in dir.read_dir().expect("could not read dir") {
                let entry = entry.expect("could not read dir entry");
                let ty = entry.file_type().expect("could not get file type");
                let relative = entry.path().relative_to(&static_dir).expect("could not create relative path");
                if ty.is_dir() {
                    dir_queue.push_back(relative);
                } else if ty.is_file() {
                    let path = format!("/{}", relative);
                    debug!("creating route GET:{path}");
                    router = router.route(
                        &path,
                        get(static_file_handler(
                            entry.path()
                        )),
                    );
                }
            }
        }
    }
    router
}

fn static_file_handler<P: AsRef<Path>>(path: P) -> (StatusCode, Vec<u8>) {
    let read = std::fs::read(path).expect("could not read file");
    (StatusCode::OK, read)
}

mod get {
    use axum::http::{StatusCode, Uri};
    use axum::response::{ErrorResponse, IntoResponse};
    use axum_extra::headers::ContentType;
    use axum_extra::TypedHeader;
    use mime::{Mime, APPLICATION_JAVASCRIPT, APPLICATION_OCTET_STREAM, TEXT_HTML, TEXT_PLAIN};
    use moka::future::Cache;
    use std::collections::HashMap;
    use std::env::current_dir;
    use std::ffi::OsStr;
    use std::fs::File;
    use std::io;
    use std::str::FromStr;
    use std::sync::{Arc, LazyLock, OnceLock};
    use std::time::Duration;
    use tokio::sync::RwLock;
    use tracing::{debug, info, instrument, trace};

    #[instrument(level = "trace")]
    pub async fn public(uri: Uri) -> Result<impl IntoResponse, StatusCode> {
        trace!("fallback public: {uri}");
        let path = uri.path().trim_start_matches('/').trim_end_matches('/');
        let working_dir = current_dir().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let public_path = working_dir.join("public").join(path);
        trace!("public_path: {public_path:?}");

        let media_type: Mime = match public_path.extension().and_then(|s| s.to_str()) {
            Some("js") => {
                APPLICATION_JAVASCRIPT
            }
            Some("wasm") => {
                Mime::from_str("application/wasm").expect("application/wasm is not a valid MIME type")
            }
            Some("html") => {
                TEXT_HTML
            }
            Some("txt") => {
                TEXT_PLAIN
            }
            _ => APPLICATION_OCTET_STREAM
        };

        let read = tokio::fs::read(public_path)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        let ret = (
            TypedHeader::<ContentType>(media_type.into()),
            read);
        Ok(ret)
    }
}

