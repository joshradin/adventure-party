use crate::app::App;
use crate::web::router;
use clap::Parser;
use common::tracing::{LoggingOptions, Stdout};
use std::env::set_current_dir;
use tokio::net::TcpListener;
use tokio::time::Instant;
use tracing::level_filters::LevelFilter;
use tracing::{info, instrument, trace};
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry;
use tracing_subscriber::util::SubscriberInitExt;

pub use axum_yew_compat::Yew;
mod app;
mod axum_yew_compat;
mod components;
mod web;

#[tokio::main]
#[instrument]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let app = App::try_parse()?;

    if let Some(cwd) = app.cwd {
        set_current_dir(cwd)?;
    }

    let level_filter = if app.debug {
        LevelFilter::TRACE
    } else {
        LevelFilter::INFO
    };
    let options = LoggingOptions::new()
        .thread_names(true)
        .thread_ids(true)
        .target(Stdout(level_filter));

    registry()
        .with(options.into_layer()?)
        .with(ErrorLayer::default())
        .try_init()?;

    info!("starting site at {}:{}...", app.host, app.port);
    info!("initializing app...");
    let instant = Instant::now();
    let router = router().await;
    info!(
        "app initialization finished in {:.3} seconds",
        instant.elapsed().as_secs_f32()
    );

    let listener = TcpListener::bind((app.host, app.port)).await?;
    info!("serving at http://{:?}", listener.local_addr()?);
    axum::serve(listener, router).await?;
    Ok(())
}
