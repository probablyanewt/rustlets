use logrs::{LogLevel, Logrs};

#[allow(dead_code)]
#[derive(Debug)]
struct Data<'a> {
    field: &'a str,
    other_field: i32,
}

pub fn main() {
    let log = Logrs::new_ex().set_log_level(LogLevel::Trace).done();
    let data = Data {
        field: "value",
        other_field: 123,
    };
    let child1 = log.child_with_new_context(Some("first:"), &data);
    child1.info("log with child 1");

    let child2 = child1.child_with_additional_context(Some("second:"), &data);
    child2.infof("log with child 2", &data);
}
