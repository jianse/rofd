pub mod de;
pub mod ser;

#[cfg(test)]
pub(crate) fn init_tracing_subscriber() {
    use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
    let fmt = fmt::layer()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true);
    let filter = filter::LevelFilter::TRACE;
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(fmt)
        .try_init();
}
