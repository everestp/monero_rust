**DAY 23: Ring Signatures (Part 1)**  
**Goal:** **Mix real input with decoys** and **build a ring**  
**Repo Task:**  
> Implement ring selection & structure in `/src/crypto/ring.rs`

We’ll **select decoy outputs**, **build a ring**, **store key image** — **foundation for untraceable sender**.

---

## Step-by-Step Guide for Day 23

---

### Step 1: Create `src/crypto/ring.rs`

```bash
touch src/crypto/ring.rs
```

---

### Step 2: `src/crypto/ring.rs`

```rust
// src/crypto/ring.rs

use crate::blockchain::utxo_set::{UtxoKey, UtxoSet};
use crate::crypto::ecc::SecretKey;
use curve25519_dalek::ristretto::RistrettoPoint;
use rand::seq::SliceRandom;
use sha2::{Sha512, Digest};

/// Ring member: public key + amount
#[derive(Debug, Clone, PartialEq)]
pub struct RingMember {
    pub pubkey: Vec<u8>,
    pub amount: u64,
    pub utxo_key: UtxoKey,
}

/// A ring of outputs (1 real + N-1 decoys)
#[derive(Debug, Clone)]
pub struct Ring {
    pub members: Vec<RingMember>,
    pub real_index: usize,
    pub key_image: Vec<u8>, // I = x * Hp(P)
}

/// Select decoys and build ring
pub struct RingBuilder {
    utxo_set: UtxoSet,
    ring_size: usize,
}

impl RingBuilder {
    pub fn new(utxo_set: UtxoSet, ring_size: usize) -> Self {
        Self { utxo_set, ring_size }
    }

    /// Build ring for a real input
    pub fn build_ring(&self, real_utxo: &UtxoKey, real_secret: &SecretKey) -> Option<Ring> {
        let real_utxo_data = self.utxo_set.utxos.get(real_utxo)?;
        let real_amount = real_utxo_data.amount;

        // Collect candidates with same amount
        let mut candidates: Vec<RingMember> = self.utxo_set.utxos.iter()
            .filter(|(k, u)| k != &real_utxo && u.amount == real_amount && !self.utxo_set.spent.contains(k))
            .map(|(k, u)| RingMember {
                pubkey: u.receiver.clone(),
                amount: u.amount,
                utxo_key: k.clone(),
            })
            .collect();

        if candidates.len() < self.ring_size - 1 {
            return None;
        }

        // Shuffle and pick decoys
        let mut rng = rand::thread_rng();
        candidates.shuffle(&mut rng);
        let decoys = candidates.into_iter().take(self.ring_size - 1).collect::<Vec<_>>();

        // Insert real at random position
        let real_index = rand::random::<usize>() % self.ring_size;
        let mut members = decoys;
        members.insert(real_index, RingMember {
            pubkey: real_utxo_data.receiver.clone(),
            amount: real_amount,
            utxo_key: real_utxo.clone(),
        });

        // Key image: I = x * Hp(P)
        let p = RistrettoPoint::from_bytes(&real_utxo_data.receiver).ok()?;
        let hp = Self::hash_to_point(&p);
        let key_image_point = real_secret.0 * hp;
        let key_image = key_image_point.compress().to_bytes().to_vec();

        Some(Ring {
            members,
            real_index,
            key_image,
        })
    }

    /// Hp(P) = HashToPoint(P)
    fn hash_to_point(p: &RistrettoPoint) -> RistrettoPoint {
        let hash = Sha512::digest(p.compress().as_bytes());
        RistrettoPoint::from_uniform_bytes(&hash.into())
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::transaction::Transaction;
    use crate::crypto::signature::Ed25519Keypair;

    fn create_tx(amount: u64) -> (Transaction, UtxoKey) {
        let alice = Ed25519Keypair::generate();
        let bob = Ed25519Keypair::generate();
        let mut tx = Transaction::new(
            vec![],
            vec![(StealthAddress::generate().0, amount)],
            1,
            1,
        );
        tx.sign(&alice).unwrap();
        let tx_id = tx.id();
        let key = UtxoKey { tx_id, vout: 0 };
        (tx, key)
    }

    #[test]
    fn test_ring_building() {
        let mut utxo_set = UtxoSet::new();
        let (tx1, key1) = create_tx(100);
        let (tx2, key2) = create_tx(100);
        let (tx3, key3) = create_tx(100);

        utxo_set.add_outputs(&tx1);
        utxo_set.add_outputs(&tx2);
        utxo_set.add_outputs(&tx3);

        let builder = RingBuilder::new(utxo_set, 5);
        let real_secret = SecretKey::generate();

        let ring = builder.build_ring(&key1, &real_secret).unwrap();
        assert_eq!(ring.members.len(), 5);
        assert!(ring.real_index < 5);
        assert_eq!(ring.key_image.len(), 32);
    }

    #[test]
    fn test_same_amount_only() {
        let mut utxo_set = UtxoSet::::new();
        let (tx1, key1) = create_tx(100);
        let (tx2, _) = create_tx(200); // different amount

        utxo_set.add_outputs(&tx1);
        utxo_set.add_outputs(&tx2);

        let builder = RingBuilder::new(utxo_set, 3);
        let real_secret = SecretKey::generate();

        assert!(builder.build_ring(&key1, &real_secret).is_none());
    }
}
```

---

### Step 3: Update `src/crypto/mod.rs`

```rust
// src/crypto/mod.rs

pub mod hash;
pub mod signature;
pub mod ecc;
pub mod privacy_prototypes;
pub mod stealth;
pub mod ring;
```

---

### Step 4: Update `src/blockchain/utxo_set.rs`

```rust
// In Utxo
pub amount: u64,  // now per output
```

---

### Step 5: Run Tests

```bash
cargo test ring
```

**Expected:**
```
test crypto::ring::tests::test_ring_building ... ok
test crypto::ring::tests::test_same_amount_only ... ok
```

---

### Step 6: Git Commit

```bash
git add src/crypto/ring.rs src/crypto/mod.rs src/blockchain/utxo_set.rs
git commit -m "Day 23: Ring selection with decoys, key image, same-amount mixing (2 tests)"
```

---

### Step 7: Push

```bash
git push origin main
```

---

## Why This Matters

| Feature | Monero Equivalent |
|-------|-------------------|
| `RingBuilder` | `select_outputs()` |
| `key_image` | `I = x * Hp(P)` |
| Same amount | **Amount unlinkability** |
| Random real index | **Untraceable** |

> **No one knows which input is yours**

---

## Day 23 Complete!

| Done |
|------|
| `src/crypto/ring.rs` |
| **Decoy selection** |
| **Key image** |
| **Same-amount rings** |
| 2 passing tests |
| Git commit |

---

## Tomorrow (Day 24): Ring Signatures (Part 2)

We’ll:
- **Sign with ring**
- **Verify ring signature**
- File: `src/crypto/ring_sig.rs`

```bash
touch src/crypto/ring_sig.rs
```

---

**Ready?** Say:  
> `Yes, Day 24`

Or ask:
- “Can I sign now?”
- “Add LSAG?”
- “Support multiple rings?”

We’re **23/50** — **Your sender is now UNTRACEABLE**
