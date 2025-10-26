use std::fmt::Error;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::filter::EnvFilter;

pub fn init_tracing() -> Result<(), Error> {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_level(true);

    let fmt_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(fmt_filter)
        .init();
    Ok(())
}
