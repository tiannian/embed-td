use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Duration};

use super::Key;

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

    /// Consensus parameters
    pub consensus_params: ConsensusParams,

    /// Validators
    #[serde(default)]
    pub validators: Vec<ValidatorInfo>,

    /// App hash
    #[serde(with = "super::serializers::hexstring")]
    pub app_hash: Vec<u8>,

    /// App state
    pub app_state: AppState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsensusParams {
    /// Block size parameters
    pub block: BlockSize,

    /// Evidence parameters
    pub evidence: EvidenceParams,

    /// Validator parameters
    pub validator: ValidatorParams,

    /// Version parameters
    #[serde(skip)]
    pub version: Option<VersionParams>,
}

/// Block size parameters
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockSize {
    /// Maximum number of bytes in a block
    #[serde(with = "super::serializers::from_str")]
    pub max_bytes: u64,

    /// Maximum amount of gas which can be spent on a block
    #[serde(with = "super::serializers::from_str")]
    pub max_gas: i64,

    /// This parameter has no value anymore in Tendermint-core
    #[serde(with = "super::serializers::from_str", default = "BlockSize::default_time_iota_ms")]
    pub time_iota_ms: i64,
}

impl BlockSize {
    /// The default value for the `time_iota_ms` parameter.
    pub fn default_time_iota_ms() -> i64 {
        1000
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvidenceParams {
    /// Maximum allowed age for evidence to be collected
    #[serde(with = "super::serializers::from_str")]
    pub max_age_num_blocks: u64,

    /// Max age duration
    pub max_age_duration: Duration,

    /// Max bytes
    #[serde(with = "super::serializers::from_str", default)]
    pub max_bytes: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValidatorParams {
    pub pub_key_types: Vec<PublicKeyAlgorithm>,
}

/// Public key algorithms
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PublicKeyAlgorithm {
    /// ed25519
    #[serde(rename = "ed25519")]
    Ed25519,

    /// secp256k1
    #[serde(rename = "secp256k1")]
    Secp256k1,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct VersionParams {
    #[serde(with = "super::serializers::from_str")]
    app_version: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidatorInfo {
    /// Validator account address
    #[serde(with = "super::serializers::hexstring")]
    pub address: [u8; 20],

    /// Validator public key
    pub pub_key: Key,

    /// Validator voting power
    // Compatibility with genesis.json https://github.com/tendermint/tendermint/issues/5549
    #[serde(alias = "voting_power", alias = "total_voting_power")]
    pub power: u64,

    /// Validator name
    pub name: Option<String>,

    /// Validator proposer priority
    #[serde(skip)]
    pub proposer_priority: i64,
}

