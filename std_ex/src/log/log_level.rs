use std::{collections::HashMap, sync::LazyLock};

use crate::{fmt::Display, str::FromStr, term::Colour};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub enum LogLevel {
    Trace = 1,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Silent,
}

const DEFAULT_COLOURS: LazyLock<HashMap<LogLevel, Colour>> = LazyLock::new(|| {
    let mut hashmap = HashMap::new();

    hashmap.insert(LogLevel::Trace, Colour::Grey);
    hashmap.insert(LogLevel::Debug, Colour::Magenta);
    hashmap.insert(LogLevel::Info, Colour::Blue);
    hashmap.insert(LogLevel::Warn, Colour::Yellow);
    hashmap.insert(LogLevel::Error, Colour::Red);
    hashmap.insert(LogLevel::Fatal, Colour::Red);

    hashmap
});

pub struct ParseLogLevelError;

impl FromStr for LogLevel {
    type Err = ParseLogLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let log_level = match s.to_lowercase().as_str() {
            "trace" => LogLevel::Trace,
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            "fatal" => LogLevel::Fatal,
            "silent" => LogLevel::Silent,
            _ => return Err(ParseLogLevelError),
        };

        Ok(log_level)
    }
}

impl From<LogLevel> for Colour {
    fn from(value: LogLevel) -> Self {
        DEFAULT_COLOURS.get(&value).unwrap_or(&Colour::Off).clone()
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
