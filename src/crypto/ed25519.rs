use curve25519_dalek_ng::{constants, scalar::Scalar};
use rand_core::{CryptoRng, RngCore};
use sha2::{Digest, Sha512};

use crate::define_as_ref_u8_array;

#[derive(Debug, Clone)]
pub struct SecretKey(pub [u8; 64]);
define_as_ref_u8_array!(SecretKey);

#[derive(Debug, Clone)]
pub struct PublicKey(pub [u8; 32]);
define_as_ref_u8_array!(PublicKey);

impl SecretKey {
    pub fn generate(rng: impl RngCore + CryptoRng) -> Self {
        let mut rng = rng;

        let mut sk = [0u8; 64];

        rng.fill_bytes(&mut sk[..32]);

        // sk to pk

        let mut h = Sha512::new();
        let mut hash: [u8; 64] = [0u8; 64];
        let mut digest: [u8; 32] = [0u8; 32];

        h.update(&sk[..32]);
        hash.copy_from_slice(&h.finalize());

        digest.copy_from_slice(&hash[..32]);

        // mangle_scalar_bits_and_multiply_by_basepoint_to_produce_public_key
        {
            let mut bits = digest;

            bits[0] &= 248;
            bits[31] &= 127;
            bits[31] |= 64;

            let point = &Scalar::from_bits(bits) * &constants::ED25519_BASEPOINT_TABLE;
            let compressed = point.compress();

            let pk = &mut sk[32..];

            pk.copy_from_slice(&compressed.0);
        }

        Self(sk)
    }

    pub fn public_key(&self) -> PublicKey {
        let mut pk = [0u8; 32];

        pk.copy_from_slice(&self.0[32..]);
        PublicKey(pk)
    }
}
