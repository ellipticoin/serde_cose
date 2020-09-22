use crate::COSE;
use serde::{Deserialize, Deserializer};
use serde_cbor::tags::Tagged;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Sign1 {
    pub protected: Vec<u8>,
    pub unprotected: HashMap<i32, Vec<u8>>,
    pub message: Vec<u8>,
    pub signature: Vec<u8>,
}
const TAG_ID: u64 = 18;

impl<'de> Deserialize<'de> for COSE {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let tagged = Tagged::<Vec<serde_cbor::Value>>::deserialize(deserializer)?;
        let protected =
            serde_cbor::value::from_value::<serde_bytes::ByteBuf>(tagged.value[0].clone())
                .unwrap()
                .to_vec();
        let unprotected: HashMap<i32, Vec<u8>> = serde_cbor::value::from_value::<
            HashMap<i32, serde_bytes::ByteBuf>,
        >(tagged.value[1].clone())
        .unwrap()
        .iter()
        .map(|(k, v)| (*k, v.to_vec()))
        .collect();
        let message =
            serde_cbor::value::from_value::<serde_bytes::ByteBuf>(tagged.value[2].clone())
                .unwrap()
                .to_vec();
        let signature =
            serde_cbor::value::from_value::<serde_bytes::ByteBuf>(tagged.value[3].clone())
                .unwrap()
                .to_vec();
        match tagged.tag {
            Some(TAG_ID) | None => Ok(COSE::Sign1(Sign1 {
                message,
                signature,
                protected,
                unprotected,
            })),
            Some(_) => Err(serde::de::Error::custom("unexpected tag")),
        }
    }
}
