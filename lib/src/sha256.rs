use crate::U256;
use serde::{Deserialize, Serialize};
use sha256::digest;
use std::fmt;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Hash(U256);

impl Hash {
    // hash anything that can be serde Serialized via ciborium
    pub fn hash<T: serde::Serialize>(data: &T) -> Self {
        let mut serialized: Vec<u8> = vec![];
        if let Err(e) = ciborium::into_writer(data, &mut serialized) {
            panic!("Failed to serialize data: {:?}.", e)
        };

        let hash = digest(&serialized);
        // hash is a string containing the hexadecimal representation of a hash.
        // we take this hex string and convert it into a vector.
        let hash_bytes = hex::decode(hash).unwrap();
        let hash_array: [u8; 32] = hash_bytes.as_slice().try_into().unwrap();
        Hash(U256::from(hash_array))
        /*
        check https://github.com/braiins/build-bitcoin-in-rust/issues/7 for latest uint vesrion bug.
        // Convert the 32-byte array into a U256 using from_big_endian
        let mut u256_value = U256::zero();
        u256_value = U256::from_big_endian(&hash_array);

        Hash(u256_value)
        */
    }

    // check if a hash matches a target (for POW)
    pub fn matches_target(&self, target: U256) -> bool {
        self.0 <= target
    }

    // zero hash
    pub fn zero() -> Self {
        Hash(U256::zero())
    }

    // convert to bytes
    pub fn as_bytes(&self) -> [u8; 32] {
        let mut bytes: Vec<u8> = vec![0; 32];
        // the convention is typically little-endian. Bitcoin specifically uses little-endian for hashing and serialization.
        self.0.to_little_endian(&mut bytes);
        bytes.as_slice().try_into().unwrap()
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
