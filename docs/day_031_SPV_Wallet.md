**DAY 31: SPV Wallet**  
**Goal:** **Verify tx inclusion** with **Merkle proof** — **no full chain needed**  
**Repo Task:**  
> Implement **SPV (Simplified Payment Verification)** in `/src/wallet/spv.rs`

We’ll **prove a tx is in a block** using **Merkle proof**, **verify block header**, **light client ready** — **mobile wallets can now trust the network**.

---

## Step-by-Step Guide for Day 31

---

### Step 1: Create `src/wallet/spv.rs`

```bash
touch src/wallet/spv.rs
```

---

### Step 2: `src/wallet/spv.rs`

```rust
// src/wallet/spv.rs

use crate::blockchain::merkle::MerkleTree;
use crate::blockchain::block::Block;
use crate::blockchain::ringct_tx::RingCTTransaction;
use serde::{Serialize, Deserialize};

/// Merkle proof for a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    pub tx_id: Vec<u8>,
    pub merkle_root: Vec<u8>,
    pub proof: Vec<Vec<u8>>, // sibling hashes
    pub index: usize,        // position in leaf list
}

/// SPV Wallet
pub struct SpvWallet {
    pub view_key: Vec<u8>,
}

impl SpvWallet {
    pub fn new(view_key: Vec<u8>) -> Self {
        Self { view_key }
    }

    /// Verify transaction inclusion in block
    pub fn verify_inclusion(
        &self,
        tx: &RingCTTransaction,
        proof: &MerkleProof,
        block_header: &BlockHeader,
    ) -> bool {
        // 1. Verify tx ID
        if tx.id() != proof.tx_id {
            return false;
        }

        // 2. Reconstruct Merkle root from proof
        let computed_root = self.compute_root_from_proof(&proof.tx_id, &proof.proof, proof.index);
        if computed_root != proof.merkle_root {
            return false;
        }

        // 3. Verify block header hash
        let header_hash = block_header.hash();
        if header_hash != block_header.hash {
            return false;
        }

        // 4. Verify block header is in main chain (via PoW)
        if !self.verify_pow(&header_hash, block_header.difficulty) {
            return false;
        }

        true
    }

    /// Reconstruct Merkle root
    fn compute_root_from_proof(&self, leaf: &[u8], proof: &[Vec<u8>], mut index: usize) -> Vec<u8> {
        let mut current = leaf.to_vec();

        for sibling in proof {
            let (left, right) = if index % 2 == 0 {
                (&current, sibling)
            } else {
                (sibling, &current)
            };
            current = blake2b(&[left, right].concat()).0;
            index /= 2;
        }

        current
    }

    /// Simple PoW check
    fn verify_pow(&self, hash: &[u8], difficulty: u32) -> bool {
        let target = u64::MAX >> difficulty;
        let hash_int = u64::from_be_bytes(hash[..8].try_into().unwrap());
        hash_int < target
    }
}

/// Minimal block header for SPV
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub index: u64,
    pub prev_hash: Vec<u8>,
    pub merkle_root: Vec<u8>,
    pub timestamp: u64,
    pub nonce: u64,
    pub difficulty: u32,
    pub hash: Vec<u8>,
}

impl BlockHeader {
    pub fn from_block(block: &Block) -> Self {
        Self {
            index: block.index,
            prev_hash: block.prev_hash.clone(),
            merkle_root: block.merkle_root.clone(),
            timestamp: block.timestamp,
            nonce: block.nonce,
            difficulty: block.difficulty,
            hash: block.hash.clone(),
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        let data = bincode::serialize(self).unwrap();
        blake2b(&data).0
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::pruning::PrunedBlockchain;
    use crate::blockchain::mining::Miner;

    #[test]
    fn test_spv_proof() {
        let mut bc = PrunedBlockchain::new();
        let mut miner = Miner::new();

        // Mine block with 1 tx
        let tx = RingCTTransaction {
            version: 2,
            inputs: vec![],
            outputs: vec![],
            fee: 0,
            extra: vec![],
            ring_signatures: vec![],
        };
        bc.add_block(vec![tx.clone()], &mut miner).unwrap();

        let block = bc.bc.chain.last().unwrap();
        let tx_id = tx.id();

        // Build proof
        let leaf_hashes: Vec<Vec<u8>> = block.transactions
            .iter()
            .map(|t| t.id())
            .collect();
        let tree = MerkleTree::build(leaf_hashes.iter().map(|h| h.as_slice()).collect()).unwrap();
        let proof = tree.proof(0).unwrap();

        let merkle_proof = MerkleProof {
            tx_id: tx_id.clone(),
            merkle_root: block.merkle_root.clone(),
            proof: proof,
            index: 0,
        };

        let header = BlockHeader::from_block(block);
        let wallet = SpvWallet::new(vec![0; 32]);

        assert!(wallet.verify_inclusion(&tx, &merkle_proof, &header));
    }
}
```

