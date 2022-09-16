#[derive(Debug, Clone)]
pub enum LogLevel {
    Info,
    Debug,
    Warn,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl LogLevel {
    pub(crate) fn to_str(&self) -> &'static str {
        match self {
            &Self::Error => "error",
            &Self::Info => "info",
            &Self::Warn => "warn",
            &Self::Debug => "debug",
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub home: String,
    pub loglevel: LogLevel,
    pub trace: bool,
}
