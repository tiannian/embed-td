use super::define_to_str_for_enum;

#[derive(Debug, Default, Clone)]
pub enum DbBackend {
    #[default]
    GoLevelDB,
    CLevelDB,
    BoltDB,
    RocksDB,
    BadgerDB,
}

define_to_str_for_enum!(
    DbBackend,
    GoLevelDB => "goleveldb",
    CLevelDB => "cleveldb",
    BoltDB => "boltdb",
    RocksDB => "rocksdb",
    BadgerDB => "badgerdb"
);
