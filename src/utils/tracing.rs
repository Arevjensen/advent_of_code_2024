use tracing::subscriber::set_global_default;

pub fn init_tracing() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .pretty()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .with_ansi(true)
        .finish();

    let _ = set_global_default(subscriber);
}
