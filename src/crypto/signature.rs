// src/crypto/signature.rs
use ed25519_dalek::{Signer, Verifier, SigningKey, VerifyingKey, Signature};
use rand::rngs::OsRng;
use std::error::Error;

/// Our own keypair wrapper (clean and safe)
#[derive(Clone)]
pub struct Ed25519Keypair {
    pub public: VerifyingKey,   pub // Only public part is exposed
    signing_key: SigningKey,    // Full key (includes secret) — kept private
}

impl Ed25519Keypair {
    /// Generate a new random keypair using secure OS randomness
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let public = signing_key.verifying_key();
        Self { public, signing_key }
    }

    /// Sign a message (e.g. transaction)
    pub fn sign(&self, msg: &[u8]) -> Signature {
        self.signing_key.sign(msg)
    }

    /// Get public key as 32 bytes
    pub fn public_bytes(&self) -> [u8; 32] {
        self.public.to_bytes()
    }

    /// Get public key as VerifyingKey (useful for verification)
    pub fn verifying_key(&self) -> VerifyingKey {
        self.public
    }
}

/// Standalone function to verify a signature with raw bytes
pub fn verify_signature(
    public_key: &[u8],   // 32 bytes
    message: &[u8],
    signature: &[u8],   // 64 bytes
) -> Result<(), Box<dyn Error>> {
    let pub_key = VerifyingKey::from_bytes(public_key.try_into()?)?;
    let sig = Signature::from_bytes(signature.try_into()?);
    pub_key.verify(message, &sig)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify_full_flow() {
        let keypair = Ed25519Keypair::generate();
        let message = b"Monero is private money";

        let signature = keypair.sign(message);
        let sig_bytes = signature.to_bytes();
        let pub_bytes = keypair.public_bytes();

        // Should succeed
        assert!(verify_signature(&pub_bytes, message, &sig_bytes).is_ok());

        // Tampered message → fail
        assert!(verify_signature(&pub_bytes, b"hacked!", &sig_bytes).is_err());

        // Wrong public key → fail
        let other = Ed25519Keypair::generate();
        assert!(verify_signature(&other.public_bytes(), message, &sig_bytes).is_err());
    }

    #[test]
    fn test_verifying_key_directly() {
        let kp = Ed25519Keypair::generate();
        let msg = b"Ring signatures incoming...";
        let sig = kp.sign(msg);

        // Direct verify using the VerifyingKey (fastest way)
        assert!(kp.verifying_key().verify(msg, &sig).is_ok());
    }

    #[test]
    fn test_determinism() {
        // Same key + same message = same signature
        let kp = Ed25519Keypair::generate();
        let msg = b"test";

        let sig1 = kp.sign(msg);
        let sig2 = kp.sign(msg);

        assert_eq!(sig1.to_bytes(), sig2.to_bytes()); // Ed25519 is deterministic in dalek v2+
    }
}