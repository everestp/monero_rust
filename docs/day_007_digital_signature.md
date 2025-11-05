**DAY 7: Digital Signatures Practice**  
**Goal:** Build a **signed transaction** using **Ed25519** (Day 4) and **hashing** (Day 4)  
**Repo Task:**  
> Implement transactions with signing and verification in `/src/blockchain/transaction.rs`

We’ll create a **realistic transaction struct**, **sign it**, **verify it**, and **serialize it** — the **core of blockchain integrity**.

---

## Step-by-Step Guide for Day 7

---

### Step 1: Create `src/blockchain/transaction.rs`

```bash
touch src/blockchain/transaction.rs
```

---

### Step 2: `src/blockchain/transaction.rs`

```rust
// src/blockchain/transaction.rs

use crate::crypto::hash::blake2b;
use crate::crypto::signature::{Ed25519Keypair, verify_signature};
use serde::{Serialize, Deserialize};

/// A simple transaction: sender → receiver, amount
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub sender: Vec<u8>,      // Public key (32 bytes)
    pub receiver: Vec<u8>,    // Public key (32 bytes)
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    pub signature: Vec<u8>,   // Ed25519 signature (64 bytes)
}

impl Transaction {
    /// Create a new unsigned transaction
    pub fn new(sender: Vec<u8>, receiver: Vec<u8>, amount: u64, fee: u64, nonce: u64) -> Self {
        Self {
            sender,
            receiver,
            amount,
            fee,
            nonce,
            signature: vec![], // Will be filled after signing
        }
    }

    /// Hash of the transaction (without signature)
    pub fn hash(&self) -> Vec<u8> {
        let data = serde_json::to_vec(&UnsignedTx {
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
            amount: self.amount,
            fee: self.fee,
            nonce: self.nonce,
        }).expect("Serialization failed");
        blake2b(&data).0
    }

    /// Sign the transaction using sender's keypair
    pub fn sign(&mut self, keypair: &Ed25519Keypair) -> Result<(), &'static str> {
        if keypair.public_bytes().to_vec() != self.sender {
            return Err("Sender public key mismatch");
        }

        let tx_hash = self.hash();
        let signature = keypair.sign(&tx_hash);
        self.signature = signature.to_bytes().to_vec();
        Ok(())
    }

    /// Verify the transaction signature
    pub fn verify(&self) -> bool {
        if self.signature.len() != 64 {
            return false;
        }

        let tx_hash = self.hash();
        verify_signature(&self.sender, &tx_hash, &self.signature).is_ok()
    }

    /// Get transaction ID (hash of signed tx)
    pub fn id(&self) -> Vec<u8> {
        let signed_data = serde_json::to_vec(self).expect("Serialization failed");
        blake2b(&signed_data).0
    }
}

/// Internal struct for hashing (excludes signature)
#[derive(Serialize, Deserialize)]
struct UnsignedTx {
    sender: Vec<u8>,
    receiver: Vec<u8>,
    amount: u64,
    fee: u64,
    nonce: u64,
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::signature::Ed25519Keypair;

    fn create_keypair() -> Ed25519Keypair {
        Ed25519Keypair::generate()
    }

    #[test]
    fn test_transaction_sign_and_verify() {
        let alice = create_key_pair();
        let bob = create_key_pair();

        let mut tx = Transaction::new(
            alice.public_bytes().to_vec(),
            bob.public_bytes().to_vec(),
            100,
            1,
            42,
        );

        // Sign
        tx.sign(&alice).expect("Signing failed");

        // Verify
        assert!(tx.verify());

        // Check signature length
        assert_eq!(tx.signature.len(), 64);
    }

    #[test]
    fn test_invalid_signature() {
        let alice = create_key_pair();
        let bob = create_key_pair();
        let eve = create_key_pair();

        let mut tx = Transaction::new(
            alice.public_bytes().to_vec(),
            bob.public_bytes().to_vec(),
            100,
            1,
            42,
        );

        tx.sign(&alice).unwrap();

        // Tamper with amount
        let mut tampered = tx.clone();
        tampered.amount = 999;
        assert!(!tampered.verify());

        // Wrong key
        let mut wrong_key = tx.clone();
        wrong_key.sender = eve.public_bytes().to_vec();
        assert!(!wrong_key.verify());
    }

    #[test]
    fn test_transaction_id_determinism() {
        let alice = create_key_pair();
        let bob = create_key_pair();

        let mut tx1 = Transaction::new(
            alice.public_bytes().to_vec(),
            bob.public_bytes().to_vec(),
            50,
            2,
            1,
        );
        let mut tx2 = tx1.clone();

        tx1.sign(&alice).unwrap();
        tx2.sign(&alice).unwrap();

        assert_eq!(tx1.id(), tx2.id());
    }

    #[test]
    fn test_transaction_serialization() {
        let alice = create_key_pair();
        let bob = create_key_pair();

        let mut tx = Transaction::new(
            alice.public_bytes().to_vec(),
            bob.public_bytes().to_vec(),
            75,
            1,
            100,
        );
        tx.sign(&alice).unwrap();

        let json = serde_json::to_string(&tx).unwrap();
        let deserialized: Transaction = serde_json::from_str(&json).unwrap();

        assert_eq!(tx, deserialized);
        assert!(deserialized.verify());
    }

    #[test]
    fn test_empty_signature() {
        let tx = Transaction::new(
            vec![0; 32],
            vec![1; 32],
            10,
            0,
            0,
        );
        assert!(!tx.verify());
    }
}
```

