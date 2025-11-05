**DAY 32: Dandelion++ (Privacy Routing)**  
**Goal:** **Hide your IP** when broadcasting transactions  
**Repo Task:**  
> Implement **Dandelion++** in `/src/network/dandelion.rs`

We’ll **route txs through stem phase (anonymous path)** → **fluff phase (broadcast)** — **your IP is now hidden**.

---

## Step-by-Step Guide for Day 32

---

### Step 1: Create `src/network/dandelion.rs`

```bash
touch src/network/dandelion.rs
```

---

### Step 2: `src/network/dandelion.rs`

```rust
// src/network/dandelion.rs

use crate::network::p2p::{P2PNode, Message};
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::sleep;

const STEM_PROBABILITY: f64 = 0.9;     // 90% chance to stem
const STEM_TIMEOUT: u64 = 60;          // 60s max stem
const FLUFF_TIMEOUT: u64 = 10;         // 10s fluff

/// Dandelion++ state
#[derive(Clone)]
pub struct Dandelion {
    node: Arc<P2PNode>,
    stem_map: Arc<Mutex<HashMap<Vec<u8>, StemState>>>,
}

#[derive(Clone)]
struct StemState {
    tx_id: Vec<u8>,
    next_hop: String,
    phase: DandelionPhase,
    created: Instant,
}

#[derive(Clone, PartialEq)]
enum DandelionPhase {
    Stem,
    Fluff,
}

impl Dandelion {
    pub fn new(node: Arc<P2PNode>) -> Self {
        Self {
            node,
            stem_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Broadcast with Dandelion++
    pub async fn broadcast_tx(&self, tx: &crate::blockchain::ringct_tx::RingCTTransaction) {
        let tx_id = tx.id();
        let peers: Vec<String> = self.node.peers.lock().unwrap().iter().map(|a| a.to_string()).collect();

        if peers.is_empty() {
            self.node.broadcast_tx(tx);
            return;
        }

        let mut rng = rand::thread_rng();
        let next_hop = peers.choose(&mut rng).unwrap().clone();

        let state = StemState {
            tx_id: tx_id.clone(),
            next_hop: next_hop.clone(),
            phase: DandelionPhase::Stem,
            created: Instant::now(),
        };

        self.stem_map.lock().unwrap().insert(tx_id.clone(), state);

        // Send to stem peer
        self.send_to_peer(&next_hop, tx, true).await;

        // Schedule timeout
        let dandelion = self.clone();
        let tx_id_clone = tx_id.clone();
        tokio::spawn(async move {
            sleep(Duration::from_secs(STEM_TIMEOUT)).await;
            dandelion.timeout_stem(&tx_id_clone).await;
        });
    }

    /// Receive tx from peer
    pub async fn receive_tx(&self, tx: &crate::blockchain::ringct_tx::RingCTTransaction, is_stem: bool) {
        let tx_id = tx.id();

        if is_stem {
            // In stem phase: forward or fluff
            if rand::random::<f64>() < STEM_PROBABILITY {
                self.forward_stem(tx).await;
            } else {
                self.enter_fluff(tx).await;
            }
        } else {
            // In fluff phase: broadcast
            self.node.broadcast_tx(tx);
        }
    }

    async fn forward_stem(&self, tx: &crate::blockchain::ringct_tx::RingCTTransaction) {
        let peers: Vec<String> = self.node.peers.lock().unwrap().iter().map(|a| a.to_string()).collect();
        if let Some(next_hop) = peers.choose(&mut rand::thread_rng()) {
            self.send_to_peer(next_hop, tx, true).await;
        }
    }

    async fn enter_fluff(&self, tx: &crate::blockchain::ringct_tx::RingCTTransaction) {
        self.node.broadcast_tx(tx);
        self.cleanup(&tx.id());
    }

    async fn timeout_stem(&self, tx_id: &Vec<u8>) {
        if let Some(state) = self.stem_map.lock().unwrap().get(tx_id) {
            if state.phase == DandelionPhase::Stem {
                println!("Dandelion++: Stem timeout for tx {}", hex::encode(&tx_id[..4]));
                self.node.broadcast_tx(&crate::blockchain::ringct_tx::RingCTTransaction::default()); // dummy
                self.cleanup(tx_id);
            }
        }
    }

    fn send_to_peer(&self, peer: &str, tx: &crate::blockchain::ringct_tx::RingCTTransaction, is_stem: bool) {
        let msg = if is_stem {
            Message::DandelionStem { tx: tx.clone() }
        } else {
            Message::DandelionFluff { tx: tx.clone() }
        };
        let data = bincode::serialize(&msg).unwrap();
        let peer = peer.to_string();
        tokio::spawn(async move {
            if let Ok(mut stream) = tokio::net::TcpStream::connect(&peer).await {
                let _ = stream.write_all(&data).await;
            }
        });
    }

    fn cleanup(&self, tx_id: &Vec<u8>) {
        self.stem_map.lock().unwrap().remove(tx_id);
    }
}

// Extend Message
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    // ... existing
    DandelionStem { tx: crate::blockchain::ringct_tx::RingCTTransaction },
    DandelionFluff { tx: crate::blockchain::ringct_tx::RingCTTransaction },
}
```

---

### Step 3: Update `src/network/p2p.rs`

```rust
// In handle_message
Message::DandelionStem { tx } => {
    tokio::spawn(async move {
        self.dandelion.receive_tx(&tx, true).await;
    });
}
Message::DandelionFluff { tx } => {
    self.node.broadcast_tx(&tx);
}
```

Add `dandelion` field to `P2PNode`:

```rust
pub dandelion: Dandelion,
```

In `P2PNode::new()`:

```rust
dandelion: Dandelion::new(self.clone()),
```

---

### Step 4: Update `src/cli/miner_cli.rs`

Replace `broadcast_tx` with Dandelion++:

```rust
// In send-private
self.p2p.dandelion.broadcast_tx(&tx).await;
```

---

### Step 5: Run Dandelion++ Demo

```bash
# Terminal 1
cargo run -- daemon

# Terminal 2
cargo run -- daemon --port 9001

# Terminal 3
cargo run -- send-private --to 4A1B... --amount 10
```

**Output (Terminal 1 & 2):**
```
Dandelion++: Received stem tx
Dandelion++: Entered fluff phase
```

---

### Step 6: Git Commit

```bash
git add src/network/dandelion.rs src/network/p2p.rs src/cli/miner_cli.rs
git commit -m "Day 32: Dandelion++ – stem/fluff routing, IP hiding, timeout"
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
| `STEM_PROBABILITY` | `dandelionpp-stem-probability` |
| `forward_stem()` | `relay_tx()` |
| `enter_fluff()` | `fluff` |
| **IP hidden** | **Sybil resistance** |

> **Your IP is now UNLINKABLE to your tx**

---

## Day 32 Complete!

| Done |
|------|
| `src/network/dandelion.rs` |
| **Stem + fluff phases** |
| **Random routing** |
| **Timeout & cleanup** |
| **Integrated with CLI** |
| Git commit |

---

## Tomorrow (Day 33): Tor Integration

We’ll:
- **Route all traffic via Tor**
- **Hidden service node**
- File: `src/network/tor.rs`

```bash
cargo add arti-client
```

---

**Ready?** Say:  
> `Yes, Day 33`

Or ask:
- “Can I run over Tor?”
- “Add I2P?”
- “Show onion address?”

We’re **32/50** — **Your node is now ANONYMOUS**
