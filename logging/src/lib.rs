pub use slog::o as options;
use slog::{Drain, OwnedKV, SendSyncRefUnwindSafeKV};

pub use slog_scope::{info, debug, trace, warn, error};

pub fn setup_logging() -> slog_scope::GlobalLoggerGuard {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let drain = slog::LevelFilter::new(drain, slog::Level::Trace).fuse();
    let logger = slog::Logger::root(drain, slog::o!());
    slog_stdlog::init_with_level(log::Level::Info).unwrap();

    slog_scope::set_global_logger(logger)
}

pub fn open_scope<T, CB, R>(options: OwnedKV<T>, callback: CB) -> R
    where
        T: SendSyncRefUnwindSafeKV + 'static,
        CB: FnOnce() -> R {
    let logger = slog_scope::logger().new(options);

    slog_scope::scope(&logger, callback)
}
