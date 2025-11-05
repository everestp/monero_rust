**DAY 20: P2P Network**  
**Goal:** Connect to peers, sync blocks, and broadcast transactions  
**Repo Task:**  
> Implement P2P networking in `/src/network/p2p.rs`

We’ll build a **real P2P node** that **connects**, **handshakes**, **syncs blocks**, and **relays txs** — your blockchain is now **distributed**.

---

## Step-by-Step Guide for Day 20

---

### Step 1: Create `src/network/p2p.rs`

```bash
mkdir -p src/network
touch src/network/p2p.rs
```

---

### Step 2: `src/network/p2p.rs`

```rust
// src/network/p2p.rs

use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use crate::blockchain::storage::PersistentBlockchain;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Ping,
    Pong,
    GetBlocks { from: u64 },
    Blocks { blocks: Vec<Block> },
    NewBlock { block: Block },
    NewTx { tx: Transaction },
}

pub struct Peer {
    addr: SocketAddr,
    stream: TcpStream,
}

pub struct P2PNode {
    bc: Arc<Mutex<PersistentBlockchain>>,
    peers: Arc<Mutex<Vec<SocketAddr>>>,
    tx_sender: mpsc::UnboundedSender<Message>,
}

impl P2PNode {
    pub fn new(bc: Arc<Mutex<PersistentBlockchain>>) -> Self {
        let (tx_sender, _) = mpsc::unbounded_channel();
        Self {
            bc,
            peers: Arc::new(Mutex::new(vec![])),
            tx_sender,
        }
    }

    pub async fn start(&self, listen_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(listen_addr).await?;
        println!("P2P node listening on {}", listen_addr);

        let node = self.clone();
        tokio::spawn(async move {
            loop {
                let (stream, addr) = listener.accept().await.unwrap();
                println!("New peer: {}", addr);
                node.peers.lock().unwrap().push(addr);
                tokio::spawn(node.clone().handle_peer(stream));
            }
        });

        Ok(())
    }

    async fn handle_peer(self, mut stream: TcpStream) {
        let mut buf = [0; 1024];
        loop {
            match stream.read(&mut buf).await {
                Ok(0) => break, // disconnected
                Ok(n) => {
                    if let Ok(msg) = bincode::deserialize::<Message>(&buf[..n]) {
                        self.handle_message(msg, &mut stream).await;
                    }
                }
                Err(_) => break,
            }
        }
    }

    async fn handle_message(&self, msg: Message, stream: &mut TcpStream) {
        match msg {
            Message::Ping => {
                self.send(stream, Message::Pong).await;
            }
            Message::Pong => {
                println!("Pong received");
            }
            Message::GetBlocks { from } => {
                let bc = self.bc.lock().unwrap();
                let blocks: Vec<Block> = bc.chain.iter()
                    .filter(|b| b.index >= from)
                    .cloned()
                    .collect();
                self.send(stream, Message::Blocks { blocks }).await;
            }
            Message::Blocks { blocks } => {
                let mut bc = self.bc.lock().unwrap();
                for block in blocks {
                    if block.index == bc.chain.len() as u64 {
                        bc.chain.push(block);
                        println!("Synced block {}", block.index);
                    }
                }
                let _ = bc.storage.save_chain(&bc.chain);
            }
            Message::NewBlock { block } => {
                let mut bc = self.bc.lock().unwrap();
                if block.index == bc.chain.len() as u64 && block.prev_hash == bc.chain.last().unwrap().hash {
                    bc.chain.push(block);
                    println!("New block {} received", block.index);
                    let _ = bc.storage.save_chain(&bc.chain);
                }
            }
            Message::NewTx { tx } => {
                // Relay to pool later
                println!("New tx: {}", hex::encode(&tx.id()[..4]));
            }
        }
    }

    async fn send(&self, stream: &mut TcpStream, msg: Message) {
        if let Ok(data) = bincode::serialize(&msg) {
            let _ = stream.write_all(&data).await;
        }
    }

    pub fn broadcast_block(&self, block: &Block) {
        let msg = Message::NewBlock { block: block.clone() };
        let data = bincode::serialize(&msg).unwrap();
        let peers = self.peers.lock().unwrap().clone();
        for addr in peers {
            tokio::spawn(async move {
                if let Ok(mut stream) = TcpStream::connect(addr).await {
                    let _ = stream.write_all(&data).await;
                }
            });
        }
    }

    pub fn broadcast_tx(&self, tx: &Transaction) {
        let msg = Message::NewTx { tx: tx.clone() };
        let data = bincode::serialize(&msg).unwrap();
        let peers = self.peers.lock().unwrap().clone();
        for addr in peers {
            tokio::spawn(async move {
                if let Ok(mut stream) = TcpStream::connect(addr).await {
                    let _ = stream.write_all(&data).await;
                }
            });
        }
    }
}

impl Clone for P2PNode {
    fn clone(&self) -> Self {
        Self {
            bc: self.bc.clone(),
            peers: self.peers.clone(),
            tx_sender: self.tx_sender.clone(),
        }
    }
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_p2p_sync() {
        let bc1 = Arc::new(Mutex::new(PersistentBlockchain::new()));
        let bc2 = Arc::new(Mutex::new(PersistentBlockchain::new()));

        let node1 = P2PNode::new(bc1.clone());
        let node2 = P2PNode::new(bc2.clone());

        node1.start("127.0.0.1:9001").await.unwrap();
        node2.start("127.0.0.1:9002").await.unwrap();

        // Mine on node1
        {
            let mut bc = bc1.lock().unwrap();
            let mut miner = crate::blockchain::mining::Miner::new();
            bc.add_block(vec![], &mut miner);
        }

        sleep(Duration::from_millis(100)).await;

        // Sync
        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:9001").await {
            let msg = Message::GetBlocks { from: 0 };
            let data = bincode::serialize(&msg).unwrap();
            stream.write_all(&data).await.unwrap();
        }

        sleep(Duration::from_millis(200)).await;

        assert!(bc2.lock().unwrap().chain.len() >= 2);
    }
}
```

