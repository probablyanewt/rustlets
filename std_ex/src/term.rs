use crate::fmt::Display;

type Ansi = &'static str;

const CLEAR_TERMINAL: Ansi = "\x1B[2J";
const CLEAR_LINE: Ansi = "\x1B[2K";
const BOLD_ON: Ansi = "\x1B[1m";
const BOLD_OFF: Ansi = "\x1B[22m";
const RESET: Ansi = "\x1B[0m";
const UNDERLINE_ON: Ansi = "\x1B[4m";
const UNDERLINE_OFF: Ansi = "\x1B[24m";
const OVERLINE_ON: Ansi = "\x1B[53m";
const OVERLINE_OFF: Ansi = "\x1B[55m";
const FRAMED_ON: Ansi = "\x1B[51m";
const ENCIRCLED_ON: Ansi = "\x1B[52m";
const FRAMED_ENCIRCLED_OFF: Ansi = "\x1B[54m";

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

const BG_COLOUR_OFF: Ansi = "\x1B[49m";
const BLACK_BG: Ansi = "\x1B[40m";
const BLUE_BG: Ansi = "\x1B[44m";
const CYAN_BG: Ansi = "\x1B[46m";
const GREEN_BG: Ansi = "\x1B[42m";
const GREY_BG: Ansi = "\x1B[100m";
const MAGENTA_BG: Ansi = "\x1B[45m";
const ORANGE_BG: Ansi = "\x1B[101m";
const RED_BG: Ansi = "\x1B[41m";
const YELLOW_BG: Ansi = "\x1B[43m";
const WHITE_BG: Ansi = "\x1B[47m";

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
    Off,
}

pub struct CustomColour {
    value: String,
    bg_value: String,
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

    pub fn paint_bg_with<T: AsRef<str>>(colour: Colour, s: T) -> String
    where
        T: Display,
    {
        format!("{}{}{}", colour.as_bg_str(), s, Colour::Off.as_bg_str())
    }

    pub fn paint_bg<T: AsRef<str>>(&self, s: T) -> String
    where
        T: Display,
    {
        format!("{}{}{}", &self.as_bg_str(), s, Colour::Off.as_bg_str())
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Colour::Off => COLOUR_OFF,
            Colour::Black => BLACK,
            Colour::Blue => BLUE,
            Colour::Cyan => CYAN,
            Colour::Green => GREEN,
            Colour::Grey => GREY,
            Colour::Magenta => MAGENTA,
            Colour::Orange => ORANGE,
            Colour::Red => RED,
            Colour::Yellow => YELLOW,
            Colour::White => WHITE,
        }
    }

    pub const fn as_bg_str(&self) -> &'static str {
        match self {
            Colour::Off => BG_COLOUR_OFF,
            Colour::Black => BLACK_BG,
            Colour::Blue => BLUE_BG,
            Colour::Cyan => CYAN_BG,
            Colour::Green => GREEN_BG,
            Colour::Grey => GREY_BG,
            Colour::Magenta => MAGENTA_BG,
            Colour::Orange => ORANGE_BG,
            Colour::Red => RED_BG,
            Colour::Yellow => YELLOW_BG,
            Colour::White => WHITE_BG,
        }
    }

    pub fn custom(r: usize, g: usize, b: usize) -> CustomColour {
        CustomColour {
            value: format!("\x1B[38;2;{r};{g};{b}m"),
            bg_value: format!("\x1B[48;2;{r};{g};{b}m"),
        }
    }
}

impl CustomColour {
    pub fn paint<T: AsRef<str>>(&self, s: T) -> String
    where
        T: Display,
    {
        format!("{}{}{}", self, s, Colour::Off)
    }

