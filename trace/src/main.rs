use tracing::info;
use tracing_appender::{
    non_blocking::{NonBlockingBuilder, WorkerGuard},
    rolling,
};
use tracing_subscriber::{filter, fmt, prelude::*};

// non_blocking writer creates a separate thread for writing traces
// For blocking writer, you can directly use rolling file instead.
fn trace_init() -> WorkerGuard {
    let mut layers = Vec::new();
    let file = rolling::never("./", "debug.trc");
    let (non_blocking, guard) = NonBlockingBuilder::default()
        .thread_name("tracing_thread")
        .finish(file);
    let dbg_trace_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_filter(filter::LevelFilter::DEBUG);
    layers.push(dbg_trace_layer);
    tracing_subscriber::registry().with(layers).init();
    guard
}

fn main() {
    let _guard = trace_init(); // guard has to be in scope for trace to work
    info!("Hello World");
}
