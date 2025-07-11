#[doc(alias = "std::env")]
pub use std::env::*;
use std::ffi::OsStr;

/// Uses std::env::var to check if environment variable exists, by matching the result with a boolean.
///
/// Returns false if `std::var(key)` would return Err(_)
/// Returns true otherwise.
///
/// # Examples
///
/// ```rust
/// use std_ex::env;
///
/// let key = "HOME";
/// if env::var_exists(key) {
///     println!("{key} exists!");
/// } else {
///     println!("Couldn't interpret {key}");
/// }
/// ```
pub fn var_exists<K>(key: K) -> bool
where
    K: AsRef<OsStr>,
{
    match var(key) {
        Ok(_) => true,
        Err(_) => false,
    }
}
