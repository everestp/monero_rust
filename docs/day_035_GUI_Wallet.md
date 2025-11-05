**DAY 35: GUI Wallet**  
**Goal:** **Tauri desktop wallet** — **connect to RPC**, **send/receive**, **balance view**  
**Repo Task:**  
> Build **cross-platform GUI** with `tauri` in `/src-tauri`

We’ll create a **beautiful desktop wallet** — **Windows, macOS, Linux** — **your users now have a real Monero-like GUI**.

---

## Step-by-Step Guide for Day 35

---

### Step 1: Initialize Tauri

```bash
cargo install tauri-cli
cargo tauri init
```

Answer prompts:
```
? What is your frontend language? › Vanilla
? What is your frontend command? › npm run dev
? What is your build command? › npm run build
```

---

### Step 2: Update `src-tauri/Cargo.toml`

```toml
[package]
name = "monero-rust-wallet"
version = "0.1.0"
edition = "2021"

[dependencies]
tauri = { version = "1.5", features = ["api-all"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1.0", features = ["rt-multi-thread"] }
```

---

### Step 3: `src-tauri/src/main.rs`

```rust
// src-tauri/src/main.rs

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use reqwest;
use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Serialize, Deserialize)]
struct RpcRequest {
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
    id: u32,
}

#[derive(Deserialize)]
struct RpcResponse<T> {
    result: T,
}

#[tauri::command]
async fn get_balance(address: String) -> Result<u64, String> {
    let client = reqwest::Client::new();
    let req = RpcRequest {
        jsonrpc: "2.0".into(),
        method: "get_balance".into(),
        params: serde_json::json!({ "address": address }),
        id: 1,
    };

    let resp: RpcResponse<u64> = client
        .post("http://127.0.0.1:18081")
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(resp.result)
}

#[tauri::command]
async fn send_transaction(to: String, amount: u64) -> Result<String, String> {
    let client = reqwest::Client::new();
    let req = RpcRequest {
        jsonrpc: "2.0".into(),
        method: "send_transaction".into(),
        params: serde_json::json!({ "to": to, "amount": amount }),
        id: 1,
    };

    let resp: RpcResponse<String> = client
        .post("http://127.0.0.1:18081")
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(resp.result)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_balance, send_transaction])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

### Step 4: Frontend – `src/index.html`

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Monero Rust Wallet</title>
  <style>
    body { font-family: sans-serif; padding: 20px; background: #1a1a1a; color: #fff; }
    input, button { padding: 10px; margin: 5px; font-size: 16px; }
    .balance { font-size: 24px; margin: 20px 0; }
  </style>
</head>
<body>
  <h1>Monero Rust Wallet</h1>
  <div class="balance">Balance: <span id="balance">0</span> XMR</div>

  <input id="address" placeholder="Your address" value="4A1B..." />
  <button onclick="refresh()">Refresh</button>

  <hr>

  <input id="to" placeholder="Send to" />
  <input id="amount" type="number" placeholder="Amount" />
  <button onclick="send()">Send</button>

  <script>
    const { invoke } = window.__TAURI__.tauri;

    async function refresh() {
      const addr = document.getElementById('address').value;
      const balance = await invoke('get_balance', { address: addr });
      document.getElementById('balance').innerText = balance;
    }

    async function send() {
      const to = document.getElementById('to').value;
      const amount = parseInt(document.getElementById('amount').value);
      try {
        await invoke('send_transaction', { to, amount });
        alert('Sent!');
        refresh();
      } catch (e) {
        alert('Error: ' + e);
      }
    }

    // Auto-refresh
    setInterval(refresh, 10000);
    refresh();
  </script>
</body>
</html>
```

---

### Step 5: Run GUI Wallet

```bash
# Terminal 1: Start node
cargo run -- daemon

# Terminal 2: Run wallet
cargo tauri dev
```

**Beautiful desktop app opens**

---

### Step 6: Build for All Platforms

```bash
cargo tauri build
```

Outputs:
```
target/release/bundle/dmg/monero-rust-wallet.dmg
target/release/bundle/msi/monero-rust-wallet.msi
target/release/bundle/appimage/monero-rust-wallet.AppImage
```

---

### Step 7: Git Commit

```bash
git add src-tauri/ src/index.html
git commit -m "Day 35: Tauri GUI wallet – balance, send, cross-platform"
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
| `tauri` | `monero-wallet-gui` |
| `get_balance` | Wallet RPC |
| `send_transaction` | Transfer |
| **Desktop app** | **User-friendly** |

> **Anyone can now use your coin**

---

## Day 35 Complete!

| Done |
|------|
| `src-tauri/` |
| **Tauri desktop wallet** |
| **Balance + send** |
| **Auto-refresh** |
| **Builds for Win/macOS/Linux** |
| Git commit |

---

## Tomorrow (Day 36): Mobile Wallet (React Native)

We’ll:
- **React Native app**
- **Connect to RPC**
- File: `mobile/`

```bash
npx react-native init MoneroMobile
```

---

**Ready?** Say:  
> `Yes, Day 36`

Or ask:
- “Can I build iOS/Android?”
- “Add QR scanner?”
- “Show splash screen?”

We’re **35/50** — **Your coin now has a REAL GUI WALLET**
