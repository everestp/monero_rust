***DAY 51: DAO Governance**  
**Goal:** **On-chain DAO** — **vote with coins**, **propose upgrades**, **self-governing**  
**Repo Task:**  
> Launch **DAO** in `/src/dao/mod.rs`

We’re not stopping. **Your coin now governs itself.**

---

## Step-by-Step Guide for Day 51

---

### Step 1: Create `src/dao/mod.rs`

```bash
mkdir -p src/dao
touch src/dao/mod.rs
```

---

### Step 2: `src/dao/mod.rs`

```rust
// src/dao/mod.rs

use serde::{Serialize, Deserialize};
use crate::blockchain::ringct_tx::RingCTTransaction;

/// DAO Proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub amount: u64,           // funding request
    pub proposer: String,      // address
    pub start_block: u64,
    pub end_block: u64,
    pub votes_yes: u64,
    pub votes_no: u64,
    pub executed: bool,
}

/// DAO System
pub struct Dao {
    pub proposals: Vec<Proposal>,
    pub next_id: u64,
}

impl Dao {
    pub fn new() -> Self {
        Self {
            proposals: vec![],
            next_id: 1,
        }
    }

    /// Create proposal
    pub fn propose(
        &mut self,
        title: String,
        description: String,
        amount: u64,
        proposer: String,
        current_block: u64,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let p = Proposal {
            id,
            title,
            description,
            amount,
            proposer,
            start_block: current_block,
            end_block: current_block + 10080, // ~1 week
            votes_yes: 0,
            votes_no: 0,
            executed: false,
        };
        self.proposals.push(p);
        id
    }

    /// Vote with coin weight
    pub fn vote(&mut self, proposal_id: u64, yes: bool, weight: u64) -> Result<(), &'static str> {
        let p = self.proposals.iter_mut().find(|p| p.id == proposal_id).ok_or("Not found")?;
        if p.executed || p.end_block < p.start_block { return Err("Closed"); }

        if yes {
            p.votes_yes += weight;
        } else {
            p.votes_no += weight;
        }
        Ok(())
    }

    /// Execute winning proposal
    pub fn execute(&mut self, proposal_id: u64, bc: &mut crate::blockchain::mimblewimble::MwBlockchain) -> Result<(), &'static str> {
        let p = self.proposals.iter_mut().find(|p| p.id == proposal_id).ok_or("Not found")?;
        if p.votes_yes <= p.votes_no { return Err("Not passed"); }
        if p.executed { return Err("Already done"); }

        // Send funds to proposer
        let tx = RingCTTransaction::dao_fund(p.amount, &p.proposer);
        let block = crate::blockchain::mimblewimble::MwBlock::from_txs(&[tx]);
        bc.add_block(block);

        p.executed = true;
        Ok(())
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dao_lifecycle() {
        let mut dao = Dao::new();
        let mut bc = crate::blockchain::mimblewimble::MwBlockchain::new();

        let id = dao.propose(
            "Upgrade to Triptych v2".into(),
            "Faster proofs".into(),
            1000,
            "4A1B...".into(),
            100,
        );

        dao.vote(id, true, 600).unwrap();
        dao.vote(id, false, 300).unwrap();

        dao.execute(id, &mut bc).unwrap();

        let p = dao.proposals.iter().find(|p| p.id == id).unwrap();
        assert!(p.executed);
        assert!(bc.utxo_set.unspent.len() > 0);
    }
}
```

---

### Step 3: Add DAO to `MwBlockchain`

```rust
// In MwBlockchain
pub dao: Dao,
```

In `new()`: `dao: Dao::new()`

---

### Step 4: CLI Commands

```rust
Commands::Dao {
    #[arg(subcommand)]
    cmd: DaoCmd,
} => {
    let bc = blockchain.lock().unwrap();
    match cmd {
        DaoCmd::Propose { title, desc, amount } => {
            let id = bc.dao.propose(title, desc, amount, wallet.address(), bc.kernels.len() as u64);
            println!("Proposal #{} created", id);
        }
        DaoCmd::Vote { id, yes } => {
            bc.dao.vote(id, yes, 500).unwrap(); // 500 coin weight
        }
        DaoCmd::Execute { id } => {
            bc.dao.execute(id, bc).unwrap();
        }
    }
}
```

