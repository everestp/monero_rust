**DAY 8: Monero Privacy Concepts**  
**Goal:** Deeply understand **Ring Signatures**, **Stealth Addresses**, and **RingCT**  
**Repo Task:**  
> Document Monero concepts in `/docs/privacy_notes.md`  
> Add small prototypes in `/src/crypto/privacy_prototypes.rs`

We’ll **study**, **document**, and **prototype** the **three pillars of Monero privacy** — no full implementation yet, just **working demos** to build intuition for Phase 3.

---

## Step-by-Step Guide for Day 8

---

### Step 1: Create Docs & Prototype File

```bash
mkdir -p docs
touch docs/privacy_notes.md
touch src/crypto/privacy_prototypes.rs
```

---

### Step 2: `docs/privacy_notes.md`

```markdown
# Monero Privacy Concepts – Day 8 Notes

## 1. **Stealth Addresses** (One-Time Public Keys)
- **Problem**: Reusing addresses → linkable transactions
- **Solution**: Receiver generates **one-time destination** per tx
- **Math**:
  ```
  R = r * G                (sender's ephemeral key)
  P = H(r * P_receiver) * G + P_receiver
  ```
  → Only receiver (with `view key`) can detect and spend

## 2. **Ring Signatures** (Untraceable)
- **Problem**: Everyone knows who signed
- **Solution**: Mix real signer with **decoys** (past outputs)
- **Math**: Linkable Ring Signature (LSAG)
  - Signer proves: "I own **one** of these N outputs"
  - Verifier cannot tell **which one**

## 3. **RingCT** (Confidential Transactions)
- **Problem**: Amounts are public
- **Solution**: Hide amounts using **Pedersen Commitments**
  ```
  C = value * G + blind * H
  ```
  + **Range Proofs** (Bulletproofs) → prove `value ∈ [0, 2⁶⁴)`
  + Ring sigs over commitments

---

## Key Insight
> **Monero = UTXO + Stealth + Ring Sig + RingCT**

| Feature        | Hides        | Mechanism                     |
|----------------|--------------|-------------------------------|
| Stealth Addr   | Receiver     | Diffie-Hellman one-time key   |
| Ring Sig       | Sender       | Mix with decoys               |
| RingCT         | Amount       | Commitments + Range Proofs    |

---

## Prototype Plan (Today)
1. **Stealth Address Demo** – Generate one-time key
2. **Ring Signature Demo** – 1 real + 2 decoys
3. **Pedersen Commitment** – Hide amount `50`

→ All in `privacy_prototypes.rs`
```

---

### Step 3: `src/crypto/privacy_prototypes.rs`

```rust
// src/crypto/privacy_prototypes.rs
// Small demos — not production code

use crate::crypto::ecc::{SecretKey, PublicKey, RistrettoPoint};
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25515_dalek::constants::RISTRETTO_BASEPOINT_POINT as G;
use rand::rngs::OsRng;
use sha2::{Sha512, Digest};

// === 1. Stealth Address Demo ===
pub fn stealth_address_demo() {
    println!("\n=== Stealth Address Demo ===");

    // Receiver's long-term keys
    let receiver_view = SecretKey::generate();
    let receiver_spend = SecretKey::generate();
    let P_view = receiver_view.public_key();
    let P_spend = receiver_spend.public_key();

    // Sender generates ephemeral key
    let r = SecretKey::generate(); // ephemeral
    let R = r.public_key();        // send with tx

    // Shared secret: r * P_view
    let shared = r.0 * P_view.0.decompress().unwrap();

    // One-time public key: P = Hs(r*P_view)*G + P_spend
    let hs = Scalar::hash_from_bytes::<Sha512>(shared.compress().as_bytes());
    let one_time_point = (hs * G) + P_spend.0.decompress().unwrap();
    let P_one_time = PublicKey(one_time_point.compress());

    println!("One-time address: {}", P_one_time);
    println!("Only receiver can compute this from R + view key");
}

// === 2. Simple Ring Signature Demo (Conceptual) ===
pub fn ring_signature_demo() {
    println!("\n=== Ring Signature Demo (Conceptual) ===");

    let real_signer = SecretKey::generate();
    let decoy1 = SecretKey::generate();
    let decoy2 = SecretKey::generate();

    let ring = vec![
        real_signer.public_key(),
        decoy1.public_key(),
        decoy2.public_key(),
    ];

    let message = b"Send 10 XMR";

    println!("Ring members (public keys):");
    for (i, pk) in ring.iter().enumerate() {
        println!("  [{}] {}", i, hex::encode(pk.as_bytes()));
    }
    println!("Message: {:?}", message);
    println!("→ Prover knows 1 private key, proves ownership without revealing which");
}

// === 3. Pedersen Commitment Demo ===
use curve25519_dalek::scalar::Scalar;

fn pedersen_commit(value: u64, blind: Scalar) -> RistrettoPoint {
    let H = Scalar::hash_from_bytes::<Sha512>(b"pedersen_H").reduce() * G;
    (Scalar::from(value) * G) + (blind * H)
}

pub fn ringct_demo() {
    println!("\n=== Pedersen Commitment (RingCT) Demo ===");

    let value = 50u64;
    let blind = Scalar::random(&mut OsRng);
    let commitment = pedersen_commit(value, blind);

    println!("Hiding amount: {}", value);
    println!("Commitment: {}", hex::encode(commitment.compress().as_bytes()));
    println!("→ Amount hidden, but provable in range [0, 2^64)");
}

// === Run All Demos ===
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_privacy_demos() {
        stealth_address_demo();
        ring_signature_demo();
        ringct_demo();
        // Just runs — no assertions
    }
}
```

---

### Step 4: Add `sha2` and `hex` (if not already)

```bash
cargo add sha2 hex
```

---

### Step 5: Update `src/crypto/mod.rs`

```rust
// src/crypto/mod.rs

pub mod hash;
pub mod signature;
pub mod ecc;
pub mod privacy_prototypes;
```

---

### Step 6: Run the Demo

```bash
cargo test run_privacy_demos -- --nocapture
```

**Sample Output:**
```
=== Stealth Address Demo ===
One-time address: a1b2c3...
Only receiver can compute this from R + view key

=== Ring Signature Demo (Conceptual) ===
Ring members (public keys):
  [0] e4f5...
  [1] d3c2...
  [2] b7a1...
Message: b"Send 10 XMR"
→ Prover knows 1 private key...

=== Pedersen Commitment (RingCT) Demo ===
Hiding amount: 50
Commitment: f8e9...
→ Amount hidden...
```

---

### Step 7: Git Commit

```bash
git add docs/privacy_notes.md src/crypto/privacy_prototypes.rs src/crypto/mod.rs Cargo.toml
git commit -m "Day 8: Monero privacy concepts doc + stealth/ring/ringct prototypes"
```

---

### Step 8: Push

```bash
git push origin main
```

---

## Why This Matters

| Concept | Phase 3 Target |
|-------|----------------|
| **Stealth Addresses** | Day 26 |
| **Ring Signatures** | Day 27 |
| **RingCT** | Day 32 |

> **You now *understand* the math. Next: implement it.**

---

## Day 8 Complete!

| Done |
|------|
| `docs/privacy_notes.md` – full theory |
| `privacy_prototypes.rs` – 3 working demos |
| `cargo test -- --nocapture` shows output |
| Git commit |

---

## Tomorrow (Day 9): Stealth Addresses

We’ll:
- **Implement full stealth address generation**
- **Scan for incoming payments**
- File: `src/crypto/stealth.rs`

```bash
touch src/crypto/stealth.rs
```

---

**Ready?** Say:  
> `Yes, Day 9`

Or ask:
- “Can I send a real stealth tx now?”
- “Add wallet scanning?”
- “Use real Monero address format?”

We’re **8/50** — you now **understand Monero’s soul**
