use crate::{hash_map_encoder, helpers::slice_to_array_64, sign1::Sign1, COSE};
use ed25519_dalek::Verifier;
use std::collections::HashMap;

pub enum Curve {
    ED25519,
}

impl Default for Curve {
    fn default() -> Self {
        Curve::ED25519
    }
}

pub enum KeyType {
    OKP,
}

impl Default for KeyType {
    fn default() -> Self {
        KeyType::OKP
    }
}

#[derive(Default)]
pub struct Key {
    pub kty: KeyType,
    pub kid: Vec<u8>,
    pub crv: Curve,
    pub d: Option<Vec<u8>>,
    pub x: Option<Vec<u8>>,
    pub y: Option<Vec<u8>>,
}

impl From<ed25519_dalek::PublicKey> for Key {
    fn from(public_key: ed25519_dalek::PublicKey) -> Self {
        Key {
            crv: Curve::ED25519,
            kty: KeyType::OKP,
            x: Some(public_key.as_bytes().to_vec()),
            ..Default::default()
        }
    }
}

impl Key {
    pub fn verify(&self, cose: &COSE) -> bool {
        let (protected, _unprotected, message, signature) = match cose {
            COSE::Sign1(Sign1 {
                signature,
                message,
                protected,
                unprotected,
            }) => (protected, unprotected, message, signature),
        };
        let message: serde_cbor::Value = serde_cbor::Value::Array(vec![
            "Signature1".to_string().into(),
            protected.clone().into(),
            hash_map_encoder::encode(HashMap::new()).into(),
            message.clone().into(),
        ]);

        let message = serde_cbor::to_vec(&message).unwrap();

        match &self.crv {
            Curve::ED25519 => {
                let public_key =
                    ed25519_dalek::PublicKey::from_bytes(&self.x.as_ref().unwrap().clone())
                        .unwrap();
                let signature =
                    ed25519_dalek::Signature::new(slice_to_array_64(&signature).unwrap().clone());
                public_key.verify(&message, &signature).is_ok()
            }
        }
    }
}