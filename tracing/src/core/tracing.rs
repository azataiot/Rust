// src/core/tracing.rs

use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Initialize the tracing
pub fn init() {
    // Set Log level from RUST_LOG, or default to info
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Pretty, human-readable console output
    let fmt = tracing_subscriber::fmt::layer()
        .with_file(true) // include file name
        .with_line_number(true) // include line number
        .with_thread_ids(true) // include thread id
        .with_thread_names(true) // include thread name
        .with_ansi(true) // enable ANSI color codes
        .with_target(false); // hide module path

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt)
        .try_init() // ignore "already initialized"
        .ok();
}

/// Prelude: re-export macros and types from the `tracing` crate for convenience
pub mod prelude {
    pub use tracing::{
        debug, error, info, trace, warn,
    };
}
