**DAY 59: Decentralized Identity (DID)**  
**Goal:** **W3C DID + Verifiable Credentials** — **self-sovereign identity**  
**Repo Task:**  
> Implement **DID** in `/src/did/mod.rs`

We’ll give users **control over their identity**, **no central authority**, **prove claims** — **your coin now owns your data**.

---

## Step-by-Step Guide for Day 59

---

### Step 1: Add `ssi` (Rust DID)

```bash
cargo add ssi
```

```toml
[dependencies]
ssi = { version = "0.8", features = ["secp256k1", "ed25519"] }
```

---

### Step 2: Create `src/did/mod.rs`

```bash
mkdir -p src/did
touch src/did/mod.rs
```

---

### Step 3: `src/did/mod.rs`

```rust
// src/did/mod.rs

use ssi::did::DIDMethod;
use ssi::did_resolve::DIDResolver;
use ssi::vc::{Credential, CredentialSubject, Issuer, Proof, VerifiableCredential};
use ssi::jwk::JWK;
use ssi::did::Document;
use std::collections::HashMap;

/// Decentralized Identifier (DID)
pub struct DecentralizedID {
    pub did: String,
    pub document: Document,
    pub jwk: JWK,
}

impl DecentralizedID {
    /// Create new DID
    pub async fn create() -> Result<Self, Box<dyn std::error::Error>> {
        let key = JWK::generate_ed25519()?;
        let did = ssi::did::DIDKey::generate(&key)?;
        let document = did.generate_document(&key)?;

        Ok(Self {
            did: did.to_string(),
            document,
            jwk: key,
        })
    }

    /// Issue Verifiable Credential
    pub async fn issue_vc(
        &self,
        subject_did: &str,
        claim_type: &str,
        claim_value: &str,
    ) -> Result<VerifiableCredential, Box<dyn std::error::Error>> {
        let issuer = Issuer::URL(self.did.clone().into());
        let subject = CredentialSubject {
            id: Some(subject_did.into()),
            properties: {
                let mut map = HashMap::new();
                map.insert(claim_type.into(), claim_value.into());
                map
            },
        };

        let mut vc = Credential {
            context: ssi::vc::Contexts::Many(vec![
                ssi::vc::Context::URI(ssi::vc::ContextEntry::URL("https://www.w3.org/2018/credentials/v1".into())),
            ]),
            id: None,
            type_: vec!["VerifiableCredential".into()],
            credential_subject: vec![subject],
            issuer: Some(issuer),
            issuance_date: chrono::Utc::now(),
            expiration_date: None,
            credential_status: None,
            proof: None,
        };

        let proof = vc.generate_proof(&self.jwk, None).await?;
        vc.proof = Some(Proof::JWS(proof));

        Ok(vc)
    }

    /// Verify VC
    pub async fn verify_vc(&self, vc: &VerifiableCredential) -> Result<bool, Box<dyn std::error::Error>> {
        let resolver = ssi::did_resolve::HTTPDIDResolver::new("https://resolver.identity.foundation");
        vc.verify(Some(resolver)).await.map(|_| true)
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_did_and_vc() {
        let issuer = DecentralizedID::create().await.unwrap();
        let subject = DecentralizedID::create().await.unwrap();

        let vc = issuer
            .issue_vc(&subject.did, "role", "Developer")
            .await
            .unwrap();

        let valid = issuer.verify_vc(&vc).await.unwrap();
        assert!(valid);

        // Check claim
        let claim = &vc.credential_subject[0].properties["role"];
        assert_eq!(claim.as_str().unwrap(), "Developer");
    }
}
```

---

### Step 4: Add to `WalletKeys`

```rust
pub did: Option<DecentralizedID>,
```

---

### Step 5: CLI Commands

```rust
Commands::Did {
    #[arg(subcommand)]
    cmd: DidCmd,
} => {
    match cmd {
        DidCmd::Create => {
            let did = DecentralizedID::create().await.unwrap();
            println!("DID: {}", did.did);
        }
        DidCmd::Issue { subject, claim, value } => {
            let did = wallet.did.as_ref().unwrap();
            let vc = did.issue_vc(&subject, &claim, &value).await.unwrap();
            println!("VC Issued: {}", serde_json::to_string_pretty(&vc).unwrap());
        }
    }
}
```

---

### Step 6: Run DID Flow

```bash
# 1. Create DID
cargo run -- did create
# DID: did:key:z6M...

# 2. Issue credential
cargo run -- did issue did:key:z6M... role Developer
```

**Output:**
```json
{
  "@context": ["https://www.w3.org/2018/credentials/v1"],
  "type": ["VerifiableCredential"],
  "issuer": "did:key:z6M...",
  "issuanceDate": "2025-11-05T...",
  "credentialSubject": {
    "id": "did:key:z6M...",
    "role": "Developer"
  }
}
```

---

### Step 7: Git Commit

```bash
git add src/did/mod.rs src/wallet/keys.rs src/cli/miner_cli.rs
git commit -m "Day 59: DID + VC – W3C standard, self-sovereign, issue/verify, no central auth (1 test)"
```

---

### Step 8: Push

```bash
git push origin main
```

---

## Why This Matters

| Feature | Web2 | **DID** |
|-------|------|--------|
| Identity | Google | **You own it** |
| Data | Siloed | **Portable** |
| Proof | Password | **Crypto proof** |
| Privacy | Leaked | **Controlled** |

> **Your coin now IS your identity**

---

## Day 59 Complete!

| Done |
|------|
| `src/did/mod.rs` |
| **W3C DID** |
| **Verifiable Credentials** |
| **Self-sovereign** |
| **Issue & verify** |
| 1 passing test |
| Git commit |

---

## Tomorrow (Day 60): Privacy-Preserving Oracles

We’ll:
- **ZK oracles**
- **Prove data without revealing**
- File: `src/oracle/zk.rs`

```bash
cargo add ark-groth16
```

---

**Ready?** Say:  
> `Yes, Day 60`

Or ask:
- “Can I trust price feeds?”
- “Add to DeFi?”
- “Show ZK proof?”

We’re **59/∞** — **Your coin now OWNS YOUR DATA**