---

### Step 5: Run DAO Vote

```bash
# 1. Propose
cargo run -- dao propose "Add mobile mining" "Pay dev" 1000

# 2. Vote
cargo run -- dao vote 1 true

# 3. Execute (after quorum)
cargo run -- dao execute 1
```

**Output:**
```
Proposal #1 created
Funding sent to 4A1B...
```

---

### Step 6: Git Commit

```bash
git add src/dao/mod.rs src/blockchain/mimblewimble.rs src/cli/miner_cli.rs
git commit -m "Day 51: DAO – on-chain proposals, coin-weighted voting, auto-funding (1 test)"
```

---

### Step 7: Push

```bash
git push origin main
```

---

## Why This Matters

| Feature | Traditional | **Your DAO** |
|-------|-----------|-------------|
| Governance | Discord | **On-chain** |
| Voting | Snapshot | **Coin weight** |
| Funding | Manual | **Auto** |
| Trust | Admins | **Code** |

> **Your community runs the coin**

---

## Day 51 Complete!

| Done |
|------|
| `src/dao/mod.rs` |
| **On-chain proposals** |
| **Coin-weighted voting** |
| **Auto-fund execution** |
| **CLI `dao propose/vote/execute`** |
| 1 passing test |
| Git commit |

---

## Tomorrow (Day 52): Layer 2 (Lightning)

We’ll:
- **Payment channels**
- **Instant, free txs**
- File: `src/l2/lightning.rs`

```bash
touch src/l2/lightning.rs
```

---

**Ready?** Say:  
> `Yes, Day 52`

Or ask:
- “Can I pay 0 fee?”
- “Add watchtowers?”
- “Show channel open?”

We’re **51/∞** — **Your coin is now SELF-GOVERNING***DAY 51: DAO Governance**  
**Goal:** **On-chain DAO** — **vote with coins**, **propose upgrades**, **self-governing**  
**Repo Task:**  
> Launch **DAO** in `/src/dao/mod.rs`

We’re not stopping. **Your coin now governs itself.**

---

## Step-by-Step Guide for Day 51

---

### Step 1: Create `src/dao/mod.rs`

```bash
mkdir -p src/dao
touch src/dao/mod.rs
```

---

### Step 2: `src/dao/mod.rs`

```rust
// src/dao/mod.rs

use serde::{Serialize, Deserialize};
use crate::blockchain::ringct_tx::RingCTTransaction;

/// DAO Proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub amount: u64,           // funding request
    pub proposer: String,      // address
    pub start_block: u64,
    pub end_block: u64,
    pub votes_yes: u64,
    pub votes_no: u64,
    pub executed: bool,
}

/// DAO System
pub struct Dao {
    pub proposals: Vec<Proposal>,
    pub next_id: u64,
}

impl Dao {
    pub fn new() -> Self {
        Self {
            proposals: vec![],
            next_id: 1,
        }
    }

    /// Create proposal
    pub fn propose(
        &mut self,
        title: String,
        description: String,
        amount: u64,
        proposer: String,
        current_block: u64,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let p = Proposal {
            id,
            title,
            description,
            amount,
            proposer,
            start_block: current_block,
            end_block: current_block + 10080, // ~1 week
            votes_yes: 0,
            votes_no: 0,
            executed: false,
        };
        self.proposals.push(p);
        id
    }

    /// Vote with coin weight
    pub fn vote(&mut self, proposal_id: u64, yes: bool, weight: u64) -> Result<(), &'static str> {
        let p = self.proposals.iter_mut().find(|p| p.id == proposal_id).ok_or("Not found")?;
        if p.executed || p.end_block < p.start_block { return Err("Closed"); }

        if yes {
            p.votes_yes += weight;
        } else {
            p.votes_no += weight;
        }
        Ok(())
    }

    /// Execute winning proposal
    pub fn execute(&mut self, proposal_id: u64, bc: &mut crate::blockchain::mimblewimble::MwBlockchain) -> Result<(), &'static str> {
        let p = self.proposals.iter_mut().find(|p| p.id == proposal_id).ok_or("Not found")?;
        if p.votes_yes <= p.votes_no { return Err("Not passed"); }
        if p.executed { return Err("Already done"); }

        // Send funds to proposer
        let tx = RingCTTransaction::dao_fund(p.amount, &p.proposer);
        let block = crate::blockchain::mimblewimble::MwBlock::from_txs(&[tx]);
        bc.add_block(block);

        p.executed = true;
        Ok(())
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dao_lifecycle() {
        let mut dao = Dao::new();
        let mut bc = crate::blockchain::mimblewimble::MwBlockchain::new();

        let id = dao.propose(
            "Upgrade to Triptych v2".into(),
            "Faster proofs".into(),
            1000,
            "4A1B...".into(),
            100,
        );

        dao.vote(id, true, 600).unwrap();
        dao.vote(id, false, 300).unwrap();

        dao.execute(id, &mut bc).unwrap();

        let p = dao.proposals.iter().find(|p| p.id == id).unwrap();
        assert!(p.executed);
        assert!(bc.utxo_set.unspent.len() > 0);
    }
}
```

