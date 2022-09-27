use time::Duration;

use crate::define_build_mode_setter;

#[derive(Debug, Default, Clone)]
pub struct P2P {
    /// Address to listen for incoming connections.
    pub laddr: String,

    /// Address to advertise to peers for them to dial
    /// If empty, will use the same port as the laddr,
    /// and will introspect on the listener or use UPnP
    /// to figure out the address. ip and port are required
    /// example: 159.89.10.97:26656
    pub external_address: String,

    /// List of seeds
    pub seeds: Vec<String>,

    /// List of persistent peers.
    pub persistent_peers: Vec<String>,

    /// UPNP port forwarding
    pub upnp: bool,

    /// Private or local net.
    pub local_net: bool,

    /// Maximum number of inbound peers
    pub max_num_inbound_peers: u64,

    /// Maximum number of outbound peers to connect to, excluding persistent peers
    pub max_num_outbound_peers: u64,

    /// List of node IDs, to which a connection will be (re)established ignoring any existing limits
    pub unconditional_peer_ids: Vec<String>,

    /// Maximum pause when redialing a persistent peer (if zero, exponential backoff is used)
    pub persistent_peers_max_dial_period: Duration,

    /// Time to wait before flushing messages out on the connection
    pub flush_throttle_timeout: Duration,

    /// Maximum size of a message packet payload, in bytes
    pub max_packet_msg_payload_size: u64,

    /// Rate at which packets can be sent, in bytes/second
    pub send_rate: u64,

    /// Rate at which packets can be received, in bytes/second
    pub recv_rate: u64,

    /// Set true to enable the peer-exchange reactor
    pub pex: bool,

    /// Seed mode, in which node constantly crawls the network and looks for
    /// peers. If another node asks it for addresses, it responds and disconnects.
    ///
    /// Does not work if the peer-exchange reactor is disabled.
    pub seed_mode: bool,

    /// Comma separated list of peer IDs to keep private (will not be gossiped to other peers)
    pub private_peer_ids: Vec<String>,

    /// Toggle to disable guard against peers connecting from the same ip.
    pub allow_duplicate_ip: bool,

    /// Peer connection configuration.
    pub handshake_timeout: Duration,
    pub dial_timeout: Duration,
}

impl P2P {
    define_build_mode_setter!(laddr, str);

    define_build_mode_setter!(external_address, str);
}

