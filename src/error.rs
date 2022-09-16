use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No tendermint at this binary")]
    NoTendermint,

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
