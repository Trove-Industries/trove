use rand::{RngCore, rngs::OsRng};
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose, Engine as _};

/// Generates a cryptographically secure session token
pub fn generate_session_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);

    // Optional: hash then base64 for shorter length
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let hash = hasher.finalize();

    general_purpose::URL_SAFE_NO_PAD.encode(hash)
}
