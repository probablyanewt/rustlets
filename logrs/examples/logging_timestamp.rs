use logrs::{LogLevel, Logrs, Timestamp};

pub fn main() {
    let log = Logrs::new_ex()
        .set_log_level(LogLevel::Trace)
        .set_timestamp(Timestamp::DateAndTime)
        .done();
    log.trace("Trace log");
    log.debug("Debug log");
    log.info("Info log");
    log.warn("Warn log");
    log.error("Error log");
    log.fatal("Fatal log");
}
