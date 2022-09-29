use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("path utf8 error")]
    PathUtf8Error,

    #[error("No tendermint at this binary")]
    NoTendermint,

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),

    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    TomlSerError(#[from] toml::ser::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
