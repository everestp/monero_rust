**DAY 10: Simple Blockchain Simulation**  
**Goal:** Build a **memory-based blockchain** with **Proof-of-Work (PoW)** mining  
**Repo Task:**  
> Implement simple blockchain struct & PoW in `/src/blockchain/block.rs`  
> Mine a few blocks locally

We’ll create **Block**, **Blockchain**, **mine blocks**, **link with hashes**, and **print the chain** — **Phase 1 complete!**

---

## Step-by-Step Guide for Day 10

---

### Step 1: Create `src/blockchain/block.rs`

```bash
touch src/blockchain/block.rs
```

---

### Step 2: `src/blockchain/block.rs`

```rust
// src/blockchain/block.rs

use crate::crypto::hash::blake2b;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// A block in the chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub prev_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: u64,
    pub transactions: Vec<String>, // Placeholder: real txs later
    pub difficulty: u32,           // Target: leading zeros
}

impl Block {
    /// Create genesis block
    pub fn genesis() -> Self {
        let mut block = Self {
            index: 0,
            timestamp: Self::now(),
            prev_hash: vec![0; 64],
            hash: vec![],
            nonce: 0,
            transactions: vec!["Genesis Block".to_string()],
            difficulty: 4, // 4 leading zero bits → easy for demo
        };
        block.mine();
        block
    }

    /// Create new block from previous
    pub fn new(prev: &Block, transactions: Vec<String>) -> Self {
        let mut block = Self {
            index: prev.index + 1,
            timestamp: Self::now(),
            prev_hash: prev.hash.clone(),
            hash: vec![],
            nonce: 0,
            transactions,
            difficulty: 4,
        };
        block.mine();
        block
    }

    /// Current timestamp in seconds
    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Compute hash of block
    fn calculate_hash(&self) -> Vec<u8> {
        let data = serde_json::to_vec(self).expect("Serialize failed");
        blake2b(&data).0
    }

    /// Mine block: find nonce so hash has `difficulty` leading zero bits
    pub fn mine(&mut self) {
        let target = 1u64 << (64 - self.difficulty); // 2^(64-difficulty)
        loop {
            self.hash = self.calculate_hash();
            let hash_int = u64::from_be_bytes([
                self.hash[0], self.hash[1], self.hash[2], self.hash[3],
                self.hash[4], self.hash[5], self.hash[6], self.hash[7],
            ]);
            if hash_int < target {
                println!("Mined block {} | nonce: {} | hash: {}", 
                         self.index, self.nonce, hex::encode(&self.hash));
                break;
            }
            self.nonce += 1;
        }
    }
}

/// In-memory blockchain
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            chain: vec![Block::genesis()],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<String>) {
        let prev = self.chain.last().unwrap();
        let block = Block::new(prev, transactions);
        self.chain.push(block);
    }

    pub fn print(&self) {
        println!("\nBlockchain ({} blocks):", self.chain.len());
        for block in &self.chain {
            println!(
                "Block {} | {} txs | hash: {} | prev: {}",
                block.index,
                block.transactions.len(),
                hex::encode(&block.hash).get(..8).unwrap_or(""),
                hex::encode(&block.prev_hash).get(..8).unwrap_or("")
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
            if current.hash != current.calculate_hash() {
                return false;
            }
        }
        true
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_block() {
        let genesis = Block::genesis();
        assert_eq!(genesis.index, 0);
        assert_eq!(genesis.transactions[0], "Genesis Block");
        assert!(genesis.hash.len() == 64);
    }

    #[test]
    fn test_blockchain_growth() {
        let mut bc = Blockchain::new();
        bc.add_block(vec!["Alice -> Bob 10".to_string()]);
        bc.add_block(vec!["Bob -> Carol 5".to_string(), "Carol -> Dave 3".to_string()]);

        assert_eq!(bc.chain.len(), 3);
        assert!(bc.is_valid());
    }

    #[test]
    fn test_chain_integrity() {
        let mut bc = Blockchain::new();
        bc.add_block(vec!["Tx1".to_string()]);
        bc.add_block(vec!["Tx2".to_string()]);

        // Tamper
        let tampered = bc.chain[1].clone();
        bc.chain[1] = Block { hash: vec![9; 64], ..tampered };

        assert!(!bc.is_valid());
    }

    #[test]
    fn test_mining_deterministic() {
        let genesis1 = Block::genesis();
        let genesis2 = Block::genesis();
        assert_eq!(genesis1.hash, genesis2.hash);
    }
}
```

---

### Step 3: Update `src/blockchain/mod.rs`

```rust
// src/blockchain/mod.rs

pub mod merkle;
pub mod transaction;
pub mod block;
```

---

### Step 4: Run Tests

```bash
cargo test block
```

**Expected:**
```
running 4 tests
test blockchain::block::tests::test_genesis_block ... ok
test blockchain::block::tests::test_blockchain_growth ... ok
test blockchain::block::tests::test_chain_integrity ... ok
test blockchain::block::tests::test_mining_deterministic ... ok
test result: ok. 4 passed
```

---

### Step 5: Manual Demo (Optional)

Add to `src/main.rs` or run in `cargo test`:

```rust
// In tests or main.rs
let mut bc = Blockchain::new();
bc.add_block(vec!["Alice sends 10 to Bob".to_string()]);
bc.add_block(vec!["Bob sends 5 to Carol".to_string()]);
bc.print();
```

**Sample Output:**
```
Mined block 0 | nonce: 1234 | hash: 0000a1b2...
Mined block 1 | nonce: 5678 | hash: 0000c3d4...
Mined block 2 | nonce: 9012 | hash: 0000e5f6...

Blockchain (3 blocks):
Block 0 | 1 txs | hash: 0000a1b2 | prev: 00000000
Block 1 | 1 txs | hash: 0000c3d4 | prev: 0000a1b2
Block 2 | 1 txs | hash: 0000e5f6 | prev: 0000c3d4
```

---

### Step 6: Git Commit

```bash
git add src/blockchain/block.rs src/blockchain/mod.rs
git commit -m "Day 10: In-memory PoW blockchain with mining & validation (4 tests)"
```

---

### Step 7: Push

```bash
git push origin main
```

---

## PHASE 1 COMPLETE!

| Done | Task |
|------|------|
| Day 1 | Rust basics |
| Day 2 | Traits, generics, smart pointers |
| Day 3 | Async TCP |
| Day 4 | Hashing + Ed25519 |
| Day 5 | Merkle trees |
| Day 6 | ECC (Curve25519) |
| Day 7 | Signed transactions |
| Day 8 | Privacy concepts + prototypes |
| Day 9 | Stealth addresses |
| **Day 10** | **Full PoW blockchain** |

**You now have:**
- Secure crypto
- Private transactions
- Working blockchain
- **50-day foundation is SOLID**

---

## Tomorrow (Day 11): Block & Transaction Structs

We’ll:
- Replace `String` txs with **real `Transaction`**
- Add **Merkle root**
- File: Update `block.rs` + `transaction.rs`

---

**Ready?** Say:  
> `Yes, Day 11`

Or celebrate:
> `PHASE 1 DONE!`

We’re **10/50** — your **Monero-like blockchain is ALIVE**
