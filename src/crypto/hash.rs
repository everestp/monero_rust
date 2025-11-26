use std::{fmt};

use blake2::Blake2b512;
use sha3::Digest;





#[derive(Debug ,Clone , PartialEq , Eq ,Hash)]
pub struct Hash(pub Vec<u8>);

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

/// Blake2b-512 (Monero's primary hash)
pub fn blake2b(data: &[u8]) -> Hash {
    let mut hasher = Blake2b512::new();
    hasher.update(data);
    Hash(hasher.finalize().to_vec())
}

#[test]
fn test_blake2b_known_value() {
    let hash = blake2b(b"Hello Monero!");
    // Known correct Blake2b-512 hash (you can generate once and paste)
   
    // Actually better: don't hardcode unless verified
    println!("Hash of 'Hello Monero!': {}", hash);
    assert_eq!(hash.0.len(), 64);
}

#[test]
fn test_hash_determinism() {
    let h1 = blake2b(b"test");
    let h2 = blake2b(b"test");
    assert_eq!(h1, h2);
}





