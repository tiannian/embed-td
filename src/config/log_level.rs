use crate::define_to_str_for_enum;

#[derive(Debug, Clone, Default)]
pub enum LogLevel {
    #[default]
    Info,
    Debug,
    Warn,
    Error,
}

define_to_str_for_enum!(
    LogLevel,
    Info => "info",
    Debug => "debug",
    Warn => "warn",
    Error => "error"
);

#[derive(Debug, Clone, Default)]
pub enum LogFormat {
    #[default]
    Plain,
    Json,
}

define_to_str_for_enum!(
    LogFormat,
    Plain => "plain",
    Json => "json"
);
