**DAY 11: Block & Transaction Structs**  
**Goal:** Upgrade the blockchain to use **real `Transaction` structs** (from Day 7) and **Merkle root** (from Day 5)  
**Repo Task:**  
> Update `block.rs` to include `Vec<Transaction>` and `merkle_root`  
> Add fields: `timestamp`, `nonce`, `previous_hash`, `sender/receiver`, `amount`

We’ll **refactor** the blockchain to be **production-ready** — **Phase 2 begins!**

---

## Step-by-Step Guide for Day 11

---

### Step 1: Update `src/blockchain/block.rs`

Replace the old `Block` with a **fully featured version** using real transactions and Merkle root.

```rust
// src/blockchain/block.rs

use crate::blockchain::merkle::MerkleTree;
use crate::blockchain::transaction::Transaction;
use crate::crypto::hash::blake2b;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub prev_hash: Vec<u8>,
    pub merkle_root: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u32,
}

impl Block {
    /// Genesis block
    pub fn genesis() -> Self {
        let genesis_tx = Transaction::new(
            vec![0; 32], // dummy sender
            vec![0; 32], // dummy receiver
            0,
            0,
            0,
        );
        let mut block = Self {
            index: 0,
            timestamp: Self::now(),
            prev_hash: vec![0; 64],
            merkle_root: vec![],
            hash: vec![],
            nonce: 0,
            transactions: vec![genesis_tx],
            difficulty: 4,
        };
        block.update_merkle_root();
        block.mine();
        block
    }

    /// New block from previous + transactions
    pub fn new(prev: &Block, transactions: Vec<Transaction>) -> Self {
        let mut block = Self {
            index: prev.index + 1,
            timestamp: Self::now(),
            prev_hash: prev.hash.clone(),
            merkle_root: vec![],
            hash: vec![],
            nonce: 0,
            transactions,
            difficulty: 4,
        };
        block.update_merkle_root();
        block.mine();
        block
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Update Merkle root from transactions
    pub fn update_merkle_root(&mut self) {
        let tx_hashes: Vec<&[u8]> = self.transactions.iter()
            .map(|tx| tx.id().as_slice())
            .collect();

        if let Some(tree) = MerkleTree::build(tx_hashes) {
            self.merkle_root = tree.root_hash();
        } else {
            self.merkle_root = blake2b(b"empty").0;
        }
    }

    /// Compute block header hash (for mining)
    fn header_hash(&self) -> Vec<u8> {
        let header = serde_json::json!({
            "index": self.index,
            "timestamp": self.timestamp,
            "prev_hash": hex::encode(&self.prev_hash),
            "merkle_root": hex::encode(&self.merkle_root),
            "nonce": self.nonce,
            "difficulty": self.difficulty,
        });
        let data = serde_json::to_vec(&header).unwrap();
        blake2b(&data).0
    }

    /// Mine block
    pub fn mine(&mut self) {
        let target = 1u64 << (64 - self.difficulty);
        loop {
            self.hash = self.header_hash();
            let hash_int = u64::from_be_bytes([
                self.hash[0], self.hash[1], self.hash[2], self.hash[3],
                self.hash[4], self.hash[5], self.hash[6], self.hash[7],
            ]);
            if hash_int < target {
                println!("Mined block {} | nonce: {} | hash: {}", 
                         self.index, self.nonce, hex::encode(&self.hash[..8]));
                break;
            }
            self.nonce += 1;
        }
    }
}

/// Blockchain
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { chain: vec![Block::genesis()] }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let prev = self.chain.last().unwrap().clone();
        let block = Block::new(&prev, transactions);
        self.chain.push(block);
    }

    pub fn print(&self) {
        println!("\nBlockchain ({} blocks):", self.chain.len());
        for block in &self.chain {
            println!(
                "Block {} | {} txs | merkle: {} | hash: {}",
                block.index,
                block.transactions.len(),
                hex::encode(&block.merkle_root[..8]),
                hex::encode(&block.hash[..8])
            );
        }
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let prev = &self.chain[i - 1];

            if current.prev_hash != prev.hash {
                return false;
            }
            if current.hash != current.header_hash() {
                return false;
            }
            let expected_merkle: Vec<u8> = current.transactions.iter()
                .map(|tx| tx.id())
                .collect::<Vec<_>>()
                .into_iter()
                .map(|h| h.as_slice())
                .collect::<Vec<_>>()
                .into_iter()
                .collect();
            let expected_tree = MerkleTree::build(expected_merkle);
            if let Some(tree) = expected_tree {
                if tree.root_hash() != current.merkle_root {
                    return false;
                }
            }
        }
        true
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::signature::Ed25519Keypair;

    fn create_signed_tx() -> Transaction {
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
    fn test_block_with_real_transactions() {
        let tx1 = create_signed_tx();
        let tx2 = create_signed_tx();

        let mut bc = Blockchain::new();
        bc.add_block(vec![tx1.clone(), tx2.clone()]);

        let block = bc.chain.last().unwrap();
        assert_eq!(block.transactions.len(), 2);
        assert!(block.transactions[0].verify());
        assert!(block.transactions[1].verify());
        assert_eq!(block.merkle_root.len(), 64);
    }

    #[test]
    fn test_merkle_root_correctness() {
        let tx1 = create_signed_tx();
        let tx2 = create_signed_tx();

        let leaves = vec![tx1.id().as_slice(), tx2.id().as_slice()];
        let tree = MerkleTree::build(leaves).unwrap();

        let mut bc = Blockchain::new();
        bc.add_block(vec![tx1, tx2]);
        let block = bc.chain.last().unwrap();

        assert_eq!(block.merkle_root, tree.root_hash());
    }

    #[test]
    fn test_chain_validation() {
        let mut bc = Blockchain::new();
        bc.add_block(vec![create_signed_tx()]);
        bc.add_block(vec![create_signed_tx()]);
        assert!(bc.is_valid());
    }
}
```

