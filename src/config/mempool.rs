use time::Duration;

use super::{define_to_str_for_enum, define_build_mode_setter};

#[derive(Debug, Clone, Default)]
pub enum MempoolVersion {
    #[default]
    FIFO,
    Priority,
}

define_to_str_for_enum!(
    MempoolVersion,
    FIFO => "v0",
    Priority => "v1"
);

#[derive(Debug, Clone)]
pub struct MempoolConfig {
    /// Mempool version
    pub version: MempoolVersion,

    /// Recheck tx.
    pub recheck: bool,

    /// Broadcast to other node's mempool
    pub broadcast: bool,

    /// Size of mempool
    pub size: u64,

    /// Limit the total size of all txs in the mempool.
    /// This only accounts for raw transactions (e.g. given 1MB transactions and
    /// max_txs_bytes=5MB, mempool will only accept 5 transactions).
    pub max_txs_bytes: u64,

    /// Size of the cache (used to filter transactions we saw earlier) in transactions
    pub cache_size: u64,

    /// Do not remove invalid transactions from the cache (default: false)
    /// Set to true if it's not possible for any invalid transaction to become valid
    /// again in the future.
    pub keep_invalid_txs_in_cache: bool,

    /// Maximum size of a single transaction.
    /// NOTE: the max size of a tx transmitted over the network is {max_tx_bytes}.
    pub max_tx_bytes: u64,

    /// ttl-duration, if non-zero, defines the maximum amount of time a transaction
    /// can exist for in the mempool.
    ///
    /// Note, if ttl-num-blocks is also defined, a transaction will be removed if it
    /// has existed in the mempool at least ttl-num-blocks number of blocks or if it's
    /// insertion time into the mempool is beyond ttl-duration.
    pub ttl_duration: Duration,

    /// ttl-num-blocks, if non-zero, defines the maximum number of blocks a transaction
    /// can exist for in the mempool.
    ///
    /// Note, if ttl-duration is also defined, a transaction will be removed if it
    /// has existed in the mempool at least ttl-num-blocks number of blocks or if
    /// it's insertion time into the mempool is beyond ttl-duration.
    pub ttl_num_blocks: u64,
}

impl Default for MempoolConfig {
    fn default() -> Self {
        Self {
            version: Default::default(),
            recheck: true,
            broadcast: true,
            size: 5000,
            max_txs_bytes: 1073741824,
            cache_size: 10000,
            keep_invalid_txs_in_cache: false,
            max_tx_bytes: 1048576,
            ttl_duration: Duration::new(0, 0),
            ttl_num_blocks: 0,
        }
    }
}

impl MempoolConfig {
    define_build_mode_setter!(version, MempoolVersion);

    define_build_mode_setter!(recheck, bool);

    define_build_mode_setter!(broadcast, bool);

    define_build_mode_setter!(size, u64);

    define_build_mode_setter!(max_tx_bytes, u64);

    define_build_mode_setter!(cache_size, u64);

    define_build_mode_setter!(keep_invalid_txs_in_cache, bool);

    define_build_mode_setter!(ttl_duration, Duration);

    define_build_mode_setter!(ttl_num_blocks, u64);
}
