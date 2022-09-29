#[derive(Debug, Clone, Default)]
pub enum TxIndexConfig {
    Null,
    #[default]
    Kv,
    Psql(String),
}

impl TxIndexConfig {
    pub fn to_str(&self) -> &'static str {
        match self {
            TxIndexConfig::Null => "null",
            TxIndexConfig::Kv => "kv",
            TxIndexConfig::Psql(_) => "psql",
        }
    }
}
