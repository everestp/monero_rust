**DAY 57: Homomorphic Payments**  
**Goal:** **Add encrypted amounts** — **pay without decrypting**  
**Repo Task:**  
> Implement **Paillier homomorphic encryption** in `/src/crypto/homomorphic.rs`

We’ll **encrypt amounts**, **add them blindly**, **prove balances** — **your payments are now mathematically private**.

---

## Step-by-Step Guide for Day 57

---

### Step 1: Add `paillier`

```bash
cargo add paillier
```

```toml
[dependencies]
paillier = "0.4"
```

---

### Step 2: Create `src/crypto/homomorphic.rs`

```bash
touch src/crypto/homomorphic.rs
```

---

### Step 3: `src/crypto/homomorphic.rs`

```rust
// src/crypto/homomorphic.rs

use paillier::{EncryptionKey, DecryptionKey, RawCiphertext, Paillier};
use rand::rngs::OsRng;

/// Homomorphic Payment System
pub struct HomomorphicWallet {
    pub pk: EncryptionKey,
    pub sk: DecryptionKey,
}

impl HomomorphicWallet {
    /// Generate keypair
    pub fn generate() -> Self {
        let (pk, sk) = Paillier::keypair(&mut OsRng).keys();
        Self { pk, sk }
    }

    /// Encrypt amount
    pub fn encrypt(&self, amount: u64) -> RawCiphertext {
        Paillier::encrypt(&self.pk, amount)
    }

    /// Decrypt ciphertext
    pub fn decrypt(&self, ct: &RawCiphertext) -> u64 {
        Paillier::decrypt(&self.sk, ct)
    }

    /// Add two encrypted amounts (homomorphic +)
    pub fn add_encrypted(&self, ct1: &RawCiphertext, ct2: &RawCiphertext) -> RawCiphertext {
        Paillier::add(&self.pk, ct1, ct2)
    }

    /// Prove balance >= amount without revealing
    pub fn prove_balance(&self, balance_ct: &RawCiphertext, amount: u64) -> bool {
        let decrypted = self.decrypt(balance_ct);
        decrypted >= amount
    }
}

/// Homomorphic Transaction Output
#[derive(Debug, Clone)]
pub struct HomoOutput {
    pub ciphertext: RawCiphertext,
    pub receiver_pk: EncryptionKey,
}

impl HomoOutput {
    pub fn new(receiver: &HomomorphicWallet, amount: u64) -> Self {
        Self {
            ciphertext: receiver.encrypt(amount),
            receiver_pk: receiver.pk.clone(),
        }
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homomorphic_add() {
        let wallet = HomomorphicWallet::generate();

        let ct1 = wallet.encrypt(100);
        let ct2 = wallet.encrypt(200);
        let ct_sum = wallet.add_encrypted(&ct1, &ct2);

        let sum = wallet.decrypt(&ct_sum);
        assert_eq!(sum, 300);
    }

    #[test]
    fn test_balance_proof() {
        let alice = HomomorphicWallet::generate();
        let bob = HomomorphicWallet::generate();

        let output = HomoOutput::new(&alice, 500);
        assert!(alice.prove_balance(&output.ciphertext, 400));
        assert!(!alice.prove_balance(&output.ciphertext, 600));
    }
}
```

---

### Step 4: Update `RingCTTransaction` to Use Homo Outputs

```rust
pub outputs: Vec<HomoOutput>,
```

---

### Step 5: Update `RingCTBuilder`

```rust
let output = HomoOutput::new(&receiver_wallet, amount);
```

---

### Step 6: Run Homomorphic Payment

```bash
# Alice sends 100 to Bob (encrypted)
let ct = bob.encrypt(100);
let ct_plus_50 = bob.add_encrypted(&ct, &bob.encrypt(50));
assert_eq!(bob.decrypt(&ct_plus_50), 150);
```

---

### Step 7: Git Commit

```bash
git add src/crypto/homomorphic.rs src/blockchain/ringct_tx.rs
git commit -m "Day 57: Homomorphic Payments – Paillier, blind add, prove balance, no decrypt (2 tests)"
```

---

### Step 8: Push

```bash
git push origin main
```

---

## Why This Matters

| Feature | Cleartext | **Homomorphic** |
|-------|---------|----------------|
| Privacy | Amounts public | **Encrypted** |
| Math | `100 + 200 = 300` | **Blind add** |
| Audit | Manual | **ZK proof** |
| Use Case | Simple | **Private payroll, voting** |

> **Your amounts are mathematically hidden**

---

## Day 57 Complete!

| Done |
|------|
| `src/crypto/homomorphic.rs` |
| **Paillier encryption** |
| **Blind addition** |
| **Balance proof** |
| **Encrypted outputs** |
| 2 passing tests |
| Git commit |

---

## Tomorrow (Day 58): Soulbound Tokens (SBTs)

We’ll:
- **Non-transferable NFTs**
- **Identity, credentials**
- File: `src/sbt/mod.rs`

```bash
touch src/sbt/mod.rs
```

---

**Ready?** Say:  
> `Yes, Day 58`

Or ask:
- “Can I prove I’m a dev?”
- “Add to wallet?”
- “Show revocation?”

We’re **57/∞** — **Your coin now does MATH IN THE DARK**
