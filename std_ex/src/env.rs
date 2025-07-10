pub use std::env::*;
use std::ffi::OsStr;

pub fn var_exists<K>(key: K) -> bool
where
    K: AsRef<OsStr>,
{
    match var(key) {
        Ok(_) => true,
        Err(_) => false,
    }
}
