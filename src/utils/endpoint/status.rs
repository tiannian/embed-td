use serde::Deserialize;
use time::OffsetDateTime;

use crate::{model, PublicKey, Result, ValidatorInfo};

use super::serializers;

/// Node information
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct NodeInfo {
    /// Protocol version information
    pub protocol_version: ProtocolVersionInfo,

    /// Node ID
    pub id: [u8; 20],

    /// Listen address
    pub listen_addr: String,

    /// Tendermint network / chain ID,
    pub network: String,

    /// Tendermint version
    pub version: String,

    /// Channels
    pub channels: String,

    /// Moniker
    pub moniker: String,

    /// Other status information
    pub other: OtherInfo,
}

/// Other information
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct OtherInfo {
    /// TX index status
    pub tx_index: TxIndexStatus,

    /// RPC address
    pub rpc_address: String,
}

/// Transaction index status
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum TxIndexStatus {
    /// Index is on
    #[serde(rename = "on")]
    On,

    /// Index is off
    #[serde(rename = "off")]
    Off,
}

/// Protocol version information
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ProtocolVersionInfo {
    /// P2P protocol version
    #[serde(with = "serializers::from_str")]
    pub p2p: u64,

    /// Block version
    #[serde(with = "serializers::from_str")]
    pub block: u64,

    /// App version
    #[serde(with = "serializers::from_str")]
    pub app: u64,
}

/// Status responses
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ResponseSerde {
    /// Node information
    pub node_info: NodeInfo,

    /// Sync information
    pub sync_info: SyncInfo,

    /// Validator information
    pub validator_info: model::ValidatorInfo,
}

/// Status responses
#[derive(Clone, Debug)]
pub struct Response {
    /// Node information
    pub node_info: NodeInfo,

    /// Sync information
    pub sync_info: SyncInfo,

    /// Validator information
    pub validator_info: ValidatorInfo,
}

impl Response {
    pub(crate) fn from_model(s: ResponseSerde) -> Result<Self> {
        let mut address = [0u8; 20];
        let bytes = hex::decode(s.validator_info.address)?;
        address.copy_from_slice(&bytes);

        let power = u64::from_str_radix(&s.validator_info.power, 10)?;
        let proposer_priority = i64::from_str_radix(&s.validator_info.proposer_priority, 10)?;

        Ok(Self {
            node_info: s.node_info,
            sync_info: s.sync_info,
            validator_info: ValidatorInfo {
                address,
                public_key: PublicKey::from_model(s.validator_info.pub_key)?,
                power,
                name: s.validator_info.name,
                proposer_priority,
            },
        })
    }
}

/// Sync information
#[derive(Clone, Debug, Deserialize)]
pub struct SyncInfo {
    /// Latest block hash
    #[serde(with = "serializers::hex_bytes32")]
    pub latest_block_hash: [u8; 32],

    /// Latest app hash
    #[serde(with = "serializers::hex_bytes")]
    pub latest_app_hash: Vec<u8>,

    /// Latest block height
    #[serde(with = "serializers::from_str")]
    pub latest_block_height: i64,

    /// Latest block time
    #[serde(with = "serializers::time")]
    pub latest_block_time: OffsetDateTime,

    /// Are we catching up?
    pub catching_up: bool,
}
