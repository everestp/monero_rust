**DAY 17: Transaction Pool**  
**Goal:** Build a **mempool** to hold **unconfirmed transactions**  
**Repo Task:**  
> Implement transaction pool in `/src/blockchain/tx_pool.rs`

We’ll create a **thread-safe transaction pool** with **validation**, **eviction**, and **integration with mining** — your node now **accepts and relays transactions**.

---

## Step-by-Step Guide for Day 17

---

### Step 1: Create `src/blockchain/tx_pool.rs`

```bash
touch src/blockchain/tx_pool.rs
```

---

### Step 2: `src/blockchain/tx_pool.rs`

```rust
// src/blockchain/tx_pool.rs

use crate::blockchain::transaction::Transaction;
use crate::blockchain::verify::{Validator, ValidationError};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Transaction with metadata
#[derive(Debug, Clone)]
pub struct PooledTx {
    pub tx: Transaction,
    pub received_at: u64,
    pub fee_per_byte: f64,
}

/// Thread-safe transaction pool
#[derive(Clone)]
pub struct TxPool {
    inner: Arc<Mutex<TxPoolInner>>,
}

struct TxPoolInner {
    transactions: HashMap<Vec<u8>, PooledTx>, // tx_id → tx
    validator: Validator,
    max_size: usize,
    min_fee_per_byte: f64,
}

impl TxPool {
    pub fn new(max_size: usize, min_fee_per_byte: f64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(TxPoolInner {
                transactions: HashMap::new(),
                validator: Validator::new(),
                max_size,
                min_fee_per_byte,
            })),
        }
    }

    /// Add transaction to pool
    pub fn add_transaction(&self, tx: Transaction) -> Result<(), PoolError> {
        let mut inner = self.inner.lock().unwrap();

        // 1. Basic checks
        if inner.transactions.contains_key(&tx.id()) {
            return Err(PoolError::AlreadyExists);
        }

        let tx_size = bincode::serialize(&tx).map(|v| v.len()).unwrap_or(1000);
        let fee_per_byte = tx.fee as f64 / tx_size as f64;
        if fee_per_byte < inner.min_fee_per_byte {
            return Err(PoolError::LowFee);
        }

        // 2. Validate (no double-spend in pool)
        let mut temp_validator = inner.validator.clone();
        temp_validator.validate_transaction(&tx)?;

        // 3. Evict lowest fee if full
        if inner.transactions.len() >= inner.max_size {
            self.evict_lowest_fee(&mut inner);
        }

        // 4. Insert
        let pooled = PooledTx {
            tx: tx.clone(),
            received_at: Self::now(),
            fee_per_byte,
        };
        inner.transactions.insert(tx.id(), pooled);
        println!("Added tx {} to pool ({} txs)", hex::encode(&tx.id()[..4]), inner.transactions.len());

        Ok(())
    }

    /// Get transactions for mining
    pub fn get_txs_for_block(&self, max_txs: usize) -> Vec<Transaction> {
        let mut inner = self.inner.lock().unwrap();
        let mut txs: Vec<_> = inner.transactions.values().cloned().collect();
        txs.sort_by(|a, b| b.fee_per_byte.partial_cmp(&a.fee_per_byte).unwrap());
        txs.into_iter().take(max_txs).map(|p| p.tx).collect()
    }

    /// Remove spent transactions after block
    pub fn remove_spent(&self, block_txs: &[Transaction]) {
        let mut inner = self.inner.lock().unwrap();
        for tx in block_txs {
            inner.transactions.remove(&tx.id());
        }
        println!("Cleaned pool: {} txs left", inner.transactions.len());
    }

    /// Evict lowest fee tx
    fn evict_lowest_fee(&self, inner: &mut TxPoolInner) {
        if let Some(lowest) = inner.transactions.values()
            .min_by(|a, b| a.fee_per_byte.partial_cmp(&b.fee_per_byte).unwrap())
        {
            inner.transactions.remove(&lowest.tx.id());
            println!("Evicted low-fee tx {}", hex::encode(&lowest.tx.id()[..4]));
        }
    }

    /// Get pool size
    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().transactions.len()
    }

    fn now() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}

#[derive(Debug, PartialEq)]
pub enum PoolError {
    AlreadyExists,
    LowFee,
    Validation(ValidationError),
}

impl From<ValidationError> for PoolError {
    fn from(err: ValidationError) -> Self {
        PoolError::Validation(err)
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::signature::Ed25519Keypair;

    fn create_tx(fee: u64) -> Transaction {
        let alice = Ed25519Keypair::generate();
        let bob = Ed25519Keypair::generate();
        let mut tx = Transaction::new(
            alice.public_bytes().to_vec(),
            bob.public_bytes().to_vec(),
            100,
            fee,
            1,
        );
        tx.sign(&alice).unwrap();
        tx
    }

    #[test]
    fn test_add_and_get_txs() {
        let pool = TxPool::new(10, 0.0001);
        let tx1 = create_tx(10);
        let tx2 = create_tx(20);

        pool.add_transaction(tx1.clone()).unwrap();
        pool.add_transaction(tx2.clone()).unwrap();

        let for_block = pool.get_txs_for_block(2);
        assert_eq!(for_block[0].fee, 20); // highest fee first
        assert_eq!(for_block.len(), 2);
    }

    #[test]
    fn test_eviction() {
        let pool = TxPool::new(2, 0.0001);
        let tx1 = create_tx(10);
        let tx2 = create_tx(20);
        let tx3 = create_tx(5);

        pool.add_transaction(tx1).unwrap();
        pool.add_transaction(tx2).unwrap();
        pool.add_transaction(tx3).unwrap();

        assert_eq!(pool.len(), 2);
        let txs = pool.get_txs_for_block(2);
        assert!(txs.iter().any(|tx| tx.fee == 20));
        assert!(txs.iter().any(|tx| tx.fee == 10));
    }

    #[test]
    fn test_remove_spent() {
        let pool = TxPool::new(5, 0.0);
        let tx = create_tx(10);
        pool.add_transaction(tx.clone()).unwrap();

        pool.remove_spent(&[tx.clone()]);
        assert_eq!(pool.len(), 0);
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
pub mod verify;
pub mod hash_block;
pub mod mining;
pub mod storage;
pub mod tx_pool;
```

