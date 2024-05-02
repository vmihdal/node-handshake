use cli::Cli;
use std::error::Error;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let filter = EnvFilter::default()
        .add_directive("TRACE".parse()?)
        .add_directive("tokio_util=off".parse()?);
    let layer = tracing_subscriber::fmt::layer()
        .pretty()
        .map_event_format(|format| {
            format
                .with_source_location(false)
                .with_line_number(false)
                .with_file(false)
                .with_level(false)
                .with_target(false)
        })
        .with_span_events(FmtSpan::CLOSE);

    tracing_subscriber::registry()
        .with(layer)
        .with(filter)
        .init();

    Cli::run().await?;

    Ok(())
}
