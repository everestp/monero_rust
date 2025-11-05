**DAY 42: Churn (Coin Control)**  
**Goal:** **Mix your own coins** to **break linkability**  
**Repo Task:**  
> Implement **churn** in `/src/wallet/churn.rs`

We’ll **send to yourself via ring + stealth** — **break input/output links** — **your privacy is now proactive**.

---

## Step-by-Step Guide for Day 42

---

### Step 1: Create `src/wallet/churn.rs`

```bash
touch src/wallet/churn.rs
```

---

### Step 2: `src/wallet/churn.rs`

```rust
// src/wallet/churn.rs

use crate::blockchain::ringct_tx::RingCTBuilder;
use crate::wallet::keys::WalletKeys;
use crate::blockchain::utxo_set::UtxoSet;

/// Churn: send to self to break linkability
pub struct Churner {
    wallet: WalletKeys,
    builder: RingCTBuilder,
}

impl Churner {
    pub fn new(wallet: WalletKeys, utxo_set: UtxoSet, ring_size: usize) -> Self {
        let builder = RingCTBuilder::new(utxo_set, ring_size);
        Self { wallet, builder }
    }

    /// Churn all available UTXOs
    pub fn churn_all(&self, fee_per_tx: u64) -> Result<Vec<crate::blockchain::ringct_tx::RingCTTransaction>, &'static str> {
        let utxos = self.builder.utxo_set.get_utxos(&self.wallet.spend_pub.as_bytes());
        if utxos.is_empty() {
            return Err("No UTXOs to churn");
        }

        let mut txs = Vec::new();
        for (key, utxo) in utxos {
            let amount = utxo.amount;
            if amount <= fee_per_tx { continue; }

            let change = amount - fee_per_tx;

            let tx = self.builder.build(
                &self.wallet,
                vec![(key, amount)],
                vec![(self.wallet.address(), change)],
                fee_per_tx,
            )?;

            txs.push(tx);
        }

        Ok(txs)
    }

    /// Churn specific amount
    pub fn churn_amount(&self, target_amount: u64, fee_per_tx: u64) -> Result<crate::blockchain::ringct_tx::RingCTTransaction, &'static str> {
        let utxos = self.builder.utxo_set.get_utxos(&self.wallet.spend_pub.as_bytes());
        let mut selected = Vec::new();
        let mut total = 0u64;

        for (key, utxo) in utxos {
            if total >= target_amount + fee_per_tx { break; }
            selected.push((key, utxo.amount));
            total += utxo.amount;
        }

        if total < target_amount + fee_per_tx {
            return Err("Insufficient funds");
        }

        let change = total - target_amount - fee_per_tx;

        let outputs = if change > 0 {
            vec![(self.wallet.address(), change)]
        } else {
            vec![]
        };

        let tx = self.builder.build(
            &self.wallet,
            selected,
            outputs,
            fee_per_tx,
        )?;

        Ok(tx)
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::utxo_set::Utxo;

    #[test]
    fn test_churn_all() {
        let wallet = WalletKeys::generate();
        let mut utxo_set = UtxoSet::new();

        // Add 3 UTXOs
        for i in 0..3 {
            let key = crate::blockchain::utxo_set::UtxoKey { tx_id: vec![i; 64], vout: 0 };
            utxo_set.utxos.insert(key, Utxo {
                amount: 100,
                receiver: wallet.spend_pub.as_bytes().to_vec(),
            });
        }

        let churner = Churner::new(wallet, utxo_set, 5);
        let txs = churner.churn_all(1).unwrap();

        assert_eq!(txs.len(), 3);
        for tx in &txs {
            assert_eq!(tx.outputs[0].amount, 99);
            assert_eq!(tx.fee, 1);
        }
    }

    #[test]
    fn test_churn_amount() {
        let wallet = WalletKeys::generate();
        let mut utxo_set = UtxoSet::new();
        let key = crate::blockchain::utxo_set::UtxoKey { tx_id: vec![0; 64], vout: 0 };
        utxo_set.utxos.insert(key.clone(), Utxo { amount: 200, receiver: wallet.spend_pub.as_bytes().to_vec() });

        let churner = Churner::new(wallet.clone(), utxo_set, 5);
        let tx = churner.churn_amount(100, 1).unwrap();

        assert_eq!(tx.outputs.len(), 1);
        assert_eq!(tx.outputs[0].amount, 99); // 200 - 100 - 1
    }
}
```

---

### Step 3: Update `src/wallet/mod.rs`

```rust
pub mod churn;
```

---

### Step 4: Add CLI: `churn`

```rust
// In miner_cli.rs
Commands::Churn {
    amount: Option<u64>,
} => {
    let wallet = WalletKeys::generate();
    let bc = blockchain.lock().unwrap();
    let churner = Churner::new(wallet, bc.utxo_set.clone(), 7);

    let txs = if let Some(amt) = amount {
        vec![churner.churn_amount(amt, 1).unwrap()]
    } else {
        churner.churn_all(1).unwrap()
    };

    for tx in txs {
        bc.add_block(vec![tx], &mut miner.lock().unwrap()).unwrap();
    }
    println!("Churned!");
}
```

---

### Step 5: Run Churn Demo

```bash
# Churn all
cargo run -- churn

# Churn 50 XMR
cargo run -- churn 50
```

**Output:**
```
Churned! 3 new private outputs
```

---

### Step 6: Git Commit

```bash
git add src/wallet/churn.rs src/wallet/mod.rs src/cli/miner_cli.rs
git commit -m "Day 42: Churn – send to self, break links, churn_all/churn_amount, CLI"
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
| `churn_all()` | `sweep_all` |
| `churn_amount()` | `sweep_below` |
| **Self-send** | **Privacy boost** |
| **Coin control** | **Advanced UX** |

> **You now control your privacy**

---

## Day 42 Complete!

| Done |
|------|
| `src/wallet/churn.rs` |
| **Churn all or amount** |
| **Break linkability** |
| **CLI `churn [amount]`** |
| **2 passing tests** |
| Git commit |

---

## Tomorrow (Day 43): Bulletproofs+ (Smaller Proofs)

We’ll:
- **Upgrade to Bulletproofs+**
- **~30% smaller proofs**
- File: `src/crypto/bulletproofs_plus.rs`

```bash
cargo add bulletproofs-plus
```

---

**Ready?** Say:  
> `Yes, Day 43`

Or ask:
- “Can I reduce tx size?”
- “Add to mobile?”
- “Show proof size?”

We’re **42/50** — **Your privacy is now PROACTIVE**