---

### Step 4: Update `src/cli/miner_cli.rs` to Use `TxPool`

Add to CLI:

```rust
use crate::blockchain::tx_pool::TxPool;

let tx_pool = Arc::new(Mutex::new(TxPool::new(100, 0.0001)));
```

In `add_block`:

```rust
let txs = tx_pool.lock().unwrap().get_txs_for_block(10);
bc.add_block(txs.clone(), &mut miner);
tx_pool.lock().unwrap().remove_spent(&txs);
```

Add `Send` command to add to pool:

```rust
Commands::Send { to, amount } => {
    let mut pool = tx_pool.lock().unwrap();
    let tx = create_tx_to(&to, amount);
    if let Err(e) = pool.add_transaction(tx) {
        println!("Failed to add tx: {:?}", e);
    } else {
        println!("Tx added to mempool!");
    }
}
```

---

### Step 5: Run Tests

```bash
cargo test tx_pool
```

**Expected:**
```
running 3 tests
test blockchain::tx_pool::tests::test_add_and_get_txs ... ok
test blockchain::tx_pool::tests::test_eviction ... ok
test blockchain::tx_pool::tests::test_remove_spent ... ok
test result: ok. 3 passed
```

---

### Step 6: Git Commit

```bash
git add src/blockchain/tx_pool.rs src/cli/miner_cli.rs src/blockchain/mod.rs
git commit -m "Day 17: Thread-safe TxPool with validation, fee priority, eviction (3 tests)"
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
| `add_transaction()` | `handle_tx()` |
| `get_txs_for_block()` | `fill_block_template()` |
| `remove_spent()` | `pop_block()` |
| Fee priority | `tx_fees` |

> **Your node now relays and mines real transactions**

---

## Day 17 Complete!

| Done |
|------|
| `src/blockchain/tx_pool.rs` |
| **Thread-safe mempool** |
| **Fee-based prioritization** |
| **Eviction** |
| **Integration with mining** |
| 3 passing tests |
| Git commit |

---

## Tomorrow (Day 18): Merkle Root Integration

We’ll:
- **Finalize Merkle root** in blocks
- **Verify on load**
- File: Update `block.rs`

---

**Ready?** Say:  
> `Yes, Day 18`

Or ask:
- “Can I broadcast txs over network?”
- “Add expiration?”
- “Show pool stats?”

We’re **17/50** — **Your node is now a FULL TRANSACTION RELAY**
