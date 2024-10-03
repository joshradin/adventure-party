use common::tracing::{init_logging, LoggingOptions, Stdout};
use tracing::level_filters::LevelFilter;

// mod app;

#[tokio::main]
async fn main() {
    init_logging(
        LoggingOptions::new()
            .target(Stdout(LevelFilter::TRACE))
    );


}
