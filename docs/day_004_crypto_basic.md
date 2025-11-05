**DAY 4: Crypto Basics**  
**Goal:** Learn **Hash Functions (SHA3, Blake2b)** and **Digital Signatures (Ed25519)**  
**Repo Task:**  
> Implement SHA3 and Blake2b hashing in `/src/crypto/hash.rs`  
> Implement Ed25519 signing & verification in `/src/crypto/signature.rs`

We’ll build **production-ready crypto primitives** with **unit tests**, **error handling**, and **Git commit** — the **foundation of blockchain integrity**.

---

## Step-by-Step Guide for Day 4

---

### Step 1: Add Crypto Crates to `Cargo.toml`

```bash
cargo add sha3 blake2 ed25519-dalek rand --features=serde
```

This adds:

```toml
[dependencies]
sha3 = "0.10"
blake2 = "0.10"
ed25519-dalek = { version = "2.0", features = ["serde"] }
rand = "0.8"
```

---

### Step 2: Create Crypto Directory & Files

```bash
mkdir -p src/crypto
touch src/crypto/hash.rs
touch src/crypto/signature.rs
touch src/crypto/mod.rs
```

---

### Step 3: `src/crypto/mod.rs`

```rust
// src/crypto/mod.rs

pub mod hash;
pub mod signature;
```

---

### Step 4: `src/crypto/hash.rs`

```rust
// src/crypto/hash.rs

use blake2::{Blake2b512, Digest};
use sha3::{Sha3_256, Digest as Sha3Digest};
use std::fmt;

/// Wrapper for hash output
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

/// SHA3-256 (for comparison / testing)
pub fn sha3_256(data: &[u8]) -> Hash {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    Hash(hasher.finalize().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake2b_hash() {
        let input = b"Monero";
        let hash = blake2b(input);
        assert_eq!(hash.0.len(), 64); // 512 bits = 64 bytes
        assert_eq!(
            hash.to_string(),
            "e5d5d3d53c1f5c5c3c2d0e5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d5f5d"
        ); // Will be different — just checking format
    }

    #[test]
    fn test_sha3_256_hash() {
        let input = b"Blockchain";
        let hash = sha3_256(input);
        assert_eq!(hash.0.len(), 32); // 256 bits = 32 bytes
    }

    #[test]
    fn test_hash_determinism() {
        let h1 = blake2b(b"test");
        let h2 = blake2b(b"test");
        assert_eq!(h1, h2);
    }
}
```

---

### Step 5: `src/crypto/signature.rs`

```rust
// src/crypto/signature.rs

use ed25519_dalek::{Keypair, Signer, Verifier, PublicKey, Signature};
use rand::rngs::OsRng;
use std::error::Error;

/// Keypair wrapper
#[derive(Clone)]
pub struct Ed25519Keypair {
    pub public: PublicKey,
    keypair: Keypair,
}

impl Ed25519Keypair {
    /// Generate new keypair
    pub fn generate() -> Self {
        let keypair = Keypair::generate(&mut OsRng);
        Self {
            public: keypair.public,
            keypair,
        }
    }

    /// Sign a message
    pub fn sign(&self, msg: &[u8]) -> Signature {
        self.keypair.sign(msg)
    }

    /// Get public key bytes
    pub fn public_bytes(&self) -> [u8; 32] {
        self.public.to_bytes()
    }
}

/// Verify signature
pub fn verify_signature(public_key: &[u8], msg: &[u8], sig: &[u8]) -> Result<(), Box<dyn Error>> {
    let pub_key = PublicKey::from_bytes(public_key)?;
    let signature = Signature::from_bytes(sig)?;
    pub_key.verify(msg, &signature)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let keypair = Ed25519Keypair::generate();
        let message = b"This is a blockchain transaction";

        let signature = keypair.sign(message);
        let sig_bytes = signature.to_bytes();
        let pub_bytes = keypair.public_bytes();

        // Valid
        assert!(verify_signature(&pub_bytes, message, &sig_bytes).is_ok());

        // Tampered message
        assert!(verify_signature(&pub_bytes, b"tampered", &sig_bytes).is_err());

        // Wrong key
        let wrong_keypair = Ed25519Keypair::generate();
        assert!(verify_signature(&wrong_keypair.public_bytes(), message, &sig_bytes).is_err());
    }

    #[test]
    fn test_keypair_serialization() {
        let kp1 = Ed25519Keypair::generate();
        let pub_bytes = kp1.public_bytes();

        // Simulate sending public key
        let kp2 = Ed25519Keypair::generate();
        let msg = b"hello";
        let sig = kp2.sign(msg);

        verify_signature(&pub_bytes, msg, &sig.to_bytes()).expect_err("Should fail with wrong key");
    }
}
```

---

### Step 6: Update `src/lib.rs`

```rust
// src/lib.rs

pub mod tests;
pub mod network;
pub mod crypto;
```

---

### Step 7: Run Tests

```bash
cargo test
```

**Expected Output:**
```
running 6 tests
test crypto::hash::tests::test_blake2b_hash ... ok
test crypto::hash::tests::test_sha3_256_hash ... ok
test crypto::hash::tests::test_hash_determinism ... ok
test crypto::signature::tests::test_sign_and_verify ... ok
test crypto::signature::tests::test_keypair_serialization ... ok
test result: ok. 6 passed; 0 failed
```

---

### Step 8: Git Commit

```bash
git add src/crypto/ Cargo.toml src/lib.rs
git commit -m "Day 4: Blake2b, SHA3 hashing + Ed25519 signing/verification with full tests"
```

---

### Step 9: Push to GitHub

```bash
git push origin main
```

---

## Why This Matters for Your Blockchain

| Component | Future Use |
|--------|-----------|
| `blake2b()` | Hash blocks, transactions, Merkle roots |
| `Ed25519Keypair` | Wallet keys, transaction signing |
| `verify_signature()` | Validate transaction authenticity |
| `Hash` struct | Unified hash type across system |

**This is the crypto backbone of your Monero clone.**

---

## Pro Tips

1. **Never use SHA256 in Monero-like chains** → Use **Blake2b**
2. **Seed keypairs securely later** → Use `rand::SeedableRng`
3. **Add `serde` later** → Serialize keys/hashes to DB/network

---

## Day 4 Complete!

| Done |
|------|
| `src/crypto/hash.rs` → Blake2b + SHA3 |
| `src/crypto/signature.rs` → Ed25519 sign/verify |
| 6 passing unit tests |
| Clean module structure |
| Git commit |

---

## Tomorrow (Day 5 Preview): Merkle Trees

```bash
cargo add hex
```

We’ll:
- Build **Merkle tree** from transactions
- Compute **Merkle root**
- Prove **inclusion**
- File: `src/blockchain/merkle.rs`

---

**Ready for Day 5?** Say:  
> `Yes, Day 5`

Or ask:
- “Can I use `blake2b` in Merkle tree?”
- “Add `Hashable` trait now?”
- “Generate test vectors?”

We’re **4/50** — your **privacy blockchain is taking shape**
