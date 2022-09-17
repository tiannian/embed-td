use crate::model;
use k256::ecdsa;

enum SecretKey {
    Secp256k1(k256::ecdsa::VerifyingKey),
}

impl SecretKey {
    pub fn to_key(&self) -> model::Key {
        match self {
            Self::Secp256k1(k) => {
                let ty = String::from("tendermint/PrivKeySecp256k1");
                let value = base64::encode(k.to_bytes());

                model::Key { ty, value }
            }
        }
    }
}

enum PublicKey {
    Secp256k1(k256::ecdsa::SigningKey),
}

impl PublicKey {
    pub fn to_key(&self) -> model::Key {
        match self {
            Self::Secp256k1(k) => {
                let ty = String::from("tendermint/PubKeySecp256k1");
                let value = base64::encode(k.to_bytes());

                model::Key { ty, value }
            }
        }
    }
}

pub struct SPKey<S, P> {
    secret_key: S,
    public_key: P,
}

pub enum Keypair {
    Secp256k1(SPKey<ecdsa::SigningKey, ecdsa::VerifyingKey>),
}

impl Keypair {}
