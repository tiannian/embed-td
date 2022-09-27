mod log_level;
pub use log_level::*;

mod fast_sync;
pub use fast_sync::*;

mod db_backend;
pub use db_backend::*;

mod p2p;
pub use p2p::*;

mod mempool;
pub use mempool::*;

mod state_sync;
pub use state_sync::*;

mod consensus;
pub use consensus::*;

mod tx_index;
pub use tx_index::*;

mod prometheus;
pub use prometheus::*;

/// Config for tendermint
#[derive(Debug, Clone)]
pub struct Config {
    /// Log level
    pub log_level: LogLevel,
    /// Readable name of this node. Default is a random name.
    pub moniker: String,

    /// Backend storage, default is goleveldb
    pub db_backend: DbBackend,

    /// Format for output log,
    pub log_format: LogFormat,

    /// TCP or UNIX socket address for Tendermint to listen on for
    /// connections from an external PrivValidator process
    pub priv_validator_laddr: String,

    /// If true, query the ABCI app on connecting to a new peer
    /// so the app can decide if we should keep the connection or not
    pub filter_peers: bool,

    /// pprof listen addr. Useful to debug tendermint.
    pub pprof_laddr: String,

    /// P2P config
    pub p2p: P2PConfig,

    /// Mempool config
    pub mempool: MempoolConfig,

    /// State sync rapidly bootstraps a new node by discovering, fetching, and restoring a state machine
    /// snapshot from peers instead of fetching and replaying historical blocks. Requires some peers in
    /// the network to take and serve state machine snapshots. State sync is not attempted if the node
    /// has any local state (LastBlockHeight > 0). The node will have a truncated block history,
    /// starting from the height of the snapshot.
    pub state_sync: Option<StateSyncConfig>,

    /// Version of fast sync.
    pub fast_sync: FastSyncVersion,

    /// Consensus config
    pub consensus: ConsensusConfig,

    /// What indexer to use for transactions
    ///
    /// The application will set which txs to index. In some cases a node operator will be able
    /// to decide which txs to index based on configuration set in the application.
    ///
    /// Options:
    ///   1) "null"
    ///   2) "kv" (default) - the simplest possible indexer, backed by key-value storage (defaults to levelDB; see DBBackend).
    /// 		- When "kv" is chosen "tx.height" and "tx.hash" will always be indexed.
    ///   3) "psql" - the indexer services backed by PostgreSQL.
    /// When "kv" or "psql" is chosen "tx.height" and "tx.hash" will always be indexed.
    pub tx_index: TxIndexConfig,

    /// When true, Prometheus metrics are served under /metrics on
    /// PrometheusListenAddr.
    /// Check out the documentation for the list of available metrics.
    pub prometheus: Option<PrometheusConfig>,
}

impl Default for Config {
    fn default() -> Self {
        let moniker = String::from("aa");

        Self {
            moniker,
            log_format: Default::default(),
            db_backend: Default::default(),
            log_level: Default::default(),
            priv_validator_laddr: Default::default(),
            filter_peers: false,
            pprof_laddr: Default::default(),
            p2p: Default::default(),
            mempool: Default::default(),
            state_sync: None,
            fast_sync: Default::default(),
            consensus: Default::default(),
            tx_index: Default::default(),
            prometheus: None,
        }
    }
}

#[macro_export]
macro_rules! define_to_str_for_enum {
    ($e:ident, $( $key:ident => $value:expr ),*) => {
        impl $e {
            pub fn to_str(&self) -> &'static str {
                match self {
                    $( Self::$key => $value, )*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_build_mode_setter {
    ($name:ident, str) => {
        pub fn $name(self, $name: &str) -> Self {
            let mut this = self;
            this.$name = String::from($name);
            this
        }
    };
    ($name:ident, $type:ty) => {
        pub fn $name(self, $name: $type) -> Self {
            let mut this = self;
            this.$name = $name;
            this
        }
    };
}

impl Config {
    define_build_mode_setter!(log_level, LogLevel);

    define_build_mode_setter!(moniker, str);

    define_build_mode_setter!(db_backend, DbBackend);

    define_build_mode_setter!(log_format, LogFormat);

    define_build_mode_setter!(priv_validator_laddr, str);

    define_build_mode_setter!(filter_peers, bool);

    define_build_mode_setter!(p2p, P2PConfig);

    define_build_mode_setter!(mempool, MempoolConfig);
}
