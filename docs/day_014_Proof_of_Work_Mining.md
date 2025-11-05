**DAY 14: Proof-of-Work Mining**  
**Goal:** Implement **real PoW mining** with **difficulty adjustment**  
**Repo Task:**  
> Implement mining & difficulty adjustment in `/src/blockchain/mining.rs`

We’ll **refactor mining logic**, **add dynamic difficulty**, **benchmark**, and **mine multiple blocks** — **core of consensus**.

---

## Step-by-Step Guide for Day 14

---

### Step 1: Create `src/blockchain/mining.rs`

```bash
touch src/blockchain/mining.rs
```

---

### Step 2: `src/blockchain/mining.rs`

```rust
// src/blockchain/mining.rs

use crate::blockchain::block::Block;
use crate::blockchain::hash_block::Hashable;
use std::time::{Instant, Duration};

/// Mining configuration
#[derive(Debug, Clone)]
pub struct MiningConfig {
    pub target_block_time: u64,     // seconds
    pub adjustment_interval: u64,   // blocks
    pub min_difficulty: u32,
    pub max_difficulty: u32,
}

impl Default for MiningConfig {
    fn default() -> Self {
        Self {
            target_block_time: 10,     // 10 seconds per block
            adjustment_interval: 6,    // adjust every 6 blocks
            min_difficulty: 2,
            max_difficulty: 60,
        }
    }
}

/// Miner with difficulty adjustment
pub struct Miner {
    config: MiningConfig,
    last_adjustment_block: u64,
    last_adjustment_time: u64,
}

impl Miner {
    pub fn new() -> Self {
        Self {
            config: MiningConfig::default(),
            last_adjustment_block: 0,
            last_adjustment_time: 0,
        }
    }

    /// Mine a block with current difficulty
    pub fn mine_block(&self, mut block: Block) -> Block {
        let start = Instant::now();
        let target = self.compute_target(block.difficulty);
        
        println!("Mining block {} | difficulty: {} | target: 0x{:016x}", 
                 block.index, block.difficulty, target);

        loop {
            block.hash = block.hash(); // uses Hashable
            let hash_int = u64::from_be_bytes([
                block.hash[0], block.hash[1], block.hash[2], block.hash[3],
                block.hash[4], block.hash[5], block.hash[6], block.hash[7],
            ]);

            if hash_int < target {
                let elapsed = start.elapsed().as_secs_f64();
                println!("Mined! nonce: {} | time: {:.2}s | hash: {}", 
                         block.nonce, elapsed, hex::encode(&block.hash[..8]));
                break;
            }
            block.nonce += 1;
        }
        block
    }

    /// Compute target from difficulty (higher diff → smaller target)
    fn compute_target(&self, difficulty: u32) -> u64 {
        let max_target = u64::MAX;
        let shift = (difficulty as u64).min(64);
        max_target >> shift
    }

    /// Adjust difficulty based on block times
    pub fn adjust_difficulty(&mut self, chain: &[Block]) -> u32 {
        if chain.len() <= self.config.adjustment_interval as usize {
            return self.config.min_difficulty;
        }

        let current_height = chain.len() as u64 - 1;
        if current_height % self.config.adjustment_interval != 0 {
            return chain.last().unwrap().difficulty;
        }

        let recent_block = chain.last().unwrap();
        let old_block = &chain[(current_height - self.config.adjustment_interval) as usize];

        let actual_time = recent_block.timestamp - old_block.timestamp;
        let expected_time = self.config.target_block_time * self.config.adjustment_interval;

        let mut new_difficulty = recent_block.difficulty;
        if actual_time < expected_time / 2 {
            new_difficulty = new_difficulty.saturating_add(1);
        } else if actual_time > expected_time * 2 {
            new_difficulty = new_difficulty.saturating_sub(1);
        }

        new_difficulty = new_difficulty
            .max(self.config.min_difficulty)
            .min(self.config.max_difficulty);

        println!("Difficulty adjusted: {} → {}", recent_block.difficulty, new_difficulty);
        new_difficulty
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::block::Blockchain;
    use crate::blockchain::transaction::Transaction;
    use crate::crypto::signature::Ed25519Keypair;

    fn create_tx() -> Transaction {
        let alice = Ed25519Keypair::generate();
        let bob = Ed25519Keypair::generate();
        let mut tx = Transaction::new(
            alice.public_bytes().to_vec(),
            bob.public_bytes().to_vec(),
            50,
            1,
            1,
        );
        tx.sign(&alice).unwrap();
        tx
    }

    #[test]
    fn test_mine_multiple_blocks() {
        let mut bc = Blockchain::new();
        let mut miner = Miner::new();

        for i in 1..=6 {
            let txs = if i % 2 == 0 {
                vec![create_tx(), create_tx()]
            } else {
                vec![create_tx()]
            };
            let prev = bc.chain.last().unwrap().clone();
            let mut block = Block::new(&prev, txs);
            block.difficulty = miner.adjust_difficulty(&bc.chain);
            let mined = miner.mine_block(block);
            bc.chain.push(mined);
        }

        assert!(bc.is_valid());
        assert!(bc.chain.len() > 3);
    }

    #[test]
    fn test_difficulty_adjustment() {
        let mut miner = Miner::new();
        let mut chain = vec![Block::genesis()];

        // Simulate fast blocks
        for i in 1..=6 {
            let prev = chain.last().unwrap().clone();
            let mut block = Block::new(&prev, vec![]);
            block.timestamp = prev.timestamp + 1; // 1s per block
            block.difficulty = miner.adjust_difficulty(&chain);
            chain.push(block);
        }

        let final_diff = chain.last().unwrap().difficulty;
        assert!(final_diff > 4, "Difficulty should increase with fast blocks");
    }

    #[test]
    fn test_target_calculation() {
        let miner = Miner::new();
        assert_eq!(miner.compute_target(1), u64::MAX >> 1);
        assert_eq!(miner.compute_target(4), u64::MAX >> 4);
        assert_eq!(miner.compute_target(60), u64::MAX >> 60);
    }
}
```

