use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    proxy_app: String,
    moniker: String,
    fast_sync: bool,
    db_backend: String,
    db_dir: String,
    log_level: String,
    log_format: String,
    genesis_file: String,
    priv_validator_key_file: String,
    priv_validator_state_file: String,
    priv_validator_laddr: String,
    node_key_file: String,
    abci: String,
    filter_peers: bool,
    rpc: Rpc,
    p2p: P2P,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rpc {
    laddr: String,
    cors_allowed_origins: Vec<String>,
    cors_allowed_methods: Vec<String>,
    cors_allowed_headers: Vec<String>,
    grpc_laddr: String,
    grpc_max_open_connections: u64,
    #[serde(rename = "unsafe")]
    unsafe_opt: bool,
    max_open_connections: i64,
    max_subscription_clients: i64,
    max_subscriptions_per_client: i64,
    experimental_subscription_buffer_size: i64,
    experimental_websocket_write_buffer_size: i64,
    experimental_close_on_slow_client: bool,
    timeout_broadcast_tx_commit: String,
    max_body_bytes: i64,
    max_header_bytes: i64,
    tls_cert_file: String,
    tls_key_file: String,
    pprof_laddr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct P2P {
    laddr: String,
    external_address: String,
    seeds: String,
    persistent_peers: String,
    upnp: bool,
    addr_book_file: String,
    addr_book_strict: bool,
    max_num_inbound_peers: i64,
    max_num_outbound_peers: i64,
    unconditional_peer_ids: String,
    persistent_peers_max_dial_period: String,
    flush_throttle_timeout: String,
    max_packet_msg_payload_size: i64,
    send_rate: i64,
    recv_rate: i64,
    pex: bool,
    seed_mode: bool,
    private_peer_ids: String,
    allow_duplicate_ip: bool,
    handshake_timeout: String,
    dial_timeout: String,
}
