//! Detached Ed25519 signatures for the extension registry.
//!
//! `registry.json` pins every extension archive's SHA-256, and each extension
//! manifest pins its tools' SHA-256, so the registry is the root of trust for
//! everything the marketplace installs. Signing it means that whoever controls
//! the bucket or the CDN in front of it still cannot swap in a different
//! archive: only the holder of the private key can publish a registry that a
//! build with the matching public key accepts.

use anyhow::{ensure, Context, Result};
use ring::signature::{UnparsedPublicKey, ED25519};
use serde::{Deserialize, Serialize};

/// `registry.signed.json`: the exact registry text and its signature in one
/// object, so a reader never pairs a new payload with a stale signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedDocument {
    /// The registry JSON, verbatim.
    pub payload: String,
    /// Hex-encoded Ed25519 signature over the UTF-8 bytes of `payload`.
    pub signature: String,
}

/// Checks `doc` against a hex-encoded 32-byte Ed25519 public key.
pub fn verify(doc: &SignedDocument, public_key_hex: &str) -> Result<()> {
    let key = hex::decode(public_key_hex.trim()).context("registry public key is not hex")?;
    ensure!(key.len() == 32, "registry public key must be 32 bytes");
    let signature = hex::decode(doc.signature.trim()).context("registry signature is not hex")?;
    UnparsedPublicKey::new(&ED25519, &key)
        .verify(doc.payload.as_bytes(), &signature)
        .map_err(|_| anyhow::anyhow!("registry signature does not verify"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ring::rand::SystemRandom;
    use ring::signature::{Ed25519KeyPair, KeyPair as _};

    fn key_pair() -> Ed25519KeyPair {
        let pkcs8 = Ed25519KeyPair::generate_pkcs8(&SystemRandom::new()).unwrap();
        Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap()
    }

    fn signed(keys: &Ed25519KeyPair, payload: &str) -> SignedDocument {
        SignedDocument {
            payload: payload.to_owned(),
            signature: hex::encode(keys.sign(payload.as_bytes())),
        }
    }

    #[test]
    fn accepts_the_matching_key_and_rejects_everything_else() {
        let keys = key_pair();
        let public = hex::encode(keys.public_key());
        let doc = signed(&keys, r#"{"last_updated":"now","extensions":{}}"#);
        assert!(verify(&doc, &public).is_ok());
        assert!(verify(&doc, &format!(" {public}\n")).is_ok());

        let mut tampered = doc.clone();
        tampered.payload.push(' ');
        assert!(verify(&tampered, &public).is_err());

        let other = hex::encode(key_pair().public_key());
        assert!(verify(&doc, &other).is_err());

        let mut garbled = doc.clone();
        garbled.signature = "zz".into();
        assert!(verify(&garbled, &public).is_err());
        assert!(verify(&doc, "abcd").is_err());
    }
}
