//! for testing test-macros
#![cfg(feature = "test-macros")]
#![cfg(feature = "tracing")]

use common::test;
use common::tracing::info;

#[test(target = Stdout(LevelFilter::DEBUG))]
fn test_test_log_with_tracing() {
    info!("Hello, world!");
    assert!(true);
}