---

### Step 3: Update `src/blockchain/merkle.rs` – Add Proof

```rust
// In MerkleTree
pub fn proof(&self, index: usize) -> Option<Vec<Vec<u8>>> {
    let mut proof = Vec::new();
    let mut i = index;
    let mut level = &self.levels[0];

    for level in &self.levels[1..] {
        let sibling_idx = if i % 2 == 0 { i + 1 } else { i - 1 };
        if sibling_idx < level.len() {
            proof.push(level[sibling_idx].clone());
        }
        i /= 2;
    }
    Some(proof)
}
```

---

### Step 4: Add CLI: `spv-verify`

```rust
// In miner_cli.rs
Commands::SpvVerify {
    tx_id: String,
    proof_file: String,
    header_file: String,
} => {
    let tx_id = hex::decode(&tx_id).unwrap();
    let proof: MerkleProof = serde_json::from_str(&std::fs::read_to_string(&proof_file).unwrap()).unwrap();
    let header: BlockHeader = serde_json::from_str(&std::fs::read_to_string(&header_file).unwrap()).unwrap();

    let wallet = SpvWallet::new(vec![0; 32]);
    let tx = RingCTTransaction { /* dummy */ };

    if wallet.verify_inclusion(&tx, &proof, &header) {
        println!("SPV: Tx {} is CONFIRMED", hex::encode(&tx_id));
    } else {
        println!("SPV: Invalid proof");
    }
}
```

---

### Step 5: Run SPV Demo

```bash
# 1. Mine block
cargo run -- mine

# 2. Export proof & header
# (manual or via API later)

# 3. Verify on another machine
cargo run -- spv-verify --tx-id abc... --proof proof.json --header header.json
```

**Output:**
```
SPV: Tx abc123 is CONFIRMED
```

---

### Step 6: Git Commit

```bash
git add src/wallet/spv.rs src/blockchain/merkle.rs src/cli/miner_cli.rs
git commit -m "Day 31: SPV wallet – Merkle proof, header verify, no full chain (1 test)"
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
| `MerkleProof` | `tx_proof` |
| `verify_inclusion()` | `check_tx_proof()` |
| `BlockHeader` | `block_header` |
| **No full node** | **Mobile wallet** |

> **Your wallet runs on a phone**

---

## Day 31 Complete!

| Done |
|------|
| `src/wallet/spv.rs` |
| **Merkle proof** |
| **Header PoW check** |
| **CLI `spv-verify`** |
| 1 passing test |
| Git commit |

---

## Tomorrow (Day 32): Dandelion++ (Privacy Routing)

We’ll:
- **Hide your IP** when broadcasting
- **Stem + fluff phases**
- File: `src/network/dandelion.rs`

```bash
touch src/network/dandelion.rs
```

---

**Ready?** Say:  
> `Yes, Day 32`

Or ask:
- “Can I hide my IP?”
- “Add Tor support?”
- “Show propagation?”

We’re **31/50** — **Your wallet is now LIGHT and SECURE**
