use time::{Duration, OffsetDateTime};

use crate::{model, utils, AlgorithmType, PublicKey};

pub mod example {
    use serde::Serialize;

    #[derive(Debug, Clone, Serialize)]
    pub struct ExampleAppState {}
}

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
    pub evidence: Evidence,

    /// Validator parameters
    pub validator: Validator,

    /// Version parameters
    pub version: Option<Version>,
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

pub struct Evidence {
    /// Maximum allowed age for evidence to be collected
    pub max_age_num_blocks: u64,

    /// Max age duration
    pub max_age_duration: Duration,

    /// Max bytes
    pub max_bytes: i64,
}

pub struct Validator {
    pub pub_key_types: Vec<AlgorithmType>,
}

pub struct Version {
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

impl ValidatorInfo {
    pub fn generate(public_key: PublicKey) -> Self {
        Self {
            address: public_key.address(),
            public_key,
            power: 10,
            name: None,
            proposer_priority: 0,
        }
    }
}

impl<AppState> Genesis<AppState> {
    pub fn generate(public_key: PublicKey) -> Genesis<example::ExampleAppState> {
        let block = Block {
            max_bytes: 22020096,
            max_gas: -1,
            time_iota_ms: 1000,
        };

        let evidence = Evidence {
            max_age_num_blocks: 100000,
            max_age_duration: Duration::days(2000),
            max_bytes: 1048576,
        };

        let validator = Validator {
            pub_key_types: vec![AlgorithmType::Ed25519],
        };

        let consensus_params = ConsensusParams {
            block,
            evidence,
            validator,
            version: None,
        };

        let chain_id = String::from("test-chain");

        let validator_info = ValidatorInfo::generate(public_key);

        Genesis {
            genesis_time: OffsetDateTime::now_utc(),
            chain_id,
            initial_height: 0,
            consensus_params,
            validators: vec![validator_info],
            app_hash: Vec::new(),
            app_state: example::ExampleAppState {},
        }
    }

    pub(crate) fn into_model(self) -> model::Genesis<AppState> {
        let mut validators = Vec::with_capacity(self.validators.len());

        for v in self.validators {
            let vi = model::ValidatorInfo {
                address: hex::encode(v.address),
                pub_key: v.public_key.into_model(),
                power: v.power,
                name: v.name,
                proposer_priority: v.proposer_priority,
            };

            validators.push(vi);
        }

        let block = model::BlockSize {
            max_bytes: format!("{}", self.consensus_params.block.max_bytes),
            max_gas: format!("{}", self.consensus_params.block.max_gas),
            time_iota_ms: format!("{}", self.consensus_params.block.time_iota_ms),
        };

        let evidence = model::EvidenceParams {
            max_bytes: format!("{}", self.consensus_params.evidence.max_bytes),
            max_age_duration: format!("{}", self.consensus_params.evidence.max_age_duration),
            max_age_num_blocks: format!("{}", self.consensus_params.evidence.max_age_num_blocks),
        };

        let validator = model::ValidatorParams {
            pub_key_types: self
                .consensus_params
                .validator
                .pub_key_types
                .into_iter()
                .map(|e| e.into())
                .collect(),
        };

        let version = self.consensus_params.version.map(|f| model::VersionParams {
            app_version: format!("{}", f.app_version),
        });

        let consensus_params = model::ConsensusParams {
            block,
            evidence,
            validator,
            version,
        };

        model::Genesis {
            chain_id: self.chain_id,
            genesis_time: utils::to_rfc3339_nanos(self.genesis_time),
            initial_height: format!("{}", self.initial_height),
            app_hash: hex::encode(self.app_hash),
            validators,
            consensus_params,
            app_state: self.app_state,
        }
    }
}
