use time::Duration;

use super::define_build_mode_setter;

#[derive(Debug, Clone)]
pub struct StateSyncConfig {
    /// RPC servers (comma-separated) for light client verification of the synced state machine and
    /// retrieval of state data for node bootstrapping. Also needs a trusted height and corresponding
    /// header hash obtained from a trusted source, and a period during which validators can be trusted.
    ///
    /// For Cosmos SDK-based chains, trust_period should usually be about 2/3 of the unbonding time (~2
    /// weeks) during which they can be financially punished (slashed) for misbehavior.
    pub rpc_servers: Vec<String>,
    pub trust_height: u64,
    pub trust_hash: String,
    pub trust_period: Duration,

    /// Time to spend discovering snapshots before initiating a restore.
    pub discovery_time: Duration,

    /// The timeout duration before re-requesting a chunk, possibly from a different
    /// peer (default: 1 minute).
    pub chunk_request_timeout: Duration,

    /// The number of concurrent chunk fetchers to run (default: 1).
    pub chunk_fetchers: u64,
}

impl Default for StateSyncConfig {
    fn default() -> Self {
        Self {
            rpc_servers: Default::default(),
            trust_height: 0,
            trust_hash: Default::default(),
            trust_period: Duration::hours(168),
            discovery_time: Duration::new(15, 0),
            chunk_request_timeout: Duration::new(10, 0),
            chunk_fetchers: 4,
        }
    }
}

impl StateSyncConfig {
    define_build_mode_setter!(rpc_servers, Vec<String>);

    define_build_mode_setter!(trust_height, u64);

    define_build_mode_setter!(trust_hash, str);

    define_build_mode_setter!(trust_period, Duration);

    define_build_mode_setter!(discovery_time, Duration);

    define_build_mode_setter!(chunk_request_timeout, Duration);

    define_build_mode_setter!(chunk_fetchers, u64);
}
