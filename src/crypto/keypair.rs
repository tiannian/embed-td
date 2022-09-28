use crate::model;
use k256::ecdsa;
use rand_core::{CryptoRng, RngCore};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

use super::{ed25519, sr25519};

enum SecretKey<'a> {
    Secp256k1(&'a k256::ecdsa::SigningKey),
    Ed25519(&'a [u8; 64]),
    Sr25519(&'a [u8; 32]),
}

impl<'a> SecretKey<'a> {
    pub fn to_key(&self) -> model::Key {
        match self {
            Self::Secp256k1(k) => {
                let ty = String::from("tendermint/PrivKeySecp256k1");
                let value = base64::encode(k.to_bytes());

                model::Key { ty, value }
            }
            Self::Ed25519(k) => {
                let ty = String::from("tendermint/PrivKeyEd25519");
                let value = base64::encode(k);

                model::Key { ty, value }
            }
            Self::Sr25519(k) => {
                let ty = String::from("tendermint/PrivKeySr25519");
                let value = base64::encode(k);

                model::Key { ty, value }
            }
        }
    }
}

enum PublicKey<'a> {
    Secp256k1(&'a k256::ecdsa::VerifyingKey),
    Ed25519(&'a [u8; 32]),
    Sr25519(&'a [u8; 32]),
}

impl<'a> PublicKey<'a> {
    pub fn to_key(&self) -> model::Key {
        match self {
            Self::Secp256k1(k) => {
                let ty = String::from("tendermint/PubKeySecp256k1");
                let value = base64::encode(k.to_bytes());

                model::Key { ty, value }
            }
            Self::Ed25519(k) => {
                let ty = String::from("tendermint/PubKeyEd25519");
                let value = base64::encode(k);

                model::Key { ty, value }
            }
            Self::Sr25519(k) => {
                let ty = String::from("tendermint/PubKeySr25519");
                let value = base64::encode(k);

                model::Key { ty, value }
            }
        }
    }

    pub fn address(&self) -> String {
        match self {
            Self::Secp256k1(k) => {
                let bytes = k.to_bytes();

                let step1_sha = Sha256::digest(bytes);

                let res = Ripemd160::digest(step1_sha);

                hex::encode(res)
            }
            Self::Ed25519(k) => {
                let res = Sha256::digest(k);

                hex::encode(&res[..20])
            }
            Self::Sr25519(k) => {
                let res = Sha256::digest(k);

                hex::encode(&res[..20])
            }
        }
    }
}

pub enum KeypairType {
    Secp256k1,
    Ed25519,
    Sr25519,
}

pub enum Keypair {
    Secp256k1(ecdsa::SigningKey, ecdsa::VerifyingKey),
    Ed25519([u8; 64], [u8; 32]),
    Sr25519([u8; 32], [u8; 32]),
}

impl Keypair {
    pub(crate) fn to_serde(&self) -> model::Keypair {
        match self {
            Self::Secp256k1(s, p) => {
                let sk = SecretKey::Secp256k1(s);
                let pk = PublicKey::Secp256k1(p);

                let address = pk.address();
                let pub_key = pk.to_key();

                let priv_key = sk.to_key();

                model::Keypair {
                    address,
                    priv_key,
                    pub_key,
                }
            }
            Self::Ed25519(s, p) => {
                let sk = SecretKey::Ed25519(s);
                let pk = PublicKey::Ed25519(p);

                let address = pk.address();
                let pub_key = pk.to_key();

                let priv_key = sk.to_key();

                model::Keypair {
                    address,
                    priv_key,
                    pub_key,
                }
            }
            Self::Sr25519(s, p) => {
                let sk = SecretKey::Sr25519(s);
                let pk = PublicKey::Sr25519(p);

                let address = pk.address();
                let pub_key = pk.to_key();

                let priv_key = sk.to_key();

                model::Keypair {
                    address,
                    priv_key,
                    pub_key,
                }
            }
        }
    }

    pub fn generate(ty: KeypairType, rng: impl RngCore + CryptoRng) -> Self {
        match ty {
            KeypairType::Secp256k1 => {
                let secret_key = ecdsa::SigningKey::random(rng);
                let public_key = secret_key.verifying_key();

                Self::Secp256k1(secret_key, public_key)
            }
            KeypairType::Ed25519 => {
                let secret_key = ed25519::generate(rng);
                let public_key = ed25519::sk2pk(&secret_key);

                Self::Ed25519(secret_key, public_key)
            }
            KeypairType::Sr25519 => {
                let secret_key = sr25519::generate(rng);
                let public_key = sr25519::sk2pk(&secret_key);

                Self::Sr25519(secret_key, public_key)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Keypair, KeypairType};

    #[test]
    fn generate_secp256k1() {
        let rng = rand::thread_rng();

        let keypair = Keypair::generate(KeypairType::Secp256k1, rng);

        let kp_serde = keypair.to_serde();

        let s = serde_json::to_string_pretty(&kp_serde).unwrap();

        println!("{}", s);
    }

    #[test]
    fn generate_ed25519() {
        let rng = rand::thread_rng();

        let keypair = Keypair::generate(KeypairType::Ed25519, rng);

        let kp_serde = keypair.to_serde();

        let s = serde_json::to_string_pretty(&kp_serde).unwrap();

        println!("{}", s);
    }
}
