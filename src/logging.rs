pub use log::{debug as log_debug, error as log_error, info as log_info, warn as log_warning};

const RUST_LOG: &str = "RUST_LOG";

pub(crate) fn init_logger() {
    if std::env::var(RUST_LOG).is_err() {
        #[cfg(debug_assertions)]
        std::env::set_var(RUST_LOG, "debug");
        #[cfg(not(debug_assertions))]
        std::env::set_var(RUST_LOG, "info");
    }

    env_logger::builder()
        .default_format()
        .format_timestamp_nanos()
        .format_indent(None)
        .init();
}
