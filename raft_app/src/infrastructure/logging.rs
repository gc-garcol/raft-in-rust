pub fn init_logging() {
    // Initialize the logger with detailed configuration
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or("info")
            .default_write_style_or("always")
    )
    .format_timestamp_millis()
    .format_level(true)
    .format_target(true)
    .init();
}
