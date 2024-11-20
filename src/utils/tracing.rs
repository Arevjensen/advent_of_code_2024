use tracing::subscriber::set_global_default;

pub fn init_tracing() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .finish();

    let _ = set_global_default(subscriber);
}
