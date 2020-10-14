use crate::{helpers::slice_to_array_64, sig::Sig, sign1::Sign1};
use std::convert::TryFrom;

#[derive(Debug)]
pub enum Curve {
    ED25519,
}

impl Default for Curve {
    fn default() -> Self {
        Curve::ED25519
    }
}

#[derive(Debug)]
pub enum KeyType {
    OKP,
}

impl Default for KeyType {
    fn default() -> Self {
        KeyType::OKP
    }
}

#[derive(Default, Debug)]
pub struct Key {
    pub kty: KeyType,
    pub kid: Vec<u8>,
    pub crv: Curve,
    pub d: Option<Vec<u8>>,
    pub x: Option<Vec<u8>>,
    pub y: Option<Vec<u8>>,
}

impl From<ed25519_zebra::SigningKey> for Key {
    fn from(public_key: ed25519_zebra::SigningKey) -> Self {
        Key {
            crv: Curve::ED25519,
            kty: KeyType::OKP,
            d: Some(<[u8; 32]>::from(public_key).to_vec()),
            ..Default::default()
        }
    }
}

impl From<ed25519_zebra::VerificationKey> for Key {
    fn from(private_key: ed25519_zebra::VerificationKey) -> Self {
        Key {
            crv: Curve::ED25519,
            kty: KeyType::OKP,
            x: Some(<[u8; 32]>::from(private_key).to_vec()),
            ..Default::default()
        }
    }
}

impl Key {
    pub fn verify(&self, sign1: &Sign1) -> bool {
        let message = serde_cbor::to_vec(&Sig::from(sign1.clone())).unwrap();
        match &self.crv {
            Curve::ED25519 => {
                let verification_key = ed25519_zebra::VerificationKey::try_from(
                    <[u8; 32]>::try_from(&self.x.as_ref().unwrap().to_vec()[..]).unwrap(),
                )
                .unwrap();
                let signature = ed25519_zebra::Signature::from(
                    slice_to_array_64(&sign1.signature.as_ref().unwrap())
                        .unwrap()
                        .clone(),
                );
                verification_key.verify(&signature, &message).is_ok()
            }
        }
    }
}
