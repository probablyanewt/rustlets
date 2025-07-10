pub use std::fmt::*;

pub fn pretty(data: &dyn Debug) -> String {
    format(format_args!("{:#?}", data))
}