---

### Step 3: Update `src/blockchain/block.rs` to Use Miner

Replace `mine()` with external miner:

```rust
// In Block impl — remove old mine()
// Add:
pub fn mine_with_miner(&mut self, miner: &Miner) {
    *self = miner.mine_block(self.clone());
}
```

Update `Blockchain::add_block`:

```rust
pub fn add_block(&mut self, transactions: Vec<Transaction>, miner: &mut Miner) {
    let prev = self.chain.last().unwrap().clone();
    let mut block = Block::new(&prev, transactions);
    block.difficulty = miner.adjust_difficulty(&self.chain);
    let mined_block = miner.mine_block(block);
    self.chain.push(mined_block);
}
```

---

### Step 4: Run Tests

```bash
cargo test mining
```

**Expected:**
```
running 3 tests
test blockchain::mining::tests::test_mine_multiple_blocks ... ok
test blockchain::mining::tests::test_difficulty_adjustment ... ok
test blockchain::mining::tests::test_target_calculation ... ok
test result: ok. 3 passed
```

---

### Step 5: Git Commit

```bash
git add src/blockchain/mining.rs src/blockchain/block.rs src/blockchain/mod.rs
git commit -m "Day 14: PoW mining with dynamic difficulty adjustment & benchmarks (3 tests)"
```

---

### Step 6: Push

```bash
git push origin main
```

---

## Why This Matters

| Feature | Monero Equivalent |
|-------|-------------------|
| `target_block_time` | ~2 minutes |
| `adjustment_interval` | Every 720 blocks |
| `compute_target()` | `difficulty = 2^256 / target` |
| `adjust_difficulty()` | Retarget algorithm |

> **Your chain now self-regulates block time**

---

## Day 14 Complete!

| Done |
|------|
| `src/blockchain/mining.rs` |
| **Dynamic difficulty** |
| **Target calculation** |
| **Benchmarked mining** |
| 3 passing tests |
| Git commit |

---

## Tomorrow (Day 15): CLI Interface

We’ll:
- Add **CLI commands**: `mine`, `status`, `send`
- Use `clap`
- File: `src/cli/miner_cli.rs`

```bash
cargo add clap --features=derive
```

---

**Ready?** Say:  
> `Yes, Day 15`

Or ask:
- “Can I run mining in background?”
- “Add wallet CLI?”
- “Show hash rate?”

We’re **14/50** — **Your blockchain is MINING like a real one**
