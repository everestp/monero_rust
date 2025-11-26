// src/main.rs
mod crypto;

use monero_rust::crypto::signature::Ed25519Keypair;

fn main() {
    let wallet = Ed25519Keypair::generate();
    
    println!("New Monero-style Wallet Created!");
    println!("Public Key (Address base): {}", hex::encode(wallet.public_bytes()));
    println!("Secret Key (NEVER SHARE): {}", hex::encode(wallet.signing_key.to_bytes()));
    
    let tx = b"Send 10 XMR to Alice";
    let signature = wallet.sign(tx);
    println!("Signed transaction:");
    println!("  Message: {}", String::from_utf8_lossy(tx));
    println!("  Signature: {}", hex::encode(signature.to_bytes()));
}