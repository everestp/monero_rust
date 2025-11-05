**DAY 3: Async in Rust**  
**Goal:** Master `async/await`, `Futures`, and `tokio`  
**Repo Task:**  
> Add async TCP server/client example inside `/src/network/async.rs`

We’ll build a **fully functional TCP echo server + client** using `tokio`, with **real async code**, **tests**, **error handling**, and **Git commit** — laying the **foundation for P2P blockchain networking**.

---

## Step-by-Step Guide for Day 3

---

### Step 1: Add `tokio` to `Cargo.toml`

```bash
cargo add tokio --features=full
```

This adds:

```toml
[dependencies]
tokio = { version = "1.38", features = ["full"] }
```

---

### Step 2: Create the File

```bash
mkdir -p src/network
touch src/network/async.rs
```

---

### Step 3: Write the Code – `src/network/async.rs`

```rust
// src/network/async.rs

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::error::Error;

// === 1. Echo Server ===
async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];

    loop {
        // Read from client
        let n = socket.read(&mut buffer).await?;
        if n == 0 {
            println!("Client disconnected");
            return Ok(());
        }

        let received = String::from_utf8_lossy(&buffer[..n]);
        println!("Server received: {}", received.trim());

        // Echo back
        socket.write_all(&buffer[..n]).await?;
        println!("Server echoed back: {}", received.trim());
    }
}

async fn run_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Echo server running on 127.0.0.1:7878");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client: {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Client error: {}", e);
            }
        });
    }
}

// === 2. Echo Client ===
async fn run_client() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:7878").await?;
    println!("Connected to server");

    let messages = vec![
        "Hello, Blockchain!",
        "Async Rust is awesome!",
        "Building Monero in 50 days!",
        "quit",
    ];

    for msg in messages {
        let data = msg.as_bytes();
        stream.write_all(data).await?;
        println!("Client sent: {}", msg);

        if msg == "quit" {
            break;
        }

        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]);
        println!("Client received: {}", response.trim());
    }

    Ok(())
}

// === 3. Integration Test (Run Server in Background) ===
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_echo_server_client() {
        // Start server in background
        let server_handle = tokio::spawn(async {
            run_server().await.unwrap();
        });

        // Give server time to bind
        sleep(Duration::from_millis(100)).await;

        // Run client
        let client_result = run_client().await;
        assert!(client_result.is_ok());

        // Give time for messages to process
        sleep(Duration::from_millis(100)).await;

        // Shut down server
        server_handle.abort();
        let _ = server_handle.await; // ignore abort error
    }
}

// === 4. Main Function for Manual Testing ===
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    println!("Choose mode: (s)erver or (c)lient");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    match input.trim() {
        "s" | "server" => run_server().await?,
        "c" | "client" => run_client().await?,
        _ => println!("Invalid choice. Use 's' or 'c'"),
    }

    Ok(())
}
```

---

### Step 4: Update `src/lib.rs` to Include Module

```rust
// src/lib.rs

pub mod tests;
pub mod network;
```

---

### Step 5: Run the Program Manually

#### Terminal 1: Start Server
```bash
cargo run --bin monero-rust-blockchain
```
→ Type `s` → Server starts

#### Terminal 2: Start Client
```bash
cargo run --bin monero-rust-blockchain
```
→ Type `c` → Client connects and sends messages

**Expected Output (Client):**
```
Connected to server
Client sent: Hello, Blockchain!
Client received: Hello, Blockchain!
Client sent: Async Rust is awesome!
Client received: Async Rust is awesome!
...
```

---

### Step 6: Run Automated Test

```bash
cargo test
```

**Expected:**
```
running 1 test
test network::async::tests::test_echo_server_client ... ok
```

---

### Step 7: Git Commit

```bash
git add src/network/async.rs src/lib.rs Cargo.toml
git commit -m "Day 3: Async TCP echo server + client with tokio (full test)"
```

---

### Step 8: Push to GitHub

```bash
git push origin main
```

---

## Why This Matters for Your Blockchain

| Feature | Future Use |
|-------|------------|
| `tokio::net::TcpListener` | P2P node listening |
| `tokio::spawn` | Handle 1000+ peers concurrently |
| `async/await` | Non-blocking block/tx propagation |
| `TcpStream` | Send serialized blocks/transactions |
| `handle_client` | Per-peer message handler |

This is the **exact foundation** for **Day 20: P2P Network Implementation**

---

## Pro Tips

1. **Never block in async code** → Use `tokio::fs`, `tokio::time`, etc.
2. **Graceful shutdown later** → Use `tokio::signal`
3. **Message framing** → Later: use `length-prefix` or `bincode`

---

## Day 3 Complete!

| Done |
|------|
| `tokio` added |
| TCP echo server + client |
| `async/await`, `spawn`, `read/write` |
| Full integration test |
| Manual CLI mode |
| Git commit |

---

## Tomorrow (Day 4 Preview): Crypto Basics

```bash
cargo add sha3 blake2 ed25519-dalek --features=serde
```

We’ll:
- Implement **Blake2b** hashing
- Sign/verify with **Ed25519**
- Build `src/crypto/hash.rs` and `signature.rs`

---

**Ready for Day 4?** Say:  
> `Yes, give me Day 4`

Or ask:
- “Can you generate a `Dockerfile` for testing?”
- “How do I run server/client in background?”
- “Add message types (block, tx) to async?”

We’re **3/50** — your **Monero-like blockchain is growing**