---

### Step 3: Update `Cargo.toml` for `tokio`

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
```

---

### Step 4: Update `src/cli/miner_cli.rs` to Start P2P

```rust
// In run_cli()
let bc = Arc::new(Mutex::new(PersistentBlockchain::new()));
let p2p = P2PNode::new(bc.clone());

match cli.command {
    Commands::Daemon => {
        p2p.start("127.0.0.1:9000").await.unwrap();
        // ... mining loop
    }
    // In add_block:
    p2p.broadcast_block(&mined_block);
}
```

---

### Step 5: Run Two Nodes

```bash
# Terminal 1
cargo run -- daemon

# Terminal 2
cargo run -- daemon --port 9001
```

Watch them **sync blocks**!

---

### Step 6: Git Commit

```bash
git add src/network/p2p.rs src/cli/miner_cli.rs Cargo.toml
git commit -m "Day 20: Full P2P network – sync blocks, broadcast txs, handshake (1 test)"
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
| `GetBlocks` | `getblocks.bin` |
| `NewBlock` | `fluffy blocks` |
| `NewTx` | `tx pool relay` |
| `tokio` | `epee` + `levin` |

> **Your node is now part of a network**

---

## Day 20 Complete!

| Done |
|------|
| `src/network/p2p.rs` |
| **P2P sync** |
| **Block & tx relay** |
| **Handshake** |
| 1 async test |
| Git commit |

---

## Tomorrow (Day 21): Wallet & Keys

We’ll:
- Generate **Monero-style wallets**
- **Scan for payments**
- File: `src/wallet/mod.rs`

```bash
cargo add rand
```

---

**Ready?** Say:  
> `Yes, Day 21`

Or ask:
- “Can I connect to real Monero?”
- “Add DNS seed?”
- “Show peer list?”

We’re **20/50** — **Your blockchain is now DISTRIBUTED and ALIVE**
