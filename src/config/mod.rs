mod log_level;
pub use log_level::*;

mod fast_sync;
pub use fast_sync::*;

mod db_backend;
pub use db_backend::*;

mod p2p;
pub use p2p::*;

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
}
