use crate::{LogLevel, ansi, log_methods_at_level};
use std::{env, fmt, fmt::Display, str::FromStr, time};

const AWS_LAMBDA_ENV_VAR_NAME: &str = "AWS_LAMBDA_FUNCTION_NAME";
const LOG_LEVEL_ENV_VAR_NAME: &str = "LOG_LEVEL";

#[derive(Clone)]
pub enum Timestamp {
    None,
    Time,
    DateAndTime,
}

#[derive(Clone)]
pub struct Logrs {
    context: String,
    log_level: LogLevel,
    logging_function: fn(String) -> (),
    prefix: String,
    suppress_ansi: bool,
    timestamp: Timestamp,
}

pub struct LogrsBuilder {
    instance: Logrs,
}

impl Default for Logrs {
    fn default() -> Self {
        Self {
            context: String::from(""),
            log_level: LogLevel::Info,
            logging_function: |string: String| eprintln!("{string}"),
            prefix: String::from(""),
            suppress_ansi: Self::should_suppress_ansi(),
            timestamp: Timestamp::None,
        }
    }
}

impl Default for LogrsBuilder {
    fn default() -> Self {
        Self {
            instance: Logrs::default(),
        }
    }
}

impl Logrs {
    /// Create new instance with default settings
    pub fn new() -> Self {
        Logrs::default()
    }

    /// Create instance with optional customisations using builder pattern.
    pub fn new_ex() -> LogrsBuilder {
        LogrsBuilder::default()
    }

    /// A child logger is a new instance which inherits from it's parent, with the option of adding
    /// a prefix to each log
    pub fn child(&self, prefix: Option<&str>) -> Self {
        let mut new = self.clone();
        let prefix_value = match prefix {
            Some(value) => format!("{} {}", self.prefix, value),
            None => self.prefix.clone(),
        };
        new.prefix = prefix_value;
        new
    }

    /// A child logger is a new instance which inherits from it's parent, with the option of adding
    /// a prefix, and new context to each log
    pub fn child_with_new_context(&self, prefix: Option<&str>, context: &dyn fmt::Debug) -> Self {
        let context = format!("\nContext:\n{:#?}", context);
        let mut child = self.child(prefix);
        child.context = context;
        child
    }

    /// A child logger is a new instance which inherits from it's parent, with the option of adding
    /// a prefix, and additional context to its parent to each log
    pub fn child_with_additional_context(
        &self,
        prefix: Option<&str>,
        context: &dyn fmt::Debug,
    ) -> Self {
        let context = fmt::format(format_args!("{}\n{:#?}", self.context, context));
        let mut child = self.child(prefix);
        child.context = context;
        child
    }

    log_methods_at_level!(trace, tracef, tracep, Trace);
    log_methods_at_level!(debug, debugf, debugp, Debug);
    log_methods_at_level!(info, infof, infop, Info);
    log_methods_at_level!(warn, warnf, warnp, Warn);
    log_methods_at_level!(error, errorf, errorp, Error);
    log_methods_at_level!(fatal, fatalf, fatalp, Fatal);

    /// Logs formatted log to self.logging_function
    fn logger<T: AsRef<str>>(&self, level: LogLevel, msg: T)
    where
        T: Display,
    {
        if !self.should_log(level) {
            return;
        }

        let log = self.format_log(level, msg);
        (self.logging_function)(log)
    }

    /// format all strings into single string
    fn format_log<T: AsRef<str>>(&self, log_level: LogLevel, msg: T) -> String
    where
        T: Display,
    {
        let timestamp = match self.timestamp {
            Timestamp::None => String::from(""),
            Timestamp::Time => time::Local::now().format("[%H:%M:%S%.3f] ").to_string(),
            Timestamp::DateAndTime => time::Local::now()
                .format("[%Y-%m-%d %H:%M:%S%.3f] ")
                .to_string(),
        };
        let colour = match self.suppress_ansi {
            true => String::from(""),
            false => ansi::Colour::from(log_level).to_string(),
        };
        let level = log_level.to_string().to_uppercase();
        let colour_off = match self.suppress_ansi {
            true => String::from(""),
            false => ansi::Colour::Off.to_string(),
        };
        let prefix = &self.prefix;
        let context = &self.context;

        format!("{timestamp}{colour}{level}{colour_off}:{prefix} {msg} {context}")
    }

    /// should the logger log based on currently set internal log level, and environment variable
    /// LOG_LEVEL
    fn should_log(&self, log_level: LogLevel) -> bool {
        let level = match env::var(LOG_LEVEL_ENV_VAR_NAME) {
            Ok(level) => match LogLevel::from_str(level.as_str()) {
                Ok(level) => level,
                Err(_) => self.log_level,
            },
            Err(_) => self.log_level,
        };

        log_level >= level
    }

    /// should ansi codes be suppressed
    /// for example when used inside aws lambda
    fn should_suppress_ansi() -> bool {
        // Suppress ansi codes when run inside aws lambda
        env::var_exists(AWS_LAMBDA_ENV_VAR_NAME)
    }
}

impl LogrsBuilder {
    /// Disables internal use of ansi codes.
    /// For now, any ansi codes passed into the logging functions will be untouched.
    pub fn disable_ansi(&mut self) -> &mut Self {
        self.instance.suppress_ansi = false;
        self
    }

    /// Set default enabled log level to log at.
    /// Default: LogLevel::Info.
    pub fn set_log_level(&mut self, log_level: LogLevel) -> &mut Self {
        self.instance.log_level = log_level;
        self
    }

    /// Set timestamp configuration for log message.
    /// Timestamp::None -> No timestamp on log messages.
    /// Timestamp::Time -> Time is prefixed to log messages in the format [HH:MM:SS.ms].
    /// Timestamp::TimeAndDate -> Date and time are prefixed to messages in the format [YYYY-MM-DD
    /// HH:MM:SS.ms]
    /// Default: Timestamp::None.
    pub fn set_timestamp(&mut self, timestamp: Timestamp) -> &mut Self {
        self.instance.timestamp = timestamp;
        self
    }

    /// Set the logging function for logrs.
    /// Default: Logs to stderr using eprintln.
    pub fn set_logging_function(&mut self, f: fn(String) -> ()) -> &mut Self {
        self.instance.logging_function = f;
        self
    }

    /// Done building and return logrs instance.
    pub fn done(&self) -> Logrs {
        self.instance.clone()
    }
}

#[cfg(test)]
mod should_log_tests {
    use crate::Logrs;
    use crate::log_level::LogLevel;
    use crate::logrs::LOG_LEVEL_ENV_VAR_NAME;
    use std::env;

    #[test]
    fn should_return_true_if_info_configure_and_loggin_at_error() {
        assert!(Logrs::new().should_log(LogLevel::Error))
    }

    #[test]
    fn should_return_true_if_info_configured_and_logging_at_trace_with_env_var_as_trace() {
        unsafe {
            env::set_var(LOG_LEVEL_ENV_VAR_NAME, "trace");
        }
        assert!(Logrs::new().should_log(LogLevel::Trace))
    }
}
