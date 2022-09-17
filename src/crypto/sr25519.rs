use curve25519_dalek_ng::{constants, scalar::Scalar};
use rand_core::{CryptoRng, RngCore};
use sha2::{digest::FixedOutput, Digest, Sha512};

pub fn generate(rng: impl RngCore + CryptoRng) -> [u8; 32] {
    let mut rng = rng;

    let mut sk = [0u8; 32];

    rng.fill_bytes(&mut sk);

    sk
}

pub fn sk2pk(sk: &[u8; 32]) -> [u8; 32] {
    let mut h = Sha512::default();
    h.update(sk);
    let r = h.finalize_fixed();

    let mut key = [0u8; 32];
    key.copy_from_slice(&r.as_slice()[0..32]);
    key[0] &= 248;
    key[31] &= 63;
    key[31] |= 64;

    divide_scalar_bytes_by_cofactor(&mut key);
    let key = Scalar::from_bits(key);

    let point = &key * &constants::RISTRETTO_BASEPOINT_TABLE;

    let compressed = point.compress();

    compressed.to_bytes()
}

pub fn divide_scalar_bytes_by_cofactor(scalar: &mut [u8; 32]) {
    let mut low = 0u8;
    for i in scalar.iter_mut().rev() {
        let r = *i & 0b00000111; // save remainder
        *i >>= 3; // divide by 8
        *i += low;
        low = r << 5;
    }
}
