#[cfg(unix)]
pub const TENDERMINT_BIN_FILE: &str = "tendermint";
#[cfg(windows)]
pub const TENDERMINT_BIN_FILE: &str = "tendermint.exe";

pub const CONFIG_DIR: &str = "config";
pub const CONFIG_FILE: &str = "config/config.toml";
pub const NODE_KEY_FILE: &str = "config/node_key.json";
pub const VALIDATOR_KEY_FILE: &str = "config/priv_validator_key.json";

pub const DATA_DIR: &str = "data";
pub const GENESIS_FILE: &str = "genesis.json";
pub const VALIDATOR_STATE_FILE: &str = "priv_validator_state.json";
pub const WAL_FILE: &str = "cs.wal";

pub const SOCKET_DIR: &str = "sockets";
pub const RPC_UNIX_SOCKET_FILE: &str = "sockets/rpc";
pub const APP_UNIX_SOCKET_FILE: &str = "sockets/app";

pub const P2P_DIR: &str = "p2p";
pub const ADDR_BOOK_FILE: &str = "p2p/addrbook.json";
