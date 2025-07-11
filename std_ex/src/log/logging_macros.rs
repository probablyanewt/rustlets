#[macro_export]
macro_rules! log_methods_at_level {
    ($log_name: ident, $logf_name:ident, $logp_name:ident, $level: ident) => {
        /// Log a message
        pub fn $log_name<T: AsRef<str>>(&self, msg: T)
        where
            T: crate::fmt::Display,
        {
            self.logger(LogLevel::$level, msg);
        }
        /// Log a message with data
        pub fn $logf_name<T: AsRef<str>>(&self, msg: T, data: &dyn crate::fmt::Debug)
        where
            T: crate::fmt::Display,
        {
            self.$log_name(format!("{}\n{:#?}", msg, data));
        }
        /// Log data using crate_ex::fmt::pretty
        pub fn $logp_name(&self, data: &dyn crate::fmt::Debug) {
            self.$log_name(crate::fmt::pretty(data));
        }
    };
}

#[macro_export]
macro_rules! log_fns_at_level {
    ($log_name: ident, $logf_name:ident, $logp_name:ident) => {
        /// Log a str using a global logrs instance.
        pub fn $log_name(msg: &str) {
            LOG.$log_name(msg);
        }
        /// Log a str with data using a global logrs instance.
        pub fn $logf_name(msg: &str, data: &dyn crate::fmt::Debug) {
            LOG.$logf_name(msg, data);
        }
        /// Log data using crate_ex::fmt::pretty using a global logrs instance.
        pub fn $logp_name(data: &dyn crate::fmt::Debug) {
            LOG.$logp_name(data);
        }
    };
}
