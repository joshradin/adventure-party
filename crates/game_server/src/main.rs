use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::spawn;
use tokio::time::sleep;
use tracing::info;
use tracing::level_filters::LevelFilter;
use common::tracing::{init_logging, LoggingOptions, Stdout};
use crate::app::App;

mod app;

#[tokio::main]
async fn main() {
    init_logging(
        LoggingOptions::new()
            .target(Stdout(LevelFilter::TRACE))
    );

    let mut app = App::new("127.0.0.1");

    let mut updates = Arc::new(AtomicUsize::new(0));

    spawn({
        let updates = updates.clone();
        async move {
            let started = Instant::now();
            loop {
                sleep(Duration::from_secs(30)).await;
                let elapsed = started.elapsed().as_secs_f64();
                let updates = updates.load(Ordering::Relaxed) as f64;

                let ups = updates / elapsed;
                info!("server updates: {ups:.3}/s");
            }
        }
    });

    loop {
        app.update().await;
        updates.fetch_add(1, Ordering::Relaxed);
    }
}
