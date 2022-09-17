use crate::model;
use k256::{
    ecdsa,
    sha2::{Digest, Sha256},
};
use rand_core::{CryptoRng, RngCore};
use ripemd::Ripemd160;

enum SecretKey<'a> {
    Secp256k1(&'a k256::ecdsa::SigningKey),
}

impl<'a> SecretKey<'a> {
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

enum PublicKey<'a> {
    Secp256k1(&'a k256::ecdsa::VerifyingKey),
}

impl<'a> PublicKey<'a> {
    pub fn to_key(&self) -> model::Key {
        match self {
            Self::Secp256k1(k) => {
                let ty = String::from("tendermint/PubKeySecp256k1");
                let value = base64::encode(k.to_bytes());

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
        }
    }
}

pub struct SPKey<S, P> {
    secret_key: S,
    public_key: P,
}

pub enum KeypairType {
    Secp256k1,
}

pub enum Keypair {
    Secp256k1(SPKey<ecdsa::SigningKey, ecdsa::VerifyingKey>),
}

impl Keypair {
    pub fn to_serde(&self) -> model::Keypair {
        match self {
            Self::Secp256k1(sp) => {
                let s = &sp.secret_key;
                let p = &sp.public_key;

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
        }
    }

    pub fn generate(ty: KeypairType, rng: impl RngCore + CryptoRng) -> Self {
        match ty {
            KeypairType::Secp256k1 => {
                let secret_key = ecdsa::SigningKey::random(rng);
                let public_key = secret_key.verifying_key();

                let sp = SPKey {
                    secret_key,
                    public_key,
                };

                Self::Secp256k1(sp)
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
}
