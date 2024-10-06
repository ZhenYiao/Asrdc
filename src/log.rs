use ansi_term::Color::{Blue, Red};
use time::UtcOffset;
use tracing::instrument::WithSubscriber;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, Layer, Registry};
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn logger_init() {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),
    );

    let formatting_layer = fmt::layer()
        .with_timer(local_time.clone())
        .with_level(true)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);



    let file_appender = rolling::daily("./config/log", "log");
    let file_layer = fmt::layer()
        .with_timer(local_time)
        .with_ansi(false)
        .with_writer(file_appender)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO)
        .boxed();

    Registry::default()
        .with(formatting_layer)
        .with(file_layer)
        .init();
}