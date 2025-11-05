**DAY 15: CLI Interface**  
**Goal:** Build a **user-friendly CLI** to interact with the blockchain  
**Repo Task:**  
> Add CLI commands to mine and view blockchain status in `/src/cli/miner_cli.rs`

We’ll use **`clap`** to create commands like `cargo run -- mine`, `status`, `send` — **your blockchain is now interactive!**

---

## Step-by-Step Guide for Day 15

---

### Step 1: Add `clap` to `Cargo.toml`

```bash
cargo add clap --features=derive
```

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
```

---

### Step 2: Create `src/cli/miner_cli.rs`

```bash
mkdir -p src/cli
touch src/cli/miner_cli.rs
```

---

### Step 3: `src/cli/miner_cli.rs`

```rust
// src/cli/miner_cli.rs

use clap::{Parser, Subcommand};
use crate::blockchain::block::Blockchain;
use crate::blockchain::mining::Miner;
use crate::blockchain::transaction::Transaction;
use crate::crypto::signature::Ed25519Keypair;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Monero-Rust Blockchain CLI
#[derive(Parser)]
#[command(name = "monero-rust")]
#[command(about = "A privacy-preserving blockchain in Rust")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Mine a new block
    Mine {
        #[arg(short, long, default_value_t = 1)]
        blocks: usize,
    },
    /// Show blockchain status
    Status,
    /// Send a transaction (dummy)
    Send {
        #[arg(short, long)]
        to: String,
        #[arg(short, long)]
        amount: u64,
    },
    /// Run mining in background
    Daemon,
}

pub fn run_cli() {
    let cli = Cli::parse();

    // Shared blockchain state
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let miner = Arc::new(Mutex::new(Miner::new()));

    match cli.command {
        Commands::Mine { blocks } => {
            let mut miner = miner.lock().unwrap();
            let mut bc = blockchain.lock().unwrap();

            println!("Mining {} block(s)...", blocks);
            for _ in 0..blocks {
                let txs = vec![create_dummy_tx()];
                bc.add_block(txs, &mut miner);
            }
            bc.print();
        }

        Commands::Status => {
            let bc = blockchain.lock().unwrap();
            bc.print();
            println!("Total blocks: {}", bc.chain.len());
            println!("Latest hash: {}", hex::encode(&bc.chain.last().unwrap().hash[..8]));
        }

        Commands::Send { to, amount } => {
            let mut bc = blockchain.lock().unwrap();
            let mut miner = miner.lock().unwrap();

            let tx = create_tx_to(&to, amount);
            println!("Sending {} to {}...", amount, to);
            bc.add_block(vec![tx], &mut miner);
            println!("Transaction included in block!");
        }

        Commands::Daemon => {
            let bc = Arc::clone(&blockchain);
            let miner = Arc::clone(&miner);

            thread::spawn(move || {
                let mut miner = miner.lock().unwrap();
                let mut bc = bc.lock().unwrap();

                println!("Daemon started: mining every 10s");
                loop {
                    let txs = vec![create_dummy_tx()];
                    bc.add_block(txs.clone(), &mut miner);
                    println!("Mined block {}", bc.chain.len() - 1);
                    thread::sleep(Duration::from_secs(10));
                }
            });

            println!("Daemon running in background. Press Ctrl+C to stop.");
            loop { thread::sleep(Duration::from_secs(1)); }
        }
    }
}

// Helper: create dummy signed tx
fn create_dummy_tx() -> Transaction {
    let alice = Ed25519Keypair::generate();
    let bob = Ed25519Keypair::generate();
    let mut tx = Transaction::new(
        alice.public_bytes().to_vec(),
        bob.public_bytes().to_vec(),
        50,
        1,
        rand::random(),
    );
    tx.sign(&alice).unwrap();
    tx
}

// Helper: create tx to address (hex string)
fn create_tx_to(to_hex: &str, amount: u64) -> Transaction {
    let alice = Ed25519Keypair::generate();
    let receiver = hex::decode(to_hex).unwrap_or(vec![0; 32]);
    let mut tx = Transaction::new(
        alice.public_bytes().to_vec(),
        receiver,
        amount,
        1,
        rand::random(),
    );
    tx.sign(&alice).unwrap();
    tx
}
```

---

### Step 4: Update `src/main.rs`

```rust
// src/main.rs

use monero_rust_blockchain::cli::miner_cli::run_cli;

fn main() {
    run_cli();
}
```

---

### Step 5: Update `src/lib.rs`

```rust
// src/lib.rs

pub mod tests;
pub mod network;
pub mod crypto;
pub mod blockchain;
pub mod cli;
```

---

### Step 6: Update `src/cli/mod.rs`

```bash
touch src/cli/mod.rs
```

```rust
// src/cli/mod.rs
pub mod miner_cli;
```

---

### Step 7: Test the CLI

#### 1. Mine 2 blocks
```bash
cargo run -- mine --blocks 2
```

#### 2. Check status
```bash
cargo run -- status
```

#### 3. Send transaction
```bash
cargo run -- send --to a1b2c3d4 --amount 100
```

#### 4. Run daemon (background mining)
```bash
cargo run -- daemon
```

**Sample Output:**
```
$ cargo run -- mine --blocks 2
Mining 2 block(s)...
Mined block 1 | difficulty: 4 | target: 0x0fffffff
Mined! nonce: 12345 | time: 0.12s | hash: 0000a1b2
...
Blockchain (3 blocks):
Block 0 | 1 txs | merkle: 1a2b3c4d | hash: 0000e5f6
Block 1 | 1 txs | merkle: 5f6a7b8c | hash: 0000d4c3
```

---

### Step 8: Git Commit

```bash
git add src/cli/ src/main.rs src/lib.rs Cargo.toml
git commit -m "Day 15: Full CLI with mine, status, send, daemon (clap)"
```

---

### Step 9: Push

```bash
git push origin main
```

---

## CLI Commands Summary

| Command | Action |
|-------|--------|
| `cargo run -- mine` | Mine 1 block |
| `cargo run -- mine --blocks 5` | Mine 5 blocks |
| `cargo run -- status` | Show chain |
| `cargo run -- send --to abc --amount 100` | Send tx |
| `cargo run -- daemon` | Auto-mine every 10s |

---

## Day 15 Complete!

| Done |
|------|
| `src/cli/miner_cli.rs` |
| **`clap` CLI** |
| `mine`, `status`, `send`, `daemon` |
| Background mining |
| Git commit |

---

## Tomorrow (Day 16): Persistent Storage

We’ll:
- Save blockchain to **disk** with `sled`
- Load on startup
- File: `src/blockchain/storage.rs`

```bash
cargo add sled
```

---

**Ready?** Say:  
> `Yes, Day 16`

Or ask:
- “Can I add wallet persistence?”
- “Export chain to JSON?”
- “Add --data-dir?”

We’re **15/50** — **Your blockchain is now USER-FACING and INTERACTIVE**
