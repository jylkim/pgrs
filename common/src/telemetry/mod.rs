use tracing_subscriber;

pub fn init_tracing() {
    tracing_subscriber::fmt::init();
}
