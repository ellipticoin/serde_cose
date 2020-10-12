use std::{fmt, marker::PhantomData};

use serde::{
    de::{Deserialize, Deserializer, Visitor},
    Serialize, Serializer,
};
use serde_bytes::ByteBuf;
use std::collections;

#[derive(Clone, Debug)]
pub struct HashMap<K: Serialize, V: serde::Serialize>(pub collections::HashMap<K, V>);
impl<K: serde::Serialize, V: serde::Serialize> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap(collections::HashMap::new())
    }
}
struct HashMapVisitor<K: Serialize, V: Serialize> {
    marker: PhantomData<fn() -> HashMap<K, V>>,
}

impl<K: Serialize, V: Serialize> HashMapVisitor<K, V> {
    fn new() -> Self {
        HashMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, K: Serialize, V: Serialize> Visitor<'de> for HashMapVisitor<K, V>
where
    K: Deserialize<'de> + std::hash::Hash + std::cmp::Eq,
    V: Deserialize<'de>,
{
    type Value = HashMap<K, V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("COSE HashMap")
    }

    fn visit_borrowed_bytes<M>(self, bytes: &'de [u8]) -> Result<HashMap<K, V>, M> {
        let map = serde_cbor::from_slice(bytes).unwrap();
        Ok(HashMap(map))
    }
}

impl<'de, K: Serialize, V: Serialize> Deserialize<'de> for HashMap<K, V>
where
    K: Deserialize<'de> + std::hash::Hash + std::cmp::Eq,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(HashMapVisitor::new())
    }
}
impl<K, V> Serialize for HashMap<K, V>
where
    K: Serialize + std::hash::Hash + std::cmp::Eq,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0.len() == 0 {
            return serializer.serialize_bytes(&ByteBuf::new());
        } else {
            return serializer.serialize_bytes(&serde_cbor::to_vec(&self.0).unwrap());
        }
    }
}
