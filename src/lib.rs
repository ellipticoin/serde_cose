extern crate serde;
mod hash_map_encoder;
mod helpers;
mod key;
mod sign1;
pub use key::*;
use sign1::Sign1;

#[derive(Debug)]
pub enum COSE {
    Sign1(Sign1),
}

impl COSE {
    pub fn kid(&self) -> Vec<u8> {
        match &self {
            COSE::Sign1(Sign1 { unprotected, .. }) => unprotected[&4].to_vec(),
        }
    }
}

pub fn from_slice(bytes: &[u8]) -> COSE {
    serde_cbor::from_slice(bytes).unwrap()
}