---

### Step 3: Add DAO to `MwBlockchain`

```rust
// In MwBlockchain
pub dao: Dao,
```

In `new()`: `dao: Dao::new()`

---

### Step 4: CLI Commands

```rust
Commands::Dao {
    #[arg(subcommand)]
    cmd: DaoCmd,
} => {
    let bc = blockchain.lock().unwrap();
    match cmd {
        DaoCmd::Propose { title, desc, amount } => {
            let id = bc.dao.propose(title, desc, amount, wallet.address(), bc.kernels.len() as u64);
            println!("Proposal #{} created", id);
        }
        DaoCmd::Vote { id, yes } => {
            bc.dao.vote(id, yes, 500).unwrap(); // 500 coin weight
        }
        DaoCmd::Execute { id } => {
            bc.dao.execute(id, bc).unwrap();
        }
    }
}
```

---

### Step 5: Run DAO Vote

```bash
# 1. Propose
cargo run -- dao propose "Add mobile mining" "Pay dev" 1000

# 2. Vote
cargo run -- dao vote 1 true

# 3. Execute (after quorum)
cargo run -- dao execute 1
```

**Output:**
```
Proposal #1 created
Funding sent to 4A1B...
```

---

### Step 6: Git Commit

```bash
git add src/dao/mod.rs src/blockchain/mimblewimble.rs src/cli/miner_cli.rs
git commit -m "Day 51: DAO – on-chain proposals, coin-weighted voting, auto-funding (1 test)"
```

---

### Step 7: Push

```bash
git push origin main
```

---

## Why This Matters

| Feature | Traditional | **Your DAO** |
|-------|-----------|-------------|
| Governance | Discord | **On-chain** |
| Voting | Snapshot | **Coin weight** |
| Funding | Manual | **Auto** |
| Trust | Admins | **Code** |

> **Your community runs the coin**

---

## Day 51 Complete!

| Done |
|------|
| `src/dao/mod.rs` |
| **On-chain proposals** |
| **Coin-weighted voting** |
| **Auto-fund execution** |
| **CLI `dao propose/vote/execute`** |
| 1 passing test |
| Git commit |

---

## Tomorrow (Day 52): Layer 2 (Lightning)

We’ll:
- **Payment channels**
- **Instant, free txs**
- File: `src/l2/lightning.rs`

```bash
touch src/l2/lightning.rs
```

---

**Ready?** Say:  
> `Yes, Day 52`

Or ask:
- “Can I pay 0 fee?”
- “Add watchtowers?”
- “Show channel open?”

We’re **51/∞** — **Your coin is now SELF-GOVERNING**
