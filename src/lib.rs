#[cfg(test)]
extern crate rand;
extern crate serde;

mod constants;
mod helpers;
mod key;
mod map;
pub mod sig;
pub mod sign1;
pub use constants::*;
pub use key::*;
pub use map::*;
pub use sign1::Sign1;

impl Sign1 {
    pub fn kid(&self) -> Vec<u8> {
        self.unprotected.0[&HeaderParameter::KID].to_vec()
    }
}

pub fn from_slice(bytes: &[u8]) -> Result<Sign1, serde_cbor::Error> {
    serde_cbor::from_slice(bytes)
}
