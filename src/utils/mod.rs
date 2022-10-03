mod rfc3339;
pub use rfc3339::*;
use serde::Deserialize;

/* #[cfg(feature = "smol-backend")] */
/* pub mod http { */
/*     include!("smol_http.rs"); */
/* } */

pub mod smol_http;

pub mod endpoint;

/// JSON-RPC response wrapper (i.e. message envelope)
#[derive(Debug, Deserialize, Clone)]
pub struct Wrapper<R> {
    /// JSON-RPC version
    pub(crate) jsonrpc: String,

    /// Identifier included in request
    pub(crate) id: String,

    /// Results of request (if successful)
    pub(crate) result: Option<R>,

    /// Error message if unsuccessful
    pub(crate) error: Option<String>,
}

pub trait IntoGetQuery {
    fn into_get_query(self) -> String;
}

// pub async fn status(
