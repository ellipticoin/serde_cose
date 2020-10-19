use serde::{
    de::{Deserialize, Deserializer, Visitor},
    Serialize, Serializer,
};
use serde_bytes::ByteBuf;
use std::{
    cmp::{Eq, Ord},
    collections, fmt,
    marker::PhantomData,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Map<K: Serialize, V: Serialize>(pub collections::BTreeMap<K, V>);

impl<K: Serialize + Ord, V: Serialize> Map<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }
}

impl<K: Serialize + Ord, V: Serialize> std::iter::FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: std::iter::IntoIterator<Item = (K, V)>,
    {
        Map(<collections::BTreeMap<K, V>>::from_iter(iter))
    }
}

impl<K: Serialize, V: Serialize> From<collections::BTreeMap<K, V>> for Map<K, V> {
    fn from(hash_map: collections::BTreeMap<K, V>) -> Self {
        Map(hash_map)
    }
}
impl<K: Serialize + Ord, V: Serialize> Map<K, V> {
    pub fn new() -> Self {
        Map(collections::BTreeMap::new())
    }
}

struct MapVisitor<K: Serialize, V: Serialize> {
    marker: PhantomData<fn() -> Map<K, V>>,
}

impl<K: Serialize, V: Serialize> MapVisitor<K, V> {
    fn new() -> Self {
        MapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, K: Serialize, V: Serialize> Visitor<'de> for MapVisitor<K, V>
where
    K: Deserialize<'de> + std::hash::Hash + Eq + Ord,
    V: Deserialize<'de>,
{
    type Value = Map<K, V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("COSE Map")
    }

    fn visit_borrowed_bytes<M: serde::de::Error>(self, bytes: &'de [u8]) -> Result<Map<K, V>, M> {
        let map = serde_cbor::from_slice(bytes).map_err(serde::de::Error::custom)?;
        Ok(Map(map))
    }
}

impl<'de, K: Serialize, V: Serialize> Deserialize<'de> for Map<K, V>
where
    K: Deserialize<'de> + std::hash::Hash + Eq + Ord,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(MapVisitor::new())
    }
}
impl<K, V> Serialize for Map<K, V>
where
    K: Serialize + std::hash::Hash + Eq + Ord,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0.len() == 0 {
            return serializer.serialize_bytes(&ByteBuf::new());
        } else {
            return serializer
                .serialize_bytes(&serde_cbor::to_vec(&self.0).map_err(serde::ser::Error::custom)?);
        }
    }
}
