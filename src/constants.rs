use serde_repr::{Deserialize_repr, Serialize_repr};
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i32)]
pub enum Algorithm {
    EdDSA = -8,
}
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Eq, Hash)]
#[repr(i32)]
pub enum HeaderParameter {
    Algorithm = 1,
    ContentType = 3,
    KID = 4,
}
