#[macro_export]
macro_rules! log_methods_at_level {
    ($log_name: ident, $logf_name:ident, $logp_name:ident, $level: ident) => {
        /// Log a message
        pub fn $log_name<T: AsRef<str>>(&self, msg: T)
        where
            T: std::fmt::Display,
        {
            self.logger(LogLevel::$level, msg);
        }
        /// Log a message with data
        pub fn $logf_name<T: AsRef<str>>(&self, msg: T, data: &dyn std::fmt::Debug)
        where
            T: std::fmt::Display,
        {
            self.$log_name(format!("{}\n{:#?}", msg, data));
        }
        /// Log data using std_ex::fmt::pretty
        pub fn $logp_name(&self, data: &dyn std::fmt::Debug) {
            self.$log_name(std::fmt::pretty(data));
        }
    };
}

#[macro_export]
macro_rules! log_fns_at_level {
    ($log_name: ident, $logf_name:ident, $logp_name:ident) => {
        /// Log a str
        pub fn $log_name(msg: &str) {
            LOG.$log_name(msg);
        }
        /// Log a str with data
        pub fn $logf_name(msg: &str, data: &dyn fmt::Debug) {
            LOG.$logf_name(msg, data);
        }
        /// Log data using std_ex::fmt::pretty
        pub fn $logp_name(data: &dyn std::fmt::Debug) {
            LOG.$logp_name(data);
        }
    };
}
