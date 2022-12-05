use rand_core::{CryptoRng, RngCore};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

use crate::model;

use super::{ed25519, secp256k1, sr25519};

/// Secret key for tendermint
#[derive(Debug, Clone)]
pub enum SecretKey {
    Ed25519(ed25519::SecretKey),
    Secp256k1(secp256k1::SecretKey),
    Sr25519(sr25519::SecretKey),
}

impl SecretKey {
    pub(crate) fn into_model(self) -> model::Key {
        let (ty, value) = match self {
            Self::Ed25519(k) => ("tendermint/PrivKeyEd25519", base64::encode(&k)),
            Self::Secp256k1(k) => ("tendermint/PrivKeySecp256k1", base64::encode(&k)),
            Self::Sr25519(k) => ("tendermint/PrivKeySr25519", base64::encode(&k)),
        };

        model::Key {
            ty: String::from(ty),
            value,
        }
    }

    pub fn generate(ty: AlgorithmType, rng: impl RngCore + CryptoRng) -> Self {
        match ty {
            AlgorithmType::Ed25519 => Self::Ed25519(ed25519::SecretKey::generate(rng)),
            AlgorithmType::Secp256k1 => Self::Secp256k1(secp256k1::SecretKey::generate(rng)),
            AlgorithmType::Sr25519 => Self::Sr25519(sr25519::SecretKey::generate(rng)),
        }
    }

    pub fn public_key(&self) -> PublicKey {
        match self {
            Self::Ed25519(k) => PublicKey::Ed25519(k.public_key()),
            Self::Secp256k1(k) => PublicKey::Secp256k1(k.public_key()),
            Self::Sr25519(k) => PublicKey::Sr25519(k.public_key()),
        }
    }
}

/// Public key for tendermint
#[derive(Debug, Clone)]
pub enum PublicKey {
    Ed25519(ed25519::PublicKey),
    Secp256k1(secp256k1::PublicKey),
    Sr25519(sr25519::PublicKey),
}

impl PublicKey {
    pub(crate) fn into_model(self) -> model::Key {
        let (ty, value) = match self {
            Self::Ed25519(k) => ("tendermint/PubKeyEd25519", base64::encode(&k)),
            Self::Secp256k1(k) => ("tendermint/PubKeySecp256k1", base64::encode(&k)),
            Self::Sr25519(k) => ("tendermint/PubKeySr25519", base64::encode(&k)),
        };

        model::Key {
            ty: String::from(ty),
            value,
        }
    }

    pub fn address(&self) -> [u8; 20] {
        let mut addr = [0u8; 20];

        match self {
            Self::Secp256k1(k) => {
                let step1_sha = Sha256::digest(k);

                let res = Ripemd160::digest(step1_sha);

                addr.copy_from_slice(&res);
            }
            Self::Ed25519(k) => {
                let res = Sha256::digest(k);

                addr.copy_from_slice(&res[..20]);
            }
            Self::Sr25519(k) => {
                let res = Sha256::digest(k);

                addr.copy_from_slice(&res[..20]);
            }
        }

        addr
    }
}

/// Keypair of tendermint
#[derive(Debug, Clone)]
pub struct Keypair {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
}

pub enum AlgorithmType {
    Secp256k1,
    Ed25519,
    Sr25519,
}

impl Keypair {
    pub(crate) fn into_model(self) -> model::Keypair {
        let address = self.public_key.address();
        let pub_key = self.public_key.into_model();

        let priv_key = self.secret_key.into_model();

        model::Keypair {
            address: hex::encode(address),
            priv_key,
            pub_key,
        }
    }
    pub fn generate(ty: AlgorithmType, rng: impl RngCore + CryptoRng) -> Self {
        let secret_key = SecretKey::generate(ty, rng);
        let public_key = secret_key.public_key();

        Self {
            secret_key,
            public_key,
        }
    }
}
