use logrs::{LogLevel, Logrs, ansi};

pub fn main() {
    let log = Logrs::new_ex().set_log_level(LogLevel::Trace).done();
    log.trace(format!(
        "{},{},{}",
        ansi::Colour::Magenta.paint("Trace log"),
        ansi::Colour::Yellow.paint(" with some colour"),
        format!(
            "{}{}",
            ansi::Colour::Green.paint("deep nested formats"),
            ansi::Colour::Blue.paint(" everywhere")
        )
    ));
    log.debug("Debug log");
    log.info("Info log");
    log.warn("Warn log");
    log.error("Error log");
    log.fatal("Fatal log");
}
