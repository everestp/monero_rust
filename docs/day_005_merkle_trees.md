**DAY 5: Merkle Trees**  
**Goal:** Build a **Merkle Tree library** that computes **Merkle roots** and supports **inclusion proofs**  
**Repo Task:**  
> Implement Merkle tree library in `/src/blockchain/merkle.rs`

We’ll use **Blake2b hashing** (from Day 4) to build a **binary Merkle tree** with **unit tests**, **proof generation**, and **verification** — **essential for transaction inclusion in blocks**.

---

## Step-by-Step Guide for Day 5

---

### Step 1: Add `hex` for Debugging

```bash
cargo add hex
```

```toml
[dependencies]
hex = "0.4"
```

---

### Step 2: Create Blockchain Directory & File

```bash
mkdir -p src/blockchain
touch src/blockchain/merkle.rs
touch src/blockchain/mod.rs
```

---

### Step 3: `src/blockchain/mod.rs`

```rust
// src/blockchain/mod.rs

pub mod merkle;
```

---

### Step 4: `src/blockchain/merkle.rs`

```rust
// src/blockchain/merkle.rs

use crate::crypto::hash::blake2b;
use hex;

/// A Merkle Tree Node
#[derive(Debug, Clone, PartialEq)]
pub enum MerkleNode {
    Leaf { hash: Vec<u8> },
    Internal { hash: Vec<u8>, left: Box<MerkleNode>, right: Box<MerkleNode> },
}

impl MerkleNode {
    /// Get hash of this node
    pub fn hash(&self) -> &Vec<u8> {
        match self {
            MerkleNode::Leaf { hash } => hash,
            MerkleNode::Internal { hash, .. } => hash,
        }
    }
}

/// Merkle Proof for inclusion
#[derive(Debug, Clone)]
pub struct MerkleProof {
    pub leaf_hash: Vec<u8>,
    pub path: Vec<(Vec<u8>, bool)>, // (sibling_hash, is_right)
}

/// Merkle Tree
#[derive(Debug, Clone)]
pub struct MerkleTree {
    root: MerkleNode,
}

impl MerkleTree {
    /// Build tree from leaf data (Vec<&[u8]>)
    pub fn build(leaves: Vec<&[u8]>) -> Option<Self> {
        if leaves.is_empty() {
            return None;
        }

        let leaf_nodes: Vec<MerkleNode> = leaves
            .iter()
            .map(|data| MerkleNode::Leaf { hash: blake2b(data).0 })
            .collect();

        Some(MerkleTree {
            root: Self::build_recursive(leaf_nodes),
        })
    }

    /// Recursive tree construction
    fn build_recursive(nodes: Vec<MerkleNode>) -> MerkleNode {
        if nodes.len() == 1 {
            return nodes[0].clone();
        }

        let mut parents = Vec::new();
        let mut i = 0;
        while i < nodes.len() {
            let left = nodes[i].clone();
            let right = if i + 1 < nodes.len() {
                nodes[i + 1].clone()
            } else {
                left.clone() // Duplicate last node if odd
            };

            let combined = [left.hash(), right.hash()].concat();
            let parent_hash = blake2b(&combined).0;

            parents.push(MerkleNode::Internal {
                hash: parent_hash,
                left: Box::new(left),
                right: Box::new(right),
            });

            i += 2;
        }

        Self::build_recursive(parents)
    }

    /// Get root hash
    pub fn root_hash(&self) -> Vec<u8> {
        self.root.hash().clone()
    }

    /// Generate inclusion proof for a leaf
    pub fn prove(&self, leaf_data: &[u8]) -> Option<MerkleProof> {
        let leaf_hash = blake2b(leaf_data).0;
        let mut path = Vec::new();

        if Self::prove_recursive(&self.root, &leaf_hash, &mut path) {
            Some(MerkleProof { leaf_hash, path })
        } else {
            None
        }
    }

    fn prove_recursive(node: &MerkleNode, target: &[u8], path: &mut Vec<(Vec<u8>, bool)>) -> bool {
        match node {
            MerkleNode::Leaf { hash } => hash == target,
            MerkleNode::Internal { left, right, .. } => {
                if Self::prove_recursive(left, target, path) {
                    path.push((right.hash().clone(), true));
                    true
                } else if Self::prove_recursive(right, target, path) {
                    path.push((left.hash().clone(), false));
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Verify a proof
    pub fn verify_proof(proof: &MerkleProof, root_hash: &[u8]) -> bool {
        let mut current = proof.leaf_hash.clone();

        for (sibling, is_right) in &proof.path {
            let combined = if *is_right {
                [sibling.as_slice(), &current].concat()
            } else {
                [®t.as_slice(), sibling.as_slice()].concat()
            };
            current = blake2b(&combined).0;
        }

        current == root_hash
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_txs() -> Vec<Vec<u8>> {
        vec![
            b"tx1: Alice -> Bob 10".to_vec(),
            b"tx2: Bob -> Carol 5".to_vec(),
            b"tx3: Carol -> Dave 3".to_vec(),
        ]
    }

    #[test]
    fn test_merkle_tree_build() {
        let txs = sample_txs();
        let leaves: Vec<&[u8]> = txs.iter().map(|v| v.as_slice()).collect();
        let tree = MerkleTree::build(leaves).unwrap();

        let root = tree.root_hash();
        assert_eq!(root.len(), 64);
        println!("Merkle Root: {}", hex::encode(&root));
    }

    #[test]
    fn test_merkle_proof_and_verify() {
        let txs = sample_txs();
        let leaves: Vec<&[u8]> = txs.iter().map(|v| v.as_slice()).collect();
        let tree = MerkleTree::build(leaves).unwrap();

        let tx_to_prove = &txs[1]; // "tx2"
        let proof = tree.prove(tx_to_prove).unwrap();
        let root_hash = tree.root_hash();

        assert!(MerkleTree::verify_proof(&proof, &root_hash));
    }

    #[test]
    fn test_invalid_proof() {
        let txs = sample_txs();
        let leaves: Vec<&[u8]> = txs.iter().map(|v| v.as_slice()).collect();
        let tree = MerkleTree::build(leaves).unwrap();

        let fake_tx = b"fake transaction";
        let fake_proof = MerkleProof {
            leaf_hash: blake2b(fake_tx).0,
            path: vec![],
        };

        assert!(!MerkleTree::verify_proof(&fake_proof, &tree.root_hash()));
    }

    #[test]
    fn test_single_leaf_tree() {
        let leaves = vec![b"only one tx"];
        let tree = MerkleTree::build(leaves).unwrap();
        let proof = tree.prove(&leaves[0]).unwrap();
        assert!(MerkleTree::verify_proof(&proof, &tree.root_hash()));
    }

    #[test]
    fn test_odd_number_of_leaves() {
        let leaves = vec![b"tx1", b"tx2", b"tx3"];
        let tree = MerkleTree::build(leaves).unwrap();
        assert!(tree.prove(b"tx3").is_some());
    }
}
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
test blockchain::merkle::tests::test_merkle_tree_build ... ok
test blockchain::merkle::tests::test_merkle_proof_and_verify ... ok
test blockchain::merkle::tests::test_invalid_proof ... ok
test blockchain::merkle::tests::test_single_leaf_tree ... ok
test blockchain::merkle::tests::test_odd_number_of_leaves ... ok
test result: ok. 5 passed
```

