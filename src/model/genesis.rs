use serde::Serialize;

use crate::AlgorithmType;

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
    pub app_hash: String,

    /// App state
    pub app_state: Option<AppState>,
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
    pub version: VersionParams,
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
    pub max_age_duration: String,

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

impl From<crate::AlgorithmType> for PublicKeyAlgorithm {
    fn from(e: AlgorithmType) -> Self {
        match e {
            AlgorithmType::Ed25519 => PublicKeyAlgorithm::Ed25519,
            AlgorithmType::Secp256k1 => PublicKeyAlgorithm::Secp256k1,
            AlgorithmType::Sr25519 => panic!("no support this algorithm type."),
        }
    }
}

#[derive(Clone, Serialize, Debug, Default)]
pub struct VersionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ValidatorInfo {
    /// Validator account address
    pub address: String,

    /// Validator public key
    pub pub_key: Key,

    /// Validator voting power
    pub power: String,

    /// Validator name
    pub name: Option<String>,

    /// Validator proposer priority
    pub proposer_priority: String,
}
