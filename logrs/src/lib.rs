use std::fmt;
use std::sync::LazyLock;

pub use log_level::*;
pub use logrs::*;

pub mod ansi;
mod log_level;
mod logging_macros;
mod logrs;

static LOG: LazyLock<Logrs> = LazyLock::new(|| Logrs::default());

log_fns_at_level!(trace, tracef, tracep);
log_fns_at_level!(debug, debugf, debugp);
log_fns_at_level!(info, infof, infop);
log_fns_at_level!(warn, warnf, warnp);
log_fns_at_level!(error, errorf, errorp);
log_fns_at_level!(fatal, fatalf, fatalp);
