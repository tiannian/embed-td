#[derive(Debug, Clone, Default)]
pub enum TxIndexConfig {
    Null,
    #[default]
    Kv,
    Psql(String),
}
