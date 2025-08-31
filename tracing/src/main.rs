
use core::tracing::prelude::*;
mod core;
fn main() {
    // Initialize tracing subscriber (console logger)
    core::tracing::init();
    info!("Tracing initialized");
    warn!("Something went wrong!");
    debug!("Something happened");
    trace!("Something happened trace");
    error!("This is an error");
}
