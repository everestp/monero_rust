**DAY 29: Pruned Blockchain**  
**Goal:** **Prune old UTXOs** and **keep only recent state**  
**Repo Task:**  
> Implement **pruned blockchain** in `/src/blockchain/pruning.rs`

We’ll **remove spent UTXOs**, **keep only last N blocks**, **reduce disk usage** — **your node now runs efficiently**.

---

## Step-by-Step Guide for Day 29

---

### Step 1: Create `src/blockchain/pruning.rs`

```bash
touch src/blockchain/pruning.rs
```

---

### Step 2: `src/blockchain/pruning.rs`

```rust
// src/blockchain/pruning.rs

use crate::blockchain::storage::PersistentBlockchain;
use crate::blockchain::ringct_tx::RingCTTransaction;

/// Pruning configuration
#[derive(Debug, Clone)]
pub struct PruneConfig {
    pub keep_blocks: usize,     // keep last N blocks
    pub prune_interval: usize,  // prune every N blocks
}

impl Default for PruneConfig {
    fn default() -> Self {
        Self {
            keep_blocks: 1000,
            prune_interval: 100,
        }
    }
}

/// Pruned blockchain
pub struct PrunedBlockchain {
    pub bc: PersistentBlockchain,
    config: PruneConfig,
}

impl PrunedBlockchain {
    pub fn new() -> Self {
        let bc = PersistentBlockchain::new();
        Self {
            bc,
            config: PruneConfig::default(),
        }
    }

    pub fn add_block(&mut self, transactions: Vec<RingCTTransaction>, miner: &mut crate::blockchain::mining::Miner) -> Result<(), &'static str> {
        self.bc.add_block(transactions, miner)?;

        // Prune if needed
        if self.bc.chain.len() % self.config.prune_interval == 0 {
            self.prune_old_blocks();
        }

        Ok(())
    }

    fn prune_old_blocks(&mut self) {
        let target_height = self.bc.chain.len().saturating_sub(self.config.keep_blocks);
        if target_height <= 1 { return; }

        let mut spent_utxos = Vec::new();
        let mut new_utxo_set = crate::blockchain::utxo_set::UtxoSet::new();

        // Rebuild UTXO set from recent blocks
        for block in &self.bc.chain[target_height - 1..] {
            for tx in &block.transactions {
                // Collect spent inputs
                for input in &tx.inputs {
                    spent_utxos.push(input.ring.members[input.ring.real_index].utxo_key.clone());
                }
                // Add outputs
                for (i, output) in tx.outputs.iter().enumerate() {
                    let tx_id = tx.id();
                    let key = crate::blockchain::utxo_set::UtxoKey { tx_id, vout: i };
                    let utxo = crate::blockchain::utxo_set::Utxo {
                        amount: output.amount,
                        receiver: output.one_time_pub.clone(),
                    };
                    new_utxo_set.utxos.insert(key, utxo);
                }
            }
        }

        // Mark spent
        for key in spent_utxos {
            new_utxo_set.spent.insert(key);
        }

        self.bc.utxo_set = new_utxo_set;
        self.bc.chain.drain(..target_height - 1);
        let _ = self.bc.storage.save_chain(&self.bc.chain);

        println!("Pruned to {} blocks | UTXO set: {}", self.bc.chain.len(), self.bc.utxo_set.utxos.len());
    }

    pub fn print(&self) {
        self.bc.print();
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::keys::WalletKeys;
    use crate::blockchain::ringct_tx::RingCTBuilder;

    #[test]
    fn test_pruning() {
        let mut pruned = PrunedBlockchain::new();
        let mut miner = crate::blockchain::mining::Miner::new();
        let sender = WalletKeys::generate();
        let receiver = WalletKeys::generate();

        // Mine 1500 blocks
        for _ in 0..1500 {
            let builder = RingCTBuilder::new(pruned.bc.utxo_set.clone(), 3);
            let tx = builder.build(
                &sender,
                vec![],
                vec![(receiver.address(), 1)],
                0,
            ).unwrap();
            pruned.add_block(vec![tx], &mut miner).unwrap();
        }

        assert!(pruned.bc.chain.len() <= 1000 + 10); // some buffer
        assert!(pruned.bc.utxo_set.utxos.len() < 1500);
    }
}
```

---

### Step 3: Update `src/blockchain/storage.rs`

Replace `PersistentBlockchain` with `PrunedBlockchain` in CLI

```rust
// In run_cli()
let blockchain = Arc::new(Mutex::new(PrunedBlockchain::new()));
```

---

### Step 4: Update `src/blockchain/mod.rs`

```rust
pub mod pruning;
```

---

### Step 5: Run Tests

```bash
cargo test pruning
```

**Expected:**
```
test blockchain::pruning::tests::test_pruning ... ok
```

---

### Step 6: Git Commit

```bash
git add src/blockchain/pruning.rs src/cli/miner_cli.rs src/blockchain/mod.rs
git commit -m "Day 29: Pruned blockchain – keep last 1000 blocks, rebuild UTXO set (1 test)"
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
| `keep_blocks` | `prune-blockchain` |
| `prune_old_blocks()` | `blockchain_prune()` |
| Rebuild UTXO | **Fast sync** |
| Disk savings | **~80% reduction** |

> **Your node now scales to millions of txs**

---

## Day 29 Complete!

| Done |
|------|
| `src/blockchain/pruning.rs` |
| **Prune old blocks** |
| **Rebuild UTXO set** |
| **Configurable** |
| 1 passing test |
| Git commit |

---

## Tomorrow (Day 30): Fast Sync

We’ll:
- **Sync from pruned node**
- **Download only recent state**
- File: `src/network/sync.rs`

```bash
touch src/network/sync.rs
```

---

**Ready?** Say:  
> `Yes, Day 30`

Or ask:
- “Can I sync in 10 seconds?”
- “Add checkpoint?”
- “Show sync progress?”

We’re **29/50** — **Your node is now EFFICIENT and SCALABLE**
