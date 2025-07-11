use logrs;

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
    logrs::set_log_level(logrs::LogLevel::Trace);
    logrs::set_timestamp(logrs::Timestamp::Time);
    logrs::set_timestamp(logrs::Timestamp::None);
    logrs::trace("Trace log");
    logrs::debug("Debug log");
    logrs::info("Info log");
    logrs::warn("Warn log");
    logrs::error("Error log");
    logrs::fatal("Fatal log");

    logrs::tracef("Trace log", &data);
    logrs::debugf("Debug log", &data);
    logrs::infof("Info log", &data);
    logrs::warnf("Warn log", &data);
    logrs::errorf("Error log", &data);
    logrs::fatalf("Fatal log", &data);
}
