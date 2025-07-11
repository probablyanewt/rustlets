pub use std::fmt::*;

/// Uses format! to turn any type which implements std::fmt::Debug into a pretty String.
///
/// # Examples
///
/// ```rust
/// use std_ex::fmt;
///
/// let data = Vec::from([1,2,3]);
/// let pretty_string = fmt::pretty(data);
///
/// ```
pub fn pretty(data: &dyn Debug) -> String {
    format!("{:#?}", data)
}
