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

use crate::{defined, model};

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

    /// Data dis
    pub data_dir: String,

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
    pub fast_sync: Option<FastSyncVersion>,

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
            fast_sync: Some(FastSyncVersion::default()),
            consensus: Default::default(),
            tx_index: Default::default(),
            prometheus: None,
            data_dir: String::new(),
        }
    }
}

impl Config {
    pub fn new(data_dir: &str) -> Self {
        Self {
            data_dir: String::from(data_dir),
            ..Default::default()
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
    ($name:ident, $type:ty, option, $enable: ident) => {
        pub fn $enable(self, $name: $type) -> Self {
            let mut this = self;
            this.$name = Some($name);
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

    define_build_mode_setter!(state_sync, StateSyncConfig, option, enable_state_sync);

    define_build_mode_setter!(fast_sync, FastSyncVersion, option, enable_fast_sync);

    define_build_mode_setter!(consensus, ConsensusConfig);

    define_build_mode_setter!(tx_index, TxIndexConfig);

    define_build_mode_setter!(prometheus, PrometheusConfig, option, enabel_prometheus);
}

impl Config {
    pub(crate) fn into_model(self, base_dir: &str) -> model::Config {
        let rpc = {
            let laddr = format!("{}/{}", base_dir, defined::RPC_UNIX_SOCKET_FILE);

            model::Rpc {
                laddr,
                cors_allowed_origins: Default::default(),
                cors_allowed_headers: Default::default(),
                cors_allowed_methods: Default::default(),
                grpc_laddr: Default::default(),
                unsafe_opt: true,
                max_open_connections: 900,
                max_subscription_clients: 100,
                max_subscriptions_per_client: 5,
                experimental_subscription_buffer_size: 200,
                experimental_websocket_write_buffer_size: 200,
                experimental_close_on_slow_client: false,
                timeout_broadcast_tx_commit: String::from("10s"),
                max_body_bytes: 1000000,
                max_header_bytes: 1048576,
                tls_key_file: Default::default(),
                tls_cert_file: Default::default(),
                pprof_laddr: self.pprof_laddr,
                grpc_max_open_connections: 900,
            }
        };

        let p2p = {
            let addr_book_file = format!("{}/{}", base_dir, defined::ADDR_BOOK_FILE);
            let persistent_peers_max_dial_period = format!(
                "{}s",
                self.p2p.persistent_peers_max_dial_period.whole_seconds()
            );
            let flush_throttle_timeout =
                format!("{}ms", self.p2p.flush_throttle_timeout.whole_milliseconds());
            let handshake_timeout = format!("{}s", self.p2p.handshake_timeout.whole_seconds());
            let dial_timeout = format!("{}s", self.p2p.dial_timeout.whole_seconds());

            model::P2P {
                laddr: self.p2p.laddr,
                external_address: self.p2p.external_address,
                seeds: self.p2p.seeds.join(","),
                persistent_peers: self.p2p.persistent_peers.join(","),
                upnp: self.p2p.upnp,
                addr_book_file,
                addr_book_strict: !self.p2p.local_net,
                max_num_inbound_peers: self.p2p.max_num_inbound_peers,
                max_num_outbound_peers: self.p2p.max_num_outbound_peers,
                unconditional_peer_ids: self.p2p.unconditional_peer_ids.join(","),
                persistent_peers_max_dial_period,
                flush_throttle_timeout,
                max_packet_msg_payload_size: self.p2p.max_packet_msg_payload_size,
                send_rate: self.p2p.send_rate,
                recv_rate: self.p2p.recv_rate,
                pex: self.p2p.pex,
                seed_mode: self.p2p.seed_mode,
                private_peer_ids: self.p2p.private_peer_ids.join(","),
                allow_duplicate_ip: self.p2p.allow_duplicate_ip,
                handshake_timeout,
                dial_timeout,
            }
        };

        let mempool = {
            let ttl_duration = format!("{}s", self.mempool.ttl_duration.whole_seconds());

            model::Mempool {
                version: String::from(self.mempool.version.to_str()),
                wal_dir: Default::default(),
                size: self.mempool.size,
                max_tx_bytes: self.mempool.max_tx_bytes,
                cache_size: self.mempool.cache_size,
                keep_invalid_txs_in_cache: self.mempool.keep_invalid_txs_in_cache,
                max_txs_bytes: self.mempool.max_txs_bytes,
                max_batch_bytes: 0,
                ttl_duration,
                ttl_num_blocks: self.mempool.ttl_num_blocks,
                recheck: self.mempool.recheck,
                broadcast: self.mempool.broadcast,
            }
        };

        let statesync = {
            let (enable, state_sync) = if let Some(state_sync) = self.state_sync {
                (true, state_sync)
            } else {
                (false, Default::default())
            };

            let trust_period = format!("{}s", state_sync.trust_period.whole_seconds());

            let discovery_time = format!("{}s", state_sync.discovery_time.whole_seconds());

            let chunk_request_timeout =
                format!("{}s", state_sync.chunk_request_timeout.whole_seconds());

            model::StateSync {
                enable,
                rpc_servers: state_sync.rpc_servers.join(","),
                trust_hash: state_sync.trust_hash,
                trust_height: state_sync.trust_height,
                trust_period,
                discovery_time,
                temp_dir: Default::default(),
                chunk_request_timeout,
                chunk_fetchers: format!("{}", state_sync.chunk_fetchers),
            }
        };

        let (fast_sync, fastsync) = {
            let fast_sync = self.fast_sync.is_some();

            let version = if let Some(v) = self.fast_sync {
                String::from(v.to_str())
            } else {
                String::from("v0")
            };

            (fast_sync, model::FastSync { version })
        };

        let consensus = {
            let wal_file = format!("{}/{}", base_dir, defined::WAL_FILE);

            model::Consensus {
                wal_file,
                timeout_propose: utils::build_duration_ms(self.consensus.timeout_propose),
                timeout_propose_delta: utils::build_duration_ms(
                    self.consensus.timeout_propose_delta,
                ),
                timeout_prevote: utils::build_duration_ms(self.consensus.timeout_prevote),
                timeout_prevote_delta: utils::build_duration_ms(
                    self.consensus.timeout_prevote_delta,
                ),
                timeout_precommit: utils::build_duration_ms(self.consensus.timeout_precommit),
                timeout_precommit_delta: utils::build_duration_ms(
                    self.consensus.timeout_precommit_delta,
                ),
                timeout_commit: utils::build_duration_ms(self.consensus.timeout_commit),
                double_sign_check_height: self.consensus.double_sign_check_height,
                skip_timeout_commit: self.consensus.skip_timeout_commit,
                create_empty_blocks: self.consensus.create_empty_blocks,
                create_empty_blocks_interval: utils::build_duration_ms(
                    self.consensus.create_empty_blocks_interval,
                ),
                peer_gossip_sleep_duration: utils::build_duration_ms(
                    self.consensus.peer_gossip_sleep_duration,
                ),
                peer_query_maj23_sleep_duration: utils::build_duration_ms(
                    self.consensus.peer_query_maj23_sleep_duration,
                ),
                discard_abci_responses: self.consensus.discard_abci_responses,
            }
        };

        let tx_index = {
            let indexer = self.tx_index.to_str();

            let pgsql_conn = if let TxIndexConfig::Psql(c) = self.tx_index {
                c
            } else {
                Default::default()
            };

            model::TxIndex {
                indexer: String::from(indexer),
                pgsql_conn,
            }
        };

        let prometheus = {
            let (enable, prometheus) = if let Some(p) = self.prometheus {
                (true, p)
            } else {
                (false, Default::default())
            };

            model::Instrumentation {
                prometheus: enable,
                prometheus_listen_addr: prometheus.prometheus_listen_addr,
                max_open_connections: prometheus.max_open_connections,
                namespace: prometheus.namespace,
            }
        };

        let proxy_app = format!("{}/{}", base_dir, defined::APP_UNIX_SOCKET_FILE);
        log::debug!("proxy_app socket file is : {}", proxy_app);

        let db_dir = if self.data_dir.is_empty() {
            format!("{}/{}", base_dir, defined::DATA_DIR)
        } else {
            self.data_dir
        };

        let genesis_file = format!("{}/{}", base_dir, defined::GENESIS_FILE);

        let priv_validator_key_file = format!("{}/{}", base_dir, defined::VALIDATOR_KEY_FILE);

        let priv_validator_state_file = format!("{}/{}", base_dir, defined::VALIDATOR_STATE_FILE);

        let node_key_file = format!("{}/{}", base_dir, defined::NODE_KEY_FILE);

        model::Config {
            proxy_app,
            moniker: self.moniker,
            fast_sync,
            db_backend: String::from(self.db_backend.to_str()),
            db_dir,
            log_level: String::from(self.log_level.to_str()),
            log_format: String::from(self.log_format.to_str()),
            genesis_file,
            priv_validator_key_file,
            priv_validator_state_file,
            priv_validator_laddr: self.priv_validator_laddr,
            node_key_file,
            abci: String::from("socket"),
            filter_peers: self.filter_peers,
            rpc,
            p2p,
            mempool,
            statesync,
            fastsync,
            consensus,
            tx_index,
            instrumentation: prometheus,
        }
    }
}

mod utils {
    use time::Duration;

    /*     pub fn build_duration_s(d: Duration) -> String { */
    /*     format!("{}s", d.whole_seconds()) */
    /* } */
    /*  */
    pub fn build_duration_ms(d: Duration) -> String {
        format!("{}s", d.whole_milliseconds())
    }
}
