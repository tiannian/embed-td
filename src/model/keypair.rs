use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    #[serde(rename = "type")]
    pub ty: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keypair {
    pub address: String,
    pub priv_key: Key,
    pub pub_key: Key,
}
