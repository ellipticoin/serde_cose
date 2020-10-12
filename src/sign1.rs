use crate::{constants::HeaderParameter, hash_map::HashMap};
use serde::{Deserialize, Deserializer};
use serde_bytes::ByteBuf;
use serde_cbor::tags::Tagged;
use std::collections;

#[derive(Debug)]
pub struct Sign1 {
    pub protected: HashMap<HeaderParameter, i32>,
    pub unprotected: HashMap<HeaderParameter, Vec<u8>>,
    pub message: Vec<u8>,
    pub signature: Vec<u8>,
}
const TAG_ID: u64 = 18;

impl<'de> Deserialize<'de> for Sign1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let tagged = Tagged::<Vec<serde_cbor::Value>>::deserialize(deserializer)?;
        let protected_bytes = serde_cbor::value::from_value::<ByteBuf>(tagged.value[0].clone())
            .unwrap()
            .to_vec();
        let protected = HashMap(serde_cbor::from_slice(&protected_bytes).unwrap());
        let unprotected: HashMap<HeaderParameter, Vec<u8>> = HashMap(
            serde_cbor::value::from_value::<
                collections::HashMap<HeaderParameter, serde_bytes::ByteBuf>,
            >(tagged.value[1].clone())
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.to_vec()))
            .collect::<collections::HashMap<HeaderParameter, Vec<u8>>>(),
        );
        let message =
            serde_cbor::value::from_value::<serde_bytes::ByteBuf>(tagged.value[2].clone())
                .unwrap()
                .to_vec();
        let signature =
            serde_cbor::value::from_value::<serde_bytes::ByteBuf>(tagged.value[3].clone())
                .unwrap()
                .to_vec();
        match tagged.tag {
            Some(TAG_ID) | None => Ok(Sign1 {
                message,
                signature,
                protected,
                unprotected,
            }),
            Some(_) => Err(serde::de::Error::custom("unexpected tag")),
        }
    }
}
