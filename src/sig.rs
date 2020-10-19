use crate::{constants::HeaderParameter, map::Map, serde::ser::SerializeSeq, sign1::Sign1};
use serde::{Serialize, Serializer};
use serde_bytes::ByteBuf;

pub struct Sig {
    pub body_attrs: Map<HeaderParameter, i32>,
    pub signer_attrs: Map<HeaderParameter, Vec<u8>>,
    pub application_attrs: Map<HeaderParameter, Vec<u8>>,
    pub payload: Vec<u8>,
}
impl From<Sign1> for Sig {
    fn from(sign1: Sign1) -> Self {
        Self {
            body_attrs: sign1.protected.clone(),
            signer_attrs: Map::new(),
            application_attrs: Map::new(),
            payload: sign1.payload.to_vec(),
        }
    }
}

impl Serialize for Sig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(4))?;
        seq.serialize_element("Signature1")?;
        seq.serialize_element(&self.body_attrs.clone())?;
        seq.serialize_element(&self.signer_attrs.clone())?;
        seq.serialize_element(&ByteBuf::from(self.payload.clone()))?;
        seq.end()
    }
}
