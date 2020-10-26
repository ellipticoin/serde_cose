use crate::{
    constants::{Algorithm, ContentType, HeaderParameter},
    map::Map,
    sig::Sig,
};
use ed25519_zebra::SigningKey;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_bytes::ByteBuf;
use serde_cbor::{tags::Tagged, value::to_value};
use std::{collections, convert::TryFrom};

#[derive(Debug, Clone, PartialEq)]
pub struct Sign1 {
    pub protected: Map<HeaderParameter, i32>,
    pub unprotected: Map<HeaderParameter, Vec<u8>>,
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
}

impl Default for Sign1 {
    fn default() -> Self {
        let protected = [
            (HeaderParameter::Algorithm, Algorithm::EdDSA as i32),
            (HeaderParameter::ContentType, ContentType::Text as i32),
        ]
        .iter()
        .cloned()
        .collect::<Map<HeaderParameter, i32>>();
        let unprotected = Map::new();
        Self {
            protected,
            unprotected,
            payload: vec![],
            signature: vec![],
        }
    }
}

impl Sign1 {
    pub fn new<S: Serialize>(payload: S, kid: Vec<u8>) -> Self {
        let mut unprotected = Map::new();
        unprotected.insert(HeaderParameter::KID, kid);

        Self {
            payload: serde_cbor::to_vec(&payload).unwrap(),
            unprotected,
            ..Default::default()
        }
    }

    pub fn sign(&mut self, signing_key: SigningKey) {
        let signature = signing_key.sign(&serde_cbor::to_vec(&Sig::from(self.clone())).unwrap());
        self.signature = <[u8; 64]>::from(signature).to_vec();
    }
}
const TAG_ID: u64 = 18;

impl<'de> Deserialize<'de> for Sign1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let tagged = Tagged::<Vec<serde_cbor::Value>>::deserialize(deserializer)?;
        let protected_bytes = serde_cbor::value::from_value::<ByteBuf>(tagged.value[0].clone())
            .map_err(serde::de::Error::custom)?
            .to_vec();
        let protected =
            Map(serde_cbor::from_slice(&protected_bytes).map_err(serde::de::Error::custom)?);
        let unprotected: Map<HeaderParameter, Vec<u8>> = Map(serde_cbor::value::from_value::<
            collections::BTreeMap<HeaderParameter, serde_bytes::ByteBuf>,
        >(tagged.value[1].clone())
        .map_err(serde::de::Error::custom)?
        .iter()
        .map(|(k, v)| (k.clone(), v.to_vec()))
        .collect::<collections::BTreeMap<HeaderParameter, Vec<u8>>>());
        let payload =
            serde_cbor::value::from_value::<serde_bytes::ByteBuf>(tagged.value[2].clone())
                .map_err(serde::de::Error::custom)?
                .to_vec();
        let signature =
            serde_cbor::value::from_value::<serde_bytes::ByteBuf>(tagged.value[3].clone())
                .map_err(serde::de::Error::custom)?
                .to_vec();
        match tagged.tag {
            Some(TAG_ID) | None => Ok(Sign1 {
                payload,
                signature,
                protected,
                unprotected,
            }),
            Some(_) => Err(serde::de::Error::custom("unexpected tag")),
        }
    }
}

impl Serialize for Sign1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Tagged::new(
            Some(TAG_ID),
            vec![
                to_value(&self.protected).unwrap(),
                to_value(&self.unprotected.0).unwrap(),
                to_value(ByteBuf::from(self.payload.clone())).unwrap(),
                to_value(ByteBuf::from(self.signature.clone())).unwrap(),
            ],
        )
        .serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key::Key;
    use ed25519_zebra::{SigningKey, VerificationKey};
    use rand::thread_rng;

    #[test]
    fn signs_a_payload() {
        let signing_key = SigningKey::new(thread_rng());
        let verification_key = VerificationKey::from(&signing_key);
        let mut sign1 = Sign1::new(
            b"test".to_vec(),
            <[u8; 32]>::try_from(verification_key).unwrap().to_vec(),
        );
        sign1.sign(signing_key);
        let key: Key = verification_key.into();
        key.verify(&mut sign1);
    }

    #[test]
    fn serializes_and_deseriaizes() {
        let sign1: Sign1 = Default::default();
        assert!(sign1 == serde_cbor::from_slice(&serde_cbor::to_vec(&sign1).unwrap()).unwrap())
    }
}
