//! JWT and HMAC authentication for WASI components.
//!
//! Uses `time` crate for timestamps (no JS Date dependency).

use base64::Engine;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Verify an HMAC-SHA256 signature.
///
/// # Errors
///
/// Returns `Err` if the signature is invalid.
pub fn verify_hmac(secret: &[u8], message: &[u8], signature: &[u8]) -> Result<(), AuthError> {
    let mut mac = HmacSha256::new_from_slice(secret).map_err(|_| AuthError::InvalidKey)?;
    mac.update(message);
    mac.verify_slice(signature).map_err(|_| AuthError::InvalidSignature)
}

/// Decode a base64url-encoded JWT segment.
///
/// # Errors
///
/// Returns `Err` if the base64 decoding fails.
pub fn decode_jwt_segment(segment: &str) -> Result<Vec<u8>, AuthError> {
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(segment)
        .map_err(|_| AuthError::InvalidEncoding)
}

/// Authentication errors.
#[derive(Debug)]
pub enum AuthError {
    InvalidKey,
    InvalidSignature,
    InvalidEncoding,
    Expired,
}

impl core::fmt::Display for AuthError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidKey => write!(f, "invalid key"),
            Self::InvalidSignature => write!(f, "invalid signature"),
            Self::InvalidEncoding => write!(f, "invalid encoding"),
            Self::Expired => write!(f, "token expired"),
        }
    }
}