    pub fn paint_bg<T: AsRef<str>>(&self, s: T) -> String
    where
        T: Display,
    {
        format!("{}{}{}", self.bg_value, s, Colour::Off.as_bg_str())
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Display for CustomColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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
ansi_format_fn!(overline, OVERLINE_ON, OVERLINE_OFF);
ansi_format_fn!(frame, FRAMED_ON, FRAMED_ENCIRCLED_OFF);
ansi_format_fn!(encircled, ENCIRCLED_ON, FRAMED_ENCIRCLED_OFF);

pub fn clear() {
    eprintln!("{}", CLEAR_TERMINAL);
}

pub fn clear_line() {
    eprintln!("{}", CLEAR_LINE);
}

pub fn reset() {
    eprintln!("{}", RESET)
}

pub struct Printer {
    bold_prefix: &'static str,
    bold_suffix: &'static str,
    colour_prefix: &'static str,
    colour_suffix: &'static str,
    bg_prefix: &'static str,
    bg_suffix: &'static str,
    underline_prefix: &'static str,
    underline_suffix: &'static str,
    overline_prefix: &'static str,
    overline_suffix: &'static str,
    frame_prefix: &'static str,
    frame_suffix: &'static str,
}

impl Printer {
    pub const fn new() -> Self {
        Self {
            bold_prefix: "",
            bold_suffix: "",
            colour_prefix: "",
            colour_suffix: "",
            bg_prefix: "",
            bg_suffix: "",
            underline_prefix: "",
            underline_suffix: "",
            overline_prefix: "",
            overline_suffix: "",
            frame_prefix: "",
            frame_suffix: "",
        }
    }

    pub const fn set_bold(mut self) -> Self {
        self.bold_prefix = BOLD_ON;
        self.bold_suffix = BOLD_OFF;
        self
    }

    pub const fn set_underline(mut self) -> Self {
        self.underline_prefix = UNDERLINE_ON;
        self.underline_suffix = UNDERLINE_OFF;
        self
    }

    pub const fn set_colour(mut self, colour: Colour) -> Self {
        self.colour_prefix = colour.as_str();
        self.colour_suffix = COLOUR_OFF;
        self
    }

    pub const fn set_bg_colour(mut self, colour: Colour) -> Self {
        self.bg_prefix = colour.as_bg_str();
        self.bg_suffix = BG_COLOUR_OFF;
        self
    }

    pub const fn set_overline(mut self) -> Self {
        self.overline_prefix = OVERLINE_ON;
        self.overline_suffix = OVERLINE_OFF;
        self
    }

    /// Mutually exclusive with set_encircled
    pub const fn set_framed(mut self) -> Self {
        self.frame_prefix = FRAMED_ON;
        self.frame_suffix = FRAMED_ENCIRCLED_OFF;
        self
    }

    /// Mutually exclusive with set_framed
    pub const fn set_encircled(mut self) -> Self {
        self.frame_prefix = ENCIRCLED_ON;
        self.frame_suffix = FRAMED_ENCIRCLED_OFF;
        self
    }

    pub fn print<T: AsRef<str>>(&self, str: T)
    where
        T: Display,
    {
        print!("{}", self.format(str))
    }

    pub fn println<T: AsRef<str>>(&self, str: T)
    where
        T: Display,
    {
        println!("{}", self.format(str))
    }

    pub fn eprint<T: AsRef<str>>(&self, str: T)
    where
        T: Display,
    {
        print!("{}", self.format(str))
    }

    pub fn eprintln<T: AsRef<str>>(&self, str: T)
    where
        T: Display,
    {
        eprintln!("{}", self.format(str))
    }

    pub fn clear(&self) {
        clear();
    }

    pub fn clear_line(&self) {
        clear_line();
    }

    fn format<T: AsRef<str>>(&self, str: T) -> String
    where
        T: Display,
    {
        format!(
            "{}{}{}{}{}{}{str}{}{}{}{}{}{}",
            self.frame_prefix,
            self.overline_prefix,
            self.underline_prefix,
            self.bold_prefix,
            self.bg_prefix,
            self.colour_prefix,
            self.colour_suffix,
            self.bg_suffix,
            self.bold_suffix,
            self.underline_suffix,
            self.overline_suffix,
            self.frame_suffix,
        )
    }
}
