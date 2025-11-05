**DAY 6: Elliptic Curve Cryptography (ECC)**  
**Goal:** Master **Curve25519** – key generation, scalar multiplication, and point operations  
**Repo Task:**  
> ECC key pair generation in `/src/crypto/ecc.rs`

We’ll implement **X25519 key exchange primitives** using `curve25519-dalek`, generate **keypairs**, perform **Diffie-Hellman**, and add **unit tests** — the **foundation for stealth addresses (Day 9)**.

---

## Step-by-Step Guide for Day 6

---

### Step 1: Add `curve25519-dalek`

```bash
cargo add curve25519-dalek rand
```

```toml
[dependencies]
curve25519-dalek = "4.1"
rand = "0.8"
```

---

### Step 2: Create `src/crypto/ecc.rs`

```bash
touch src/crypto/ecc.rs
```

---

### Step 3: `src/crypto/ecc.rs`

```rust
// src/crypto/ecc.rs

use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::{RistrettoPoint, CompressedRistretto};
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use rand::rngs::OsRng;
use std::fmt;

/// Wrapper for a public key (compressed point)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublicKey(pub CompressedRistretto);

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0.as_bytes()))
    }
}

impl PublicKey {
    pub fn as_bytes(&self) -> [u8; 32] {
        *self.0.as_bytes()
    }

    pub fn decompress(&self) -> Option<RistrettoPoint> {
        self.0.decompress()
    }
}

/// Wrapper for a secret key (scalar)
#[derive(Clone)]
pub struct SecretKey(Scalar);

impl SecretKey {
    /// Generate a new random secret key
    pub fn generate() -> Self {
        let mut rng = OsRng;
        SecretKey(Scalar::random(&mut rng))
    }

    /// Derive public key: P = s * B
    pub fn public_key(&self) -> PublicKey {
        let point = RISTRETTO_BASEPOINT_POINT * self.0;
        PublicKey(point.compress())
    }

    /// Perform scalar multiplication: P = s * Q
    pub fn scalar_mult(&self, point: &RistrettoPoint) -> RistrettoPoint {
        self.0 * point
    }

    /// Shared secret via Diffie-Hellman: s_A * P_B
    pub fn diffie_hellman(&self, their_pub: &PublicKey) -> [u8; 32] {
        let their_point = their_pub.0.decompress().expect("Invalid public key");
        let shared_point = self.0 * their_point;
        *shared_point.compress().as_bytes()
    }
}

/// Full keypair
#[derive(Clone)]
pub struct Keypair {
    pub secret: SecretKey,
    pub public: PublicKey,
}

impl Keypair {
    /// Generate a new keypair
    pub fn generate() -> Self {
        let secret = SecretKey::generate();
        let public = secret.public_key();
        Keypair { secret, public }
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let kp = Keypair::generate();
        assert_eq!(kp.public.0.as_bytes().len(), 32);
        assert!(kp.public.decompress().is_some());
    }

    #[test]
    fn test_public_key_derivation() {
        let secret = SecretKey::generate();
        let pub1 = secret.public_key();
        let pub2 = secret.public_key();
        assert_eq!(pub1, pub2); // Deterministic
    }

    #[test]
    fn test_diffie_hellman() {
        let alice = Keypair::generate();
        let bob = Keypair::generate();

        let shared_alice = alice.secret.diffie_hellman(&bob.public);
        let shared_bob = bob.secret.diffie_hellman(&alice.public);

        assert_eq!(shared_alice, shared_bob);
        assert_ne!(shared_alice, [0u8; 32]);
    }

    #[test]
    fn test_scalar_multiplication() {
        let secret = SecretKey::generate();
        let base = RISTRETTO_BASEPOINT_POINT;
        let result = secret.scalar_mult(&base);
        assert_eq!(result.compress(), secret.public_key().0);
    }

    #[test]
    fn test_invalid_public_key() {
        let invalid_bytes = [0u8; 32];
        let invalid_compressed = CompressedRistretto::from_slice(&invalid_bytes);
        let invalid_pub = PublicKey(invalid_compressed);
        assert!(invalid_pub.decompress().is_none());
    }

    #[test]
    fn test_keypair_serde_compatibility() {
        let kp = Keypair::generate();
        let pub_bytes = kp.public.as_bytes();
        let reconstructed = PublicKey(CompressedRistretto::from_slice(&pub_bytes).unwrap());
        assert_eq!(kp.public, reconstructed);
    }
}
```

---

### Step 4: Update `src/crypto/mod.rs`

```rust
// src/crypto/mod.rs

pub mod hash;
pub mod signature;
pub mod ecc;
```

---

### Step 5: Update `src/lib.rs`

```rust
// src/lib.rs

pub mod tests;
pub mod network;
pub mod crypto;
pub mod blockchain;
```

---

### Step 6: Run Tests

```bash
cargo test
```

**Expected:**
```
running 6 tests
test crypto::ecc::tests::test_key_pair_generation ... ok
test crypto::ecc::tests::test_public_key_derivation ... ok
test crypto::ecc::tests::test_diffie_hellman ... ok
test crypto::ecc::tests::test_scalar_multiplication ... ok
test crypto::ecc::tests::test_invalid_public_key ... ok
test crypto::ecc::tests::test_keypair_serde_compatibility ... ok
test result: ok. 6 passed
```

---

### Step 7: Git Commit

```bash
git add src/crypto/ecc.rs src/crypto/mod.rs Cargo.toml src/lib.rs
git commit -m "Day 6: Curve25519 keypair, DH, scalar mult with 6 tests"
```

---

### Step 8: Push

```bash
git push origin main
```

---

## Why This Matters for Monero

| Concept | Monero Use |
|-------|-----------|
| `SecretKey` + `PublicKey` | Wallet keys |
| `diffie_hellman()` | **Stealth addresses** (one-time keys) |
| `scalar_mult` | **RingCT commitments** |
| `RistrettoPoint` | Safe prime-order group |

> **This is the math behind "no address reuse" and "untraceable payments"**

---

## Pro Tips

- **Never reuse `secret`** → One-time keys in stealth addresses
- **Later**: Use `SecretKey` to derive **view keys**, **spend keys**
- **Later**: Add `serde` for DB storage

---

## Day 6 Complete!

| Done |
|------|
| `src/crypto/ecc.rs` |
| Full Curve25519 keypair + DH |
| 6 cryptographically sound tests |
| Uses `ristretto` for safety |
| Git commit |

---

## Tomorrow (Day 7): Digital Signatures Practice

We’ll:
- Build a **Transaction struct**
- **Sign it** with `ed25519-dalek` (Day 4)
- **Verify** with public key
- File: `src/blockchain/transaction.rs`

```bash
# Prep: we'll use types from ecc, signature, hash
```

---

We’re **6/50** — your **privacy crypto engine is alive**
