use std::collections::HashMap;
pub fn encode(hashmap: HashMap<i32, Vec<u8>>) -> Vec<u8> {
    if hashmap.len() == 0 {
        vec![]
    } else {
        serde_cbor::to_vec(&hashmap).unwrap()
    }
}