---

### Step 7: Git Commit

```bash
git add src/blockchain/ Cargo.toml src/lib.rs
git commit -m "Day 5: Full Merkle Tree with root, proof gen & verification (5 tests)"
```

---

### Step 8: Push

```bash
git push origin main
```

---

## Why This Matters

| Feature | Use in Monero |
|-------|-------------|
| `MerkleTree::build()` | Include 1000+ txs in block |
| `prove()` | Light clients verify tx inclusion |
| `verify_proof()` | SPV wallets |
| **Blake2b hashing** | Matches Monero’s hash function |

**This is how Monero proves a transaction is in a block without downloading all.**

---

## Pro Tips

- **Later**: Add `serde` for proof serialization
- **Later**: Use `Vec<Hash>` instead of `Vec<&[u8]>`
- **Later**: Add `get_proof_by_index(i)`

---

## Day 5 Complete!

| Done |
|------|
| `src/blockchain/merkle.rs` |
| Full Merkle tree + proof system |
| 5 robust tests |
| Uses `blake2b` from Day 4 |
| Git commit |

---

## Tomorrow (Day 6): Elliptic Curve Cryptography (ECC)

```bash
cargo add curve25519-dalek
```

We’ll:
- Generate **Curve25519 keypairs**
- Do **scalar multiplication**
- Build `src/crypto/ecc.rs`

---

**Ready?** Say:  
> `Yes, Day 6`

Or ask:
- “Can I store Merkle root in Block struct now?”
- “Add `Hashable` trait?”
- “Visualize tree structure?”

We’re **5/50** — your **privacy blockchain has hashing, signing, and Merkle proofs**