---

### Step 3: Add `serde` to `Cargo.toml`

```bash
cargo add serde --features=derive
cargo add serde_json
```

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

### Step 4: Update `src/blockchain/mod.rs`

```rust
// src/blockchain/mod.rs

pub mod merkle;
pub mod transaction;
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
running 5 tests
test blockchain::transaction::tests::test_transaction_sign_and_verify ... ok
test blockchain::transaction::tests::test_invalid_signature ... ok
test blockchain::transaction::tests::test_transaction_id_determinism ... ok
test blockchain::transaction::tests::test_transaction_serialization ... ok
test blockchain::transaction::tests::test_empty_signature ... ok
test result: ok. 5 passed
```

---

### Step 7: Git Commit

```bash
git add src/blockchain/transaction.rs src/blockchain/mod.rs Cargo.toml src/lib.rs
git commit -m "Day 7: Signed Transaction with Ed25519, hash, verify, serde (5 tests)"
```

---

### Step 8: Push

```bash
git push origin main
```

---

## Why This Matters for Monero

| Feature | Monero Equivalent |
|-------|-------------------|
| `Transaction::hash()` | `tx_prefix_hash` |
| `sign()` + `verify()` | `tx_key` signature |
| `id()` | `transaction_hash` |
| `serde` | For P2P & storage |

> **This is how Monero ensures only the owner can spend coins.**

---

## Pro Tips

- **Later**: Add **inputs/outputs** (UTXO model)
- **Later**: Use **Merkle tree of txs** (Day 5)
- **Later**: Add **RingCT inputs** (Phase 3)

---

## Day 7 Complete!

| Done |
|------|
| `src/blockchain/transaction.rs` |
| Full signed transaction |
| `sign()`, `verify()`, `hash()`, `id()` |
| `serde` for network/DB |
| 5 robust tests |
| Git commit |

---

## Tomorrow (Day 8): Monero Privacy Concepts

We’ll:
- **Study** Ring Signatures, Stealth Addresses, RingCT
- Write `/docs/privacy_notes.md`
- Build **small prototypes** (no full impl)
- Prep for **Phase 3**

```bash
mkdir -p docs
touch docs/privacy_notes.md
```

---

**Ready?** Say:  
> `Yes, Day 8`

Or ask:
- “Can I add inputs/outputs now?”
- “Use ECC for stealth addresses?”
- “Add `Hashable` trait?”

We’re **7/50** — your **blockchain now has secure, signed transactions**
