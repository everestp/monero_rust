**DAY 13: Hash Functions**  
 
**Goal:** Implement **Blake2b hashing** for **blocks and transactions**  
**Repo Task:**  
> Add hashing of transactions & blocks in `/src/blockchain/hash_block.rs`

We’ll **finalize hashing logic** using **Blake2b-512**, **serialize properly**, and **ensure determinism** — **critical for consensus**.

---

## Step-by-Step Guide for Day 13

---

### Step 1: Create `src/blockchain/hash_block.rs`

```bash
touch src/blockchain/hash_block.rs
```

---

### Step 2: `src/blockchain/hash_block.rs`

```rust
// src/blockchain/hash_block.rs

use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use crate::crypto::hash::blake2b;
use serde::Serialize;

/// Trait for hashable objects
pub trait Hashable {
    fn hash(&self) -> Vec<u8>;
}

/// Hash a transaction (excluding signature for prefix hash)
impl Hashable for Transaction {
    fn hash(&self) -> Vec<u8> {
        // Use prefix: exclude signature
        let prefix = TransactionPrefix {
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
            amount: self.amount,
            fee: self.fee,
            nonce: self.nonce,
        };
        let data = bincode::serialize(&prefix).expect("Serialize failed");
        blake2b(&data).0
    }
}

/// Transaction prefix (for Monero-style prefix hash)
#[derive(Serialize)]
struct TransactionPrefix {
    sender: Vec<u8>,
    receiver: Vec<u8>,
    amount: u64,
    fee: u64,
    nonce: u64,
}

/// Hash full transaction including signature → tx_id
pub fn transaction_id(tx: &Transaction) -> Vec<u8> {
    let data = bincode::serialize(tx).expect("Serialize failed");
    blake2b(&data).0
}

/// Hash block header (for PoW)
impl Hashable for Block {
    fn hash(&self) -> Vec<u8> {
        let header = BlockHeader {
            index: self.index,
            timestamp: self.timestamp,
            prev_hash: self.prev_hash.clone(),
            merkle_root: self.merkle_root.clone(),
            nonce: self.nonce,
            difficulty: self.difficulty,
        };
        let data = bincode::serialize(&header).expect("Serialize failed");
        blake2b(&data).0
    }
}

/// Block header (serialized for hashing)
#[derive(Serialize)]
struct BlockHeader {
    index: u64,
    timestamp: u64,
    prev_hash: Vec<u8>,
    merkle_root: Vec<u8>,
    nonce: u64,
    difficulty: u32,
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::signature::Ed25519Keypair;

    fn create_tx() -> Transaction {
        let alice = Ed25519Keypair::generate();
        let bob = Ed25519Keypair::generate();
        let mut tx = Transaction::new(
            alice.public_bytes().to_vec(),
            bob.public_bytes().to_vec(),
            100,
            1,
            1,
        );
        tx.sign(&alice).unwrap();
        tx
    }

    #[test]
    fn test_transaction_prefix_hash() {
        let tx = create_tx();
        let prefix_hash = tx.hash();
        assert_eq!(prefix_hash.len(), 64);

        // Same prefix → same hash
        let mut tx2 = tx.clone();
        tx2.signature = vec![9; 64]; // different sig
        assert_eq!(tx.hash(), tx2.hash());
    }

    #[test]
    fn test_transaction_id() {
        let tx = create_tx();
        let tx_id = transaction_id(&tx);
        assert_eq!(tx_id.len(), 64);
        assert_eq!(tx.id(), tx_id); // matches Day 7
    }

    #[test]
    fn test_block_header_hash() {
        let genesis = Block::genesis();
        let header_hash = genesis.hash();
        assert_eq!(header_hash, genesis.hash); // matches mining
        assert_eq!(header_hash.len(), 64);
    }

    #[test]
    fn test_hash_determinism() {
        let tx1 = create_tx();
        let tx2 = tx1.clone();

        assert_eq!(tx1.hash(), tx2.hash());
        assert_eq!(transaction_id(&tx1), transaction_id(&tx2));
    }

    #[test]
    fn test_different_nonces_different_hash() {
        let mut block1 = Block::genesis();
        let block2 = block1.clone();
        block1.nonce += 1;

        assert_ne!(block1.hash(), block2.hash());
    }
}
```

---

### Step 3: Add `bincode` to `Cargo.toml`

```bash
cargo add bincode
```

```toml
[dependencies]
bincode = "1.3"
```

---

### Step 4: Update `src/blockchain/mod.rs`

```rust
// src/blockchain/mod.rs

pub mod merkle;
pub mod transaction;
pub mod block;
pub mod verify;
pub mod hash_block;
```

---

### Step 5: Update `src/blockchain/block.rs` to Use `Hashable`

Replace `header_hash()` and `calculate_hash()` with trait:

```rust
// In Block impl
use crate::blockchain::hash_block::Hashable;

fn header_hash(&self) -> Vec<u8> {
    self.hash() // uses trait
}
```

And remove old `calculate_hash()`.

---

### Step 6: Run Tests

```bash
cargo test hash_block
```

**Expected:**
```
running 5 tests
test blockchain::hash_block::tests::test_transaction_prefix_hash ... ok
test blockchain::hash_block::tests::test_transaction_id ... ok
test blockchain::hash_block::tests::test_block_header_hash ... ok
test blockchain::hash_block::tests::test_hash_determinism ... ok
test blockchain::hash_block::tests::test_different_nonces_different_hash ... ok
test result: ok. 5 passed
```

---

### Step 7: Git Commit

```bash
git add src/blockchain/hash_block.rs src/blockchain/block.rs Cargo.toml src/blockchain/mod.rs
git commit -m "Day 13: Blake2b hashing for tx prefix, tx_id, block header via Hashable trait (5 tests)"
```

---

### Step 8: Push

```bash
git push origin main
```

---

## Why This Matters

| Hash Type | Purpose | Monero Equivalent |
|---------|-------|-------------------|
| `tx.hash()` | Prefix hash | `tx_prefix_hash` |
| `transaction_id()` | Full tx hash | `tx_hash` |
| `block.hash()` | Header hash | `block_hash` (PoW) |

> **Every node computes the same hash → consensus**

---

## Day 13 Complete!

| Done |
|------|
| `src/blockchain/hash_block.rs` |
| `Hashable` trait |
| **Prefix hash** (sig excluded) |
| **Full tx_id** |
| **Block header hash** |
| 5 passing tests |
| Git commit |

---

## Tomorrow (Day 14): Proof-of-Work Mining

We’ll:
- **Mine with real difficulty**
- **Adjust difficulty**
- **Benchmark**
- File: `src/blockchain/mining.rs`

```bash
touch src/blockchain/mining.rs
```

---

**Ready?** Say:  
> `Yes, Day 14`

Or ask:
- “Can I use GPU mining?”
- “Add difficulty retarget?”
- “Mine 10 blocks?”

We’re **13/50** — **Your blockchain now has CRYPTOGRAPHIC INTEGRITY**
