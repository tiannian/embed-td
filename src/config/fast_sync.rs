use crate::define_to_str_for_enum;

#[derive(Debug, Default, Clone)]
pub enum FastSyncVersion {
    #[default]
    V0,
    V1,
    V2,
}

define_to_str_for_enum!(
    FastSyncVersion,
    V0 => "v0",
    V1 => "v1",
    V2 => "v2"
);
