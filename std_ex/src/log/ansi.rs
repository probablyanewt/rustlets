use crate::{
    collections::HashMap,
    fmt::{Display, format},
    log::LogLevel,
    sync::LazyLock,
    u8,
};

type Ansi = &'static str;

pub const CLEAR_TERMINAL: Ansi = "\x1B[2J";
pub const CLEAR_LINE: Ansi = "\x1B[2K";
pub const BOLD_ON: Ansi = "\x1B[1m";
pub const BOLD_OFF: Ansi = "\x1B[22m";
pub const RESET: Ansi = "\x1B[0m";
pub const UNDERLINE_ON: Ansi = "\x1B[4m";
pub const UNDERLINE_OFF: Ansi = "\x1B[24m";

const COLOUR_OFF: Ansi = "\x1B[39m";
const BLACK: Ansi = "\x1B[30m";
const BLUE: Ansi = "\x1B[34m";
const CYAN: Ansi = "\x1B[36m";
const GREEN: Ansi = "\x1B[32m";
const GREY: Ansi = "\x1B[90m";
const MAGENTA: Ansi = "\x1B[35m";
const ORANGE: Ansi = "\x1B[91m";
const RED: Ansi = "\x1B[31m";
const YELLOW: Ansi = "\x1B[33m";
const WHITE: Ansi = "\x1B[37m";

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

#[derive(Clone)]
pub enum Colour {
    Black,
    Blue,
    Cyan,
    Green,
    Grey,
    Magenta,
    Orange,
    Red,
    Yellow,
    White,
    Custom(u8, u8, u8),
    Off,
}

impl Colour {
    pub fn paint_with<T: AsRef<str>>(colour: Colour, s: T) -> String
    where
        T: Display,
    {
        format!("{}{}{}", colour, s, Colour::Off)
    }

    pub fn paint<T: AsRef<str>>(&self, s: T) -> String
    where
        T: Display,
    {
        format!("{}{}{}", &self, s, Colour::Off)
    }
}

impl From<LogLevel> for Colour {
    fn from(value: LogLevel) -> Self {
        DEFAULT_COLOURS.get(&value).unwrap_or(&Colour::Off).clone()
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Colour::Off => String::from(COLOUR_OFF),
            Colour::Black => String::from(BLACK),
            Colour::Blue => String::from(BLUE),
            Colour::Cyan => String::from(CYAN),
            Colour::Green => String::from(GREEN),
            Colour::Grey => String::from(GREY),
            Colour::Magenta => String::from(MAGENTA),
            Colour::Orange => String::from(ORANGE),
            Colour::Red => String::from(RED),
            Colour::Yellow => String::from(YELLOW),
            Colour::White => String::from(WHITE),
            Colour::Custom(r, g, b) => format(format_args!("\x1B[38;2;{r};{g};{b}m")),
        };
        write!(f, "{}", string)
    }
}

macro_rules! ansi_format_fn {
    ($fn_name: ident, $format_on: ident, $format_off: ident) => {
        pub fn $fn_name(s: String) -> String {
            format!("{}{}{}", $format_on, s, $format_off)
        }
    };
}

ansi_format_fn!(bold, BOLD_ON, BOLD_OFF);
ansi_format_fn!(underline, UNDERLINE_ON, UNDERLINE_OFF);
