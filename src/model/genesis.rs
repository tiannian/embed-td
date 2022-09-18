use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Genesis data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Genesis<AppState> {
    /// Time of genesis
    pub genesis_time: OffsetDateTime,

    /// Chain ID
    pub chain_id: String,

    /// Starting height of the blockchain
    #[serde(with = "super::serializers::from_str")]
    pub initial_height: i64,
    /*  */
    /* /// Consensus parameters */
    /* pub consensus_params: Params, */
    /*  */
    /* /// Validators */
    /* #[serde(default)] */
    /* pub validators: Vec<validator::Info>, */
    /*  */
    /* /// App hash */
    /* #[serde(with = "serializers::bytes::hexstring")] */
    /* pub app_hash: Vec<u8>, */
    /*  */
    /// App state
    pub app_state: AppState,
}
