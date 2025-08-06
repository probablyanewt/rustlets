pub enum Border {
    None,
    Hash,
    Line,
    DoubleLine,
}

struct BorderCharSet {
    tl: char,
    t: char,
    tr: char,
    l: char,
    r: char,
    bl: char,
    b: char,
    br: char,
}

const HASH_BORDER_SET: BorderCharSet = BorderCharSet {
    tl: '#',
    t: '#',
    tr: '#',
    l: '#',
    r: '#',
    bl: '#',
    b: '#',
    br: '#',
};

const LINE_BORDER_SET: BorderCharSet = BorderCharSet {
    tl: '\u{250C}',
    t: '\u{2500}',
    tr: '\u{2510}',
    l: '\u{2502}',
    r: '\u{2502}',
    bl: '\u{2514}',
    b: '\u{2500}',
    br: '\u{2518}',
};

const DOUBLE_LINE_BORDER_SET: BorderCharSet = BorderCharSet {
    tl: '\u{2554}',
    t: '\u{2550}',
    tr: '\u{2557}',
    l: '\u{2551}',
    r: '\u{2551}',
    bl: '\u{255A}',
    b: '\u{2550}',
    br: '\u{255D}',
};

impl Border {
    pub fn generate_border_components(
        &self,
        content_len: usize,
    ) -> (String, String, String, String) {
        let char_set = match self.as_border_char_set() {
            Some(char_set) => char_set,
            None => {
                return (
                    String::from(""),
                    String::from(""),
                    String::from(""),
                    String::from(""),
                );
            }
        };

        let mut top = String::from(char_set.tl);
        top.push_str(&String::from(char_set.t).repeat(content_len + 2));
        top.push(char_set.tr);
        top.push('\n');

        let mut bottom = String::from(char_set.bl);
        bottom.push_str(&String::from(char_set.b).repeat(content_len + 2));
        bottom.push(char_set.br);

        let mut prefix = String::from(char_set.l);
        prefix.push(' ');

        let mut suffix = String::from(' ');
        suffix.push(char_set.r);
        suffix.push('\n');

        (top, prefix, suffix, bottom)
    }

    fn as_border_char_set(&self) -> Option<BorderCharSet> {
        match self {
            Border::Hash => Some(HASH_BORDER_SET),
            Border::Line => Some(LINE_BORDER_SET),
            Border::DoubleLine => Some(DOUBLE_LINE_BORDER_SET),
            Border::None => None,
        }
    }
}
