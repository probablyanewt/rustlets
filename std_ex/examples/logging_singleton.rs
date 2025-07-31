use std_ex::log;

#[allow(dead_code)]
#[derive(Debug)]
struct Data<'a> {
    field: &'a str,
    other_field: i32,
}

pub fn main() {
    let data = Data {
        field: "value",
        other_field: 123,
    };
    log::set_log_level(log::LogLevel::Trace);
    log::set_timestamp(log::Timestamp::Time);
    log::set_timestamp(log::Timestamp::None);
    log::trace("Trace log");
    log::debug("Debug log");
    log::info("Info log");
    log::warn("Warn log");
    log::error("Error log");
    log::fatal("Fatal log");

    log::tracef("Trace log", &data);
    log::debugf("Debug log", &data);
    log::infof("Info log", &data);
    log::warnf("Warn log", &data);
    log::errorf("Error log", &data);
    log::fatalf("Fatal log", &data);
}
