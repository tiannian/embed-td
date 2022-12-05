use super::define_build_mode_setter;

#[derive(Debug, Clone)]
pub struct PrometheusConfig {
    /// Address to listen for Prometheus collector(s) connections
    pub prometheus_listen_addr: String,

    /// Maximum number of simultaneous connections.
    /// If you want to accept a larger number than the default, make sure
    /// you increase your OS limits.
    /// 0 - unlimited.
    pub max_open_connections: u64,

    /// Instrumentation namespace
    pub namespace: String,
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            prometheus_listen_addr: String::from(":26660"),
            max_open_connections: 3,
            namespace: String::from("tendermint"),
        }
    }
}

impl PrometheusConfig {
    define_build_mode_setter!(prometheus_listen_addr, str);

    define_build_mode_setter!(max_open_connections, u64);

    define_build_mode_setter!(namespace, str);
}
