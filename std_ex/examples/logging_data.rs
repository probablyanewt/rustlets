use std_ex::{
    fmt,
    log::{Log, LogLevel},
};

#[allow(dead_code)]
#[derive(Debug)]
struct Data<'a> {
    field: &'a str,
    other_field: i32,
}

pub fn main() {
    let log = Log::new_ex().set_log_level(LogLevel::Trace).done();
    let data = Data {
        field: "value",
        other_field: 123,
    };
    log.tracef("Trace log", &Vec::from(["1", "2"]));
    log.debugf("Debug log", &Vec::from([&data, &data]));
    log.infof("Info log", &data);
    log.warnf("Warn log", &data);
    log.errorf("Error log", &data);
    log.fatalf("Fatal log", &data);

    log.info(format!("Log by using format:\n{:#?}", &data));
    log.info(fmt::pretty(&data));
}
