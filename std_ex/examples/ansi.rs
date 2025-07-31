use std_ex::{
    log::{Log, LogLevel},
    term,
};

pub fn main() {
    let log = Log::new_ex().set_log_level(LogLevel::Trace).done();
    log.trace(format!(
        "{},{},{}",
        term::Colour::Magenta.paint("Trace log"),
        term::Colour::Yellow.paint(" with some colour"),
        format!(
            "{}{}",
            term::Colour::Green.paint("deep nested formats"),
            term::Colour::Blue.paint(" everywhere")
        )
    ));
    log.debug("Debug log");
    log.info("Info log");
    log.warn("Warn log");
    log.error("Error log");
    log.fatal("Fatal log");
}
