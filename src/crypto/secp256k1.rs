use k256::ecdsa;
use rand_core::{CryptoRng, RngCore};

use crate::define_as_ref_u8_array;

#[derive(Debug, Clone)]
pub struct SecretKey(pub [u8; 32], ecdsa::SigningKey);
define_as_ref_u8_array!(SecretKey);

#[derive(Debug, Clone)]
pub struct PublicKey(pub [u8; 33], pub(crate) ecdsa::VerifyingKey);
define_as_ref_u8_array!(PublicKey);

impl SecretKey {
    pub fn generate(rng: impl RngCore + CryptoRng) -> Self {
        let secret_key = ecdsa::SigningKey::random(rng);
        let bytes = secret_key.to_bytes().into();

        Self(bytes, secret_key)
    }

    pub fn public_key(&self) -> PublicKey {
        let public_key = self.1.verifying_key();
        let bytes = public_key.to_bytes().into();

        PublicKey(bytes, public_key)
    }
}
