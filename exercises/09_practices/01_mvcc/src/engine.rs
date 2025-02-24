use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

//K: encoded Key
pub type Engine = BTreeMap<Vec<u8>, Option<Vec<u8>>>;

#[derive(Deserialize, Serialize)]
pub struct Key {
    pub raw_key: Vec<u8>,
    pub version: u64,
}

impl Key {
    pub fn encode(key: Key) -> Vec<u8> {
        bincode::serialize(&key).unwrap()
    }

    pub fn decode(v: &Vec<u8>) -> Key {
        bincode::deserialize(&v).unwrap()
    }
}
