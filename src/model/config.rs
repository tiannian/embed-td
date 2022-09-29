use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub(crate) proxy_app: String,
    pub(crate) moniker: String,
    pub(crate) fast_sync: bool,
    pub(crate) db_backend: String,
    pub(crate) db_dir: String,
    pub(crate) log_level: String,
    pub(crate) log_format: String,
    pub(crate) genesis_file: String,
    pub(crate) priv_validator_key_file: String,
    pub(crate) priv_validator_state_file: String,
    pub(crate) priv_validator_laddr: String,
    pub(crate) node_key_file: String,
    pub(crate) abci: String,
    pub(crate) filter_peers: bool,
    pub(crate) rpc: Rpc,
    pub(crate) p2p: P2P,
    pub(crate) mempool: Mempool,
    pub(crate) statesync: StateSync,
    pub(crate) fastsync: FastSync,
    pub(crate) consensus: Consensus,
    pub(crate) tx_index: TxIndex,
    pub(crate) instrumentation: Instrumentation,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Rpc {
    pub(crate) laddr: String,
    pub(crate) cors_allowed_origins: Vec<String>,
    pub(crate) cors_allowed_methods: Vec<String>,
    pub(crate) cors_allowed_headers: Vec<String>,
    pub(crate) grpc_laddr: String,
    pub(crate) grpc_max_open_connections: u64,
    #[serde(rename = "unsafe")]
    pub(crate) unsafe_opt: bool,
    pub(crate) max_open_connections: u64,
    pub(crate) max_subscription_clients: u64,
    pub(crate) max_subscriptions_per_client: u64,
    pub(crate) experimental_subscription_buffer_size: u64,
    pub(crate) experimental_websocket_write_buffer_size: u64,
    pub(crate) experimental_close_on_slow_client: bool,
    pub(crate) timeout_broadcast_tx_commit: String,
    pub(crate) max_body_bytes: u64,
    pub(crate) max_header_bytes: u64,
    pub(crate) tls_cert_file: String,
    pub(crate) tls_key_file: String,
    pub(crate) pprof_laddr: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct P2P {
    pub(crate) laddr: String,
    pub(crate) external_address: String,
    pub(crate) seeds: String,
    pub(crate) persistent_peers: String,
    pub(crate) upnp: bool,
    pub(crate) addr_book_file: String,
    pub(crate) addr_book_strict: bool,
    pub(crate) max_num_inbound_peers: u64,
    pub(crate) max_num_outbound_peers: u64,
    pub(crate) unconditional_peer_ids: String,
    pub(crate) persistent_peers_max_dial_period: String,
    pub(crate) flush_throttle_timeout: String,
    pub(crate) max_packet_msg_payload_size: u64,
    pub(crate) send_rate: u64,
    pub(crate) recv_rate: u64,
    pub(crate) pex: bool,
    pub(crate) seed_mode: bool,
    pub(crate) private_peer_ids: String,
    pub(crate) allow_duplicate_ip: bool,
    pub(crate) handshake_timeout: String,
    pub(crate) dial_timeout: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Mempool {
    pub(crate) version: String,
    pub(crate) recheck: bool,
    pub(crate) broadcast: bool,
    pub(crate) wal_dir: String,
    pub(crate) size: u64,
    pub(crate) max_txs_bytes: u64,
    pub(crate) cache_size: u64,
    #[serde(rename = "keep-invalid-txs-in-cache")]
    pub(crate) keep_invalid_txs_in_cache: bool,
    pub(crate) max_tx_bytes: u64,
    pub(crate) max_batch_bytes: u64,
    #[serde(rename = "ttl-duration")]
    pub(crate) ttl_duration: String,
    #[serde(rename = "ttl-num-blocks")]
    pub(crate) ttl_num_blocks: u64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StateSync {
    pub(crate) enable: bool,
    pub(crate) rpc_servers: String,
    pub(crate) trust_height: u64,
    pub(crate) trust_hash: String,
    pub(crate) trust_period: String,
    pub(crate) discovery_time: String,
    pub(crate) temp_dir: String,
    pub(crate) chunk_request_timeout: String,
    pub(crate) chunk_fetchers: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FastSync {
    pub(crate) version: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Consensus {
    pub(crate) wal_file: String,
    pub(crate) timeout_propose: String,
    pub(crate) timeout_propose_delta: String,
    pub(crate) timeout_prevote: String,
    pub(crate) timeout_prevote_delta: String,
    pub(crate) timeout_precommit: String,
    pub(crate) timeout_precommit_delta: String,
    pub(crate) timeout_commit: String,
    pub(crate) double_sign_check_height: u64,
    pub(crate) skip_timeout_commit: bool,
    pub(crate) create_empty_blocks: bool,
    pub(crate) create_empty_blocks_interval: String,
    pub(crate) peer_gossip_sleep_duration: String,
    pub(crate) peer_query_maj23_sleep_duration: String,
    pub(crate) discard_abci_responses: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TxIndex {
    pub(crate) indexer: String,
    #[serde(rename = "psql-conn")]
    pub(crate) pgsql_conn: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Instrumentation {
    pub(crate) prometheus: bool,
    pub(crate) prometheus_listen_addr: String,
    pub(crate) max_open_connections: u64,
    pub(crate) namespace: String,
}
