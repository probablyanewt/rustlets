use crate::sync::{LazyLock, Mutex, Once};

pub use log::*;
pub use log_level::*;

pub mod ansi;
mod log;
mod log_level;

static LOG_BUILDER: LazyLock<Mutex<LogrsBuilder>> = LazyLock::new(|| Mutex::new(Log::new_ex()));
static LOG: LazyLock<Log> = LazyLock::new(|| match LOG_BUILDER.lock() {
    Ok(builder) => builder.done(),
    Err(_) => Log::default(),
});

static ONCE_SET_LOG_LEVEL: Once = Once::new();
/// Set the default log level on the global logrs instance.
/// This function will only modify the global instance once. Further calls do nothing.
/// Once the first logging function has been called, the instance can no longer be modified.
pub fn set_log_level(log_level: LogLevel) {
    ONCE_SET_LOG_LEVEL.call_once(|| match LOG_BUILDER.lock() {
        Ok(mut builder) => {
            builder.set_log_level(log_level);
        }
        Err(_) => return,
    })
}

static ONCE_SET_TIMESTAMP: Once = Once::new();
/// Set the timestamp settings on global logrs instance.
/// This function will only modify the global instance once. Further calls do nothing.
/// Once the first logging function has been called, the instance can no longer be modified.
pub fn set_timestamp(timestamp: Timestamp) {
    ONCE_SET_TIMESTAMP.call_once(|| match LOG_BUILDER.lock() {
        Ok(mut builder) => {
            builder.set_timestamp(timestamp);
        }
        Err(_) => return,
    })
}

static ONCE_SET_LOGGING_FUNCTION: Once = Once::new();
/// Set the logging function on the global logrs instance.
/// This function will only modify the global instance once. Further calls do nothing.
/// Once the first logging function has been called, the instance can no longer be modified.
pub fn set_logging_function(f: fn(String) -> ()) {
    ONCE_SET_LOGGING_FUNCTION.call_once(|| match LOG_BUILDER.lock() {
        Ok(mut builder) => {
            builder.set_logging_function(f);
        }
        Err(_) => return,
    })
}

static ONCE_DISABLE_ANSI: Once = Once::new();
/// Disable internal ansi codes on global logrs instance.
/// This function will only modify the global instance once. Further calls do nothing.
/// Once the first logging function has been called, the instance can no longer be modified.
pub fn disable_ansi() {
    ONCE_DISABLE_ANSI.call_once(|| match LOG_BUILDER.lock() {
        Ok(mut builder) => {
            builder.disable_ansi();
        }
        Err(_) => return,
    })
}

macro_rules! log_fns_at_level {
    ($log_name: ident, $logf_name:ident, $logp_name:ident) => {
        /// Log a str using a global log instance.
        pub fn $log_name(msg: &str) {
            LOG.$log_name(msg);
        }
        /// Log a str with data using a global log instance.
        pub fn $logf_name(msg: &str, data: &dyn crate::fmt::Debug) {
            LOG.$logf_name(msg, data);
        }
        /// Log data using crate_ex::fmt::pretty using a global log instance.
        pub fn $logp_name(data: &dyn crate::fmt::Debug) {
            LOG.$logp_name(data);
        }
    };
}

log_fns_at_level!(trace, tracef, tracep);
log_fns_at_level!(debug, debugf, debugp);
log_fns_at_level!(info, infof, infop);
log_fns_at_level!(warn, warnf, warnp);
log_fns_at_level!(error, errorf, errorp);
log_fns_at_level!(fatal, fatalf, fatalp);
