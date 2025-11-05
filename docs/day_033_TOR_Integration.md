**DAY 33: Tor Integration**  
**Goal:** **Route all traffic via Tor** — **hidden service + onion address**  
**Repo Task:**  
> Integrate **Tor** using `arti-client` in `/src/network/tor.rs`

We’ll **run as a Tor hidden service**, **connect to peers over .onion**, **hide real IP** — **your node is now fully anonymous**.

---

## Step-by-Step Guide for Day 33

---

### Step 1: Add `arti-client`

```bash
cargo add arti-client --features=async,static
```

```toml
[dependencies]
arti-client = { version = "0.12", features = ["async", "static"] }
tor-rtcompat = "0.12"
```

---

### Step 2: Create `src/network/tor.rs`

```bash
touch src/network/tor.rs
```

---

### Step 3: `src/network/tor.rs`

```rust
// src/network/tor.rs

use arti_client::{TorClient, TorClientConfig};
use tor_rtcompat::Runtime;
use std::sync::Arc;
use tokio::net::TcpStream;
use std::net::SocketAddr;

pub struct TorNode {
    client: Arc<TorClient<Runtime>>,
    onion_addr: String,
}

impl TorNode {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = TorClientConfig::default();
        let runtime = tor_rtcompat::tokio::TokioRuntime::new()?;
        let client = TorClient::create_bootstrapped(config, runtime).await?;

        // Create hidden service
        let (onion_addr, _) = client
            .isolated_client()
            .await?
            .create_onion_service(80, None)
            .await?;

        let onion_addr = onion_addr.to_string();
        println!("Hidden service: {}.onion", onion_addr);

        Ok(Self {
            client: Arc::new(client),
            onion_addr,
        })
    }

    /// Connect to .onion peer
    pub async fn connect_onion(&self, onion: &str) -> Result<TcpStream, Box<dyn std::error::Error>> {
        let addr: SocketAddr = format!("{}:80", onion).parse()?;
        let stream = self.client.connect(&addr).await?;
        Ok(stream)
    }

    /// Get our onion address
    pub fn onion_address(&self) -> &str {
        &self.onion_addr
    }
}
```

---

### Step 4: Update `src/network/p2p.rs` – Use Tor

```rust
// Replace TcpStream with Tor
use crate::network::tor::TorNode;

pub struct P2PNode {
    pub tor: Arc<TorNode>,
    // ...
}

impl P2PNode {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let tor = Arc::new(TorNode::new().await?);
        Ok(Self {
            tor,
            // ...
        })
    }

    pub async fn connect_to_peer(&self, onion: &str) -> Result<(), Box<dyn std::error::Error>> {
        let stream = self.tor.connect_onion(onion).await?;
        // spawn handler
        Ok(())
    }
}
```

---

### Step 5: Update `src/cli/miner_cli.rs`

```rust
// In daemon
let p2p = P2PNode::new().await?;
println!("Connect to me at: {}.onion", p2p.tor.onion_address());
```

---

### Step 6: Run Over Tor

```bash
# Terminal 1
cargo run -- daemon
# → Hidden service: abcdef123456.onion

# Terminal 2
cargo run -- daemon --connect abcdef123456.onion
```

**All traffic now via Tor**

---

### Step 7: Git Commit

```bash
git add src/network/tor.rs src/network/p2p.rs src/cli/miner_cli.rs Cargo.toml
git commit -m "Day 33: Full Tor integration – hidden service, .onion peers, IP hidden"
```

---

### Step 8: Push

```bash
git push origin main
```

---

## Why This Matters

| Feature | Monero Equivalent |
|-------|-------------------|
| `arti-client` | `tor` |
| `.onion` service | `p2p over tor` |
| `connect_onion()` | `tor_connect()` |
| **No IP leak** | **Anonymous node** |

> **Your node is now UNTRACEABLE**

---

## Day 33 Complete!

| Done |
|------|
| `src/network/tor.rs` |
| **Tor hidden service** |
| **.onion peer connections** |
| **No real IP exposed** |
| **CLI shows onion addr** |
| Git commit |

---

## Tomorrow (Day 34): RPC API

We’ll:
- **JSON-RPC server**
- `get_balance`, `send_tx`, `get_block`
- File: `src/rpc/server.rs`

```bash
cargo add jsonrpc-http-server
```

---

**Ready?** Say:  
> `Yes, Day 34`

Or ask:
- “Can I use with wallet?”
- “Add WebSocket?”
- “Show API docs?”

We’re **33/50** — **Your node is now ANONYMOUS over TOR**
