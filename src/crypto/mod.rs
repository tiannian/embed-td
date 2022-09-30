pub mod ed25519;

pub mod sr25519;

pub mod secp256k1;

mod keypair;
pub use keypair::*;

#[macro_export]
macro_rules! define_as_ref_u8_array {
    ($t:ty) => {
        impl AsRef<[u8]> for $t {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
    };
}