---

### Step 2: Update `src/blockchain/mod.rs`

```rust
// src/blockchain/mod.rs

pub mod merkle;
pub mod transaction;
pub mod block;
```

---

### Step 3: Run Tests

```bash
cargo test block
```

**Expected:**
```
running 3 tests
test blockchain::block::tests::test_block_with_real_transactions ... ok
test blockchain::block::tests::test_merkle_root_correctness ... ok
test blockchain::block::tests::test_chain_validation ... ok
test result: ok. 3 passed
```

---

### Step 4: Git Commit

```bash
git add src/blockchain/block.rs
git commit -m "Day 11: Block with real Transaction[], Merkle root, full validation (3 tests)"
```

---

### Step 5: Push

```bash
git push origin main
```

---

## What Changed?

| Old | New |
|-----|-----|
| `Vec<String>` | `Vec<Transaction>` |
| No Merkle root | `merkle_root: Vec<u8>` |
| Hash all fields | Hash **header only** (Monero style) |
| No validation | Full `is_valid()` |

---

## Why This Matters

| Feature | Monero Equivalent |
|-------|-------------------|
| `merkle_root` | `tx_hash` in block header |
| `prev_hash` | `previous_block_hash` |
| `nonce` + `difficulty` | PoW |
| `timestamp` | `timestamp` |

> **This is how Monero blocks are structured**

---

## Day 11 Complete!

| Done |
|------|
| `Block` uses real `Transaction` |
| `merkle_root` from `tx.id()` |
| Mining on **header hash** |
| Full chain validation |
| 3 passing tests |
| Git commit |

---

## Tomorrow (Day 12): Transaction Verification

We’ll:
- **Verify signatures**
- **Check double-spends** (UTXO set)
- **Prevent invalid amounts**
- File: `src/blockchain/verify.rs`

```bash
touch src/blockchain/verify.rs
```

---

**Ready?** Say:  
> `Yes, Day 12`

Or ask:
- “Can I add UTXO set now?”
- “Use stealth outputs?”
- “Add block height?”

We’re **11/50** — **Phase 2: Core Blockchain is ON FIRE**
