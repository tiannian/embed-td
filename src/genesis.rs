use time::{Duration, OffsetDateTime};

use crate::{PublicKey, AlgorithmType};

/// Genesis data
pub struct Genesis<AppState> {
    /// Time of genesis
    pub genesis_time: OffsetDateTime,

    /// Chain ID
    pub chain_id: String,

    /// Starting height of the blockchain
    pub initial_height: i64,

    /// Consensus parameters
    pub consensus_params: ConsensusParams,

    /// Validators
    pub validators: Vec<ValidatorInfo>,

    /// App hash
    pub app_hash: Vec<u8>,

    /// App state
    pub app_state: AppState,
}

pub struct ConsensusParams {
    /// Block size parameters
    pub block: Block,

    /// Evidence parameters
    pub evidence: EvidenceParams,

    /// Validator parameters
    pub validator: ValidatorParams,

    /// Version parameters
    pub version: Option<VersionParams>,
}

/// Block size parameters
pub struct Block {
    /// Maximum number of bytes in a block
    pub max_bytes: u64,

    /// Maximum amount of gas which can be spent on a block
    pub max_gas: i64,

    /// This parameter has no value anymore in Tendermint-core
    pub time_iota_ms: i64,
}

pub struct EvidenceParams {
    /// Maximum allowed age for evidence to be collected
    pub max_age_num_blocks: u64,

    /// Max age duration
    pub max_age_duration: Duration,

    /// Max bytes
    pub max_bytes: i64,
}

pub struct ValidatorParams {
    pub pub_key_types: Vec<AlgorithmType>,
}

pub struct VersionParams {
    pub app_version: u64,
}

pub struct ValidatorInfo {
    pub address: [u8; 20],

    pub public_key: PublicKey,

    /// Validator voting power
    pub power: u64,

    /// Validator name
    pub name: Option<String>,

    /// Validator proposer priority
    pub proposer_priority: i64,
}
