use tracing::{debug, info};
use tracing_appender::{
    non_blocking::{NonBlockingBuilder, WorkerGuard},
    rolling,
};
use tracing_subscriber::Registry;
use tracing_subscriber::{filter, fmt, prelude::*, reload};

// non_blocking writer creates a separate thread for writing traces
// For blocking writer, you can directly use rolling file instead.
fn trace_init(
    level: filter::LevelFilter,
) -> (WorkerGuard, reload::Handle<filter::LevelFilter, Registry>) {
    let mut layers = Vec::new();
    let file = rolling::never("./", "debug.trc");
    let (non_blocking, guard) = NonBlockingBuilder::default()
        .thread_name("tracing_thread")
        .finish(file);
    let (filter, reload_handle) = reload::Layer::new(level);
    let dbg_trace_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_filter(filter);
    layers.push(dbg_trace_layer);
    tracing_subscriber::registry().with(layers).init();
    (guard, reload_handle)
}

fn main() {
    let (_guard, reload_handle) = trace_init(filter::LevelFilter::INFO); // guard has to be in scope for trace to work
    info!("Hello World");
    debug!("This is debug");
    let _ = reload_handle.modify(|filter| *filter = filter::LevelFilter::DEBUG);
    debug!("This is debug after enabling");
    info!("This is info after enabling debug");
}
