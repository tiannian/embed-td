use serde::Serialize;
use time::Duration;

use super::Key;

/// Genesis data
#[derive(Clone, Debug, Serialize)]
pub struct Genesis<AppState> {
    /// Time of genesis
    pub genesis_time: String,

    /// Chain ID
    pub chain_id: String,

    /// Starting height of the blockchain
    pub initial_height: String,

    /// Consensus parameters
    pub consensus_params: ConsensusParams,

    /// Validators
    pub validators: Vec<ValidatorInfo>,

    /// App hash
    pub app_hash: Vec<u8>,

    /// App state
    pub app_state: AppState,
}

#[derive(Clone, Debug, Serialize)]
pub struct ConsensusParams {
    /// Block size parameters
    pub block: BlockSize,

    /// Evidence parameters
    pub evidence: EvidenceParams,

    /// Validator parameters
    pub validator: ValidatorParams,

    /// Version parameters
    pub version: Option<VersionParams>,
}

/// Block size parameters
#[derive(Serialize, Clone, Debug)]
pub struct BlockSize {
    /// Maximum number of bytes in a block
    pub max_bytes: String,

    /// Maximum amount of gas which can be spent on a block
    pub max_gas: String,

    /// This parameter has no value anymore in Tendermint-core
    pub time_iota_ms: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct EvidenceParams {
    /// Maximum allowed age for evidence to be collected
    pub max_age_num_blocks: String,

    /// Max age duration
    pub max_age_duration: Duration,

    /// Max bytes
    pub max_bytes: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct ValidatorParams {
    pub pub_key_types: Vec<PublicKeyAlgorithm>,
}

/// Public key algorithms
#[derive(Copy, Clone, Debug, Serialize)]
pub enum PublicKeyAlgorithm {
    /// ed25519
    #[serde(rename = "ed25519")]
    Ed25519,

    /// secp256k1
    #[serde(rename = "secp256k1")]
    Secp256k1,
}

#[derive(Clone, Serialize, Debug, Default)]
pub struct VersionParams {
    app_version: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ValidatorInfo {
    /// Validator account address
    pub address: String,

    /// Validator public key
    pub pub_key: Key,

    /// Validator voting power
    // Compatibility with genesis.json https://github.com/tendermint/tendermint/issues/5549
    #[serde(alias = "voting_power", alias = "total_voting_power")]
    pub power: u64,

    /// Validator name
    pub name: Option<String>,

    /// Validator proposer priority
    pub proposer_priority: i64,
}
