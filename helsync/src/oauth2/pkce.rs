use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

use sha2::{Digest, Sha256};
use rand::Rng;

/// PKCE simplifies creating and verifying PKCE challenges. PKCE is an
/// OAuth2 security extension that allows public clients like CLI or
/// native apps (which cannot securely store a client secret) to
/// authenticate with OAuth2 APIs.
pub struct PKCE {
    pub verifier: String,
    pub challenge: String,
    pub method: String,
}

impl PKCE {

    /// Generates a new PKCE challenge with S256 method.
    pub fn new() -> Self {
        let verifier = Self::gen_verifier();
        let challenge = Self::gen_challenge(&verifier);
        let method = "S256".to_string();
        Self {verifier, challenge, method}
    }

    /// Generate a cryptographically secure PKCE code verifier.
    pub fn gen_verifier() -> String {
        let mut rng = rand::rng();
        let len = rng.random_range(43..=128);
        const CHARSET: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";

        (0..len)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Generate S256 code challenge from verifier.
    /// challenge = BASE64URL(SHA256(verifier))
    pub fn gen_challenge(verifier: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let hash = hasher.finalize();
        URL_SAFE_NO_PAD.encode(&hash)
    }

    /// Verify that a code verifier matches this challenge.
    pub fn verify(&self, verifier: &str) -> bool {
        let expected_challenge = Self::gen_challenge(verifier);
        self.challenge == expected_challenge
    }
}
