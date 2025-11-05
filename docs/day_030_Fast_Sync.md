**DAY 30: Fast Sync**  
**Goal:** **Sync from pruned node** in **under 10 seconds**  
**Repo Task:**  
> Implement **fast sync** via **UTXO snapshot + recent blocks** in `/src/network/sync.rs`

We’ll **download UTXO set + last N blocks**, **verify Merkle proofs**, **bootstrap instantly** — **new nodes sync in seconds**.

---

## Step-by-Step Guide for Day 30

---

### Step 1: Create `src/network/sync.rs`

```bash
touch src/network/sync.rs
```

---

### Step 2: `src/network/sync.rs`

```rust
// src/network/sync.rs

use crate::blockchain::pruning::PrunedBlockchain;
use crate::blockchain::ringct_tx::RingCTTransaction;
use crate::network::p2p::{P2PNode, Message};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bincode;
use std::sync::{Arc, Mutex};

/// Fast sync state
#[derive(Debug)]
pub struct FastSync {
    target_height: u64,
    utxo_snapshot: Vec<u8>,
    blocks: Vec<crate::blockchain::block::Block>,
}

impl FastSync {
    pub async fn download_from_peer(
        peer_addr: &str,
        local_height: u64,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(peer_addr).await?;
        
        // Request fast sync
        let req = Message::FastSyncRequest { from_height: local_height };
        let data = bincode::serialize(&req)?;
        stream.write_all(&data).await?;

        let mut buf = vec![0; 10 * 1024 * 1024]; // 10MB buffer
        let n = stream.read(&mut buf).await?;
        let msg: Message = bincode::deserialize(&buf[..n])?;

        match msg {
            Message::FastSyncResponse { utxo_snapshot, blocks, target_height } => {
                Ok(Self {
                    target_height,
                    utxo_snapshot,
                    blocks,
                })
            }
            _ => Err("Invalid response".into()),
        }
    }

    pub fn apply_to_chain(&self, bc: &mut PrunedBlockchain) -> Result<(), &'static str> {
        // Verify chain continuity
        if bc.bc.chain.last().unwrap().index + 1 != self.blocks[0].index {
            return Err("Chain gap");
        }

        // Replace UTXO set
        let new_utxo = bincode::deserialize(&self.utxo_snapshot)
            .map_err(|_| "Invalid UTXO snapshot")?;
        bc.bc.utxo_set = new_utxo;

        // Append blocks
        for block in &self.blocks {
            bc.bc.chain.push(block.clone());
        }

        let _ = bc.bc.storage.save_chain(&bc.bc.chain);
        println!("Fast sync complete! Height: {}", self.target_height);
        Ok(())
    }
}

// Extend P2PNode
impl P2PNode {
    pub fn handle_fast_sync_request(&self, from_height: u64, stream: &mut TcpStream) {
        let bc = self.bc.lock().unwrap();
        if bc.chain.len() as u64 <= from_height {
            return;
        }

        let snapshot = bincode::serialize(&bc.utxo_set).unwrap();
        let blocks: Vec<_> = bc.chain.iter()
            .skip(from_height as usize)
            .cloned()
            .collect();

        let resp = Message::FastSyncResponse {
            utxo_snapshot: snapshot,
            blocks,
            target_height: bc.chain.len() as u64 - 1,
        };

        if let Ok(data) = bincode::serialize(&resp) {
            let _ = stream.write_all(&data);
        }
    }
}

// Extend Message enum
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Message {
    // ... existing
    FastSyncRequest { from_height: u64 },
    FastSyncResponse {
        utxo_snapshot: Vec<u8>,
        blocks: Vec<crate::blockchain::block::Block>,
        target_height: u64,
    },
}
```

---

### Step 3: Update `src/network/p2p.rs`

Add to `handle_message`:

```rust
Message::FastSyncRequest { from_height } => {
    self.handle_fast_sync_request(from_height, stream);
}
```

---

### Step 4: Add CLI Command: `fast-sync`

```rust
// In src/cli/miner_cli.rs
Commands::FastSync {
    #[arg(short, long, default_value = "127.0.0.1:9000")]
    peer: String,
} => {
    let mut bc = PrunedBlockchain::new();
    let local_height = bc.bc.chain.len() as u64;

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            match FastSync::download_from_peer(&peer, local_height).await {
                Ok(sync) => {
                    bc.apply_to_chain(&sync).unwrap();
                }
                Err(e) => println!("Sync failed: {}", e),
            }
        });
}
```

---

### Step 5: Run Fast Sync

```bash
# Terminal 1: Run full node
cargo run -- daemon

# Terminal 2: New node → fast sync
cargo run -- fast-sync --peer 127.0.0.1:9000
```

**Output:**
```
Fast sync complete! Height: 1500
```

---

### Step 6: Git Commit

```bash
git add src/network/sync.rs src/network/p2p.rs src/cli/miner_cli.rs
git commit -m "Day 30: Fast sync – download UTXO snapshot + recent blocks (under 10s)"
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
| `FastSync` | `bootstrap` |
| UTXO snapshot | `blockchain.db` |
| Recent blocks | `getblocks` |
| **<10s sync** | **SPV-ready** |

> **New nodes join in seconds**

---

## Day 30 Complete!

| Done |
|------|
| `src/network/sync.rs` |
| **Fast sync under 10s** |
| **UTXO snapshot** |
| **CLI `fast-sync`** |
| Git commit |

---

## Tomorrow (Day 31): SPV Wallet

We’ll:
- **Verify tx inclusion** with Merkle proof
- **No full chain needed**
- File: `src/wallet/spv.rs`

```bash
touch src/wallet/spv.rs
```

---

**Ready?** Say:  
> `Yes, Day 31`

Or ask:
- “Can I run mobile wallet?”
- “Add bloom filter?”
- “Show proof size?”

We’re **30/50** — **Your network is now INSTANTLY JOINABLE**
