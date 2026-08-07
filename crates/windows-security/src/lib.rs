//! Narrow, audited adapter for Windows DPAPI user-scope protection.
//!
//! Unsafe code is confined to the two Win32 calls and copying/freeing their
//! system-owned output buffers. No machine-scope flag is ever accepted.

#[cfg(not(windows))]
compile_error!("second-brain-windows-security supports Windows only");

use std::ptr;

use windows::Win32::Foundation::{HLOCAL, LocalFree};
use windows::Win32::Security::Cryptography::{
    CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptProtectData, CryptUnprotectData,
};
use windows::core::PCWSTR;

/// Failure returned by the fail-closed DPAPI boundary.
#[derive(Debug)]
pub enum SecretError {
    /// Input or purpose is empty or too large for DPAPI.
    InvalidInput,
    /// Windows refused protection or recovery.
    Windows(windows::core::Error),
    /// The protected payload did not match the expected application envelope.
    InvalidEnvelope,
}

impl From<windows::core::Error> for SecretError {
    fn from(value: windows::core::Error) -> Self {
        Self::Windows(value)
    }
}

const ENVELOPE: &[u8] = b"SBOS-SECRET-V1\0";

/// Protects bytes for the current Windows user with versioned purpose entropy.
///
/// # Errors
/// Fails closed for empty/oversized values or any Windows error.
pub fn protect(plaintext: &[u8], purpose: &[u8]) -> Result<Vec<u8>, SecretError> {
    let mut envelope = Vec::with_capacity(ENVELOPE.len() + plaintext.len());
    envelope.extend_from_slice(ENVELOPE);
    envelope.extend_from_slice(plaintext);
    transform(&envelope, purpose, true)
}

/// Recovers bytes for the current Windows user and validates the envelope.
///
/// # Errors
/// Fails closed for another user, wrong purpose, corruption, or invalid envelope.
pub fn unprotect(ciphertext: &[u8], purpose: &[u8]) -> Result<Vec<u8>, SecretError> {
    let recovered = transform(ciphertext, purpose, false)?;
    recovered
        .strip_prefix(ENVELOPE)
        .map(<[u8]>::to_vec)
        .ok_or(SecretError::InvalidEnvelope)
}

fn blob(bytes: &[u8]) -> Result<CRYPT_INTEGER_BLOB, SecretError> {
    Ok(CRYPT_INTEGER_BLOB {
        cbData: u32::try_from(bytes.len()).map_err(|_| SecretError::InvalidInput)?,
        pbData: bytes.as_ptr().cast_mut(),
    })
}

fn transform(input: &[u8], purpose: &[u8], protecting: bool) -> Result<Vec<u8>, SecretError> {
    if input.is_empty() || purpose.is_empty() {
        return Err(SecretError::InvalidInput);
    }
    let input = blob(input)?;
    let entropy = blob(purpose)?;
    let mut output = CRYPT_INTEGER_BLOB::default();
    // SAFETY: input and entropy point to live slices for the duration of the call;
    // output is initialized by DPAPI and released exactly once with LocalFree.
    unsafe {
        if protecting {
            CryptProtectData(
                &raw const input,
                PCWSTR::null(),
                Some(&raw const entropy),
                None,
                None,
                CRYPTPROTECT_UI_FORBIDDEN,
                &raw mut output,
            )?;
        } else {
            CryptUnprotectData(
                &raw const input,
                None,
                Some(&raw const entropy),
                None,
                None,
                CRYPTPROTECT_UI_FORBIDDEN,
                &raw mut output,
            )?;
        }
        if output.pbData.is_null() || output.cbData == 0 {
            return Err(SecretError::InvalidEnvelope);
        }
        let value = std::slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec();
        let freed = LocalFree(Some(HLOCAL(output.pbData.cast())));
        debug_assert_eq!(freed.0, ptr::null_mut());
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_is_user_and_purpose_bound() {
        let protected = protect(b"local-secret", b"second-brain/test/v1").expect("protect");
        assert_ne!(protected, b"local-secret");
        assert_eq!(
            unprotect(&protected, b"second-brain/test/v1").expect("recover"),
            b"local-secret"
        );
        assert!(unprotect(&protected, b"second-brain/other/v1").is_err());
    }
}
