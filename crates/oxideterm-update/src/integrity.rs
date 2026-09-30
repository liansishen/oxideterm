// Copyright (C) 2026 AnalyseDeCircuit
// SPDX-License-Identifier: GPL-3.0-only

use base64::Engine as _;
use minisign_verify::{PublicKey, Signature};

use crate::NativeUpdateError;

pub const OXIDETERM_UPDATER_PUBKEY: Option<&str> = option_env!("OXIDETERM_UPDATER_PUBKEY");

pub fn verify_minisign_signature(
    data: &[u8],
    release_signature: &str,
) -> Result<(), NativeUpdateError> {
    let public_key = OXIDETERM_UPDATER_PUBKEY.ok_or_else(|| {
        NativeUpdateError::Integrity("updater public key is not configured".to_string())
    })?;
    verify_minisign_signature_with_key(data, release_signature, public_key)
}

pub fn configured_updater_public_key() -> Result<&'static str, NativeUpdateError> {
    configured_updater_public_key_from(OXIDETERM_UPDATER_PUBKEY)
}

fn configured_updater_public_key_from(
    public_key: Option<&'static str>,
) -> Result<&'static str, NativeUpdateError> {
    let public_key = public_key.ok_or_else(|| {
        NativeUpdateError::Integrity("updater public key is not configured".to_string())
    })?;
    validate_minisign_public_key(public_key)?;
    Ok(public_key)
}

pub fn validate_minisign_public_key(public_key_base64: &str) -> Result<(), NativeUpdateError> {
    let decoded = base64_to_string(public_key_base64)?;
    PublicKey::decode(&decoded)
        .map(|_| ())
        .map_err(|error| NativeUpdateError::Integrity(format!("decode public key failed: {error}")))
}
pub fn verify_minisign_signature_with_key(
    data: &[u8],
    release_signature: &str,
    public_key_base64: &str,
) -> Result<(), NativeUpdateError> {
    let pub_key_decoded = base64_to_string(public_key_base64)?;
    let public_key = PublicKey::decode(&pub_key_decoded).map_err(|error| {
        NativeUpdateError::Integrity(format!("decode public key failed: {error}"))
    })?;
    let signature_decoded = base64_to_string(release_signature)?;
    let signature = Signature::decode(&signature_decoded).map_err(|error| {
        NativeUpdateError::Integrity(format!("decode release signature failed: {error}"))
    })?;
    public_key.verify(data, &signature, true).map_err(|error| {
        NativeUpdateError::Integrity(format!("signature verification failed: {error}"))
    })?;
    Ok(())
}

fn base64_to_string(value: &str) -> Result<String, NativeUpdateError> {
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(value)
        .map_err(|error| NativeUpdateError::Integrity(format!("base64 decode failed: {error}")))?;
    std::str::from_utf8(&decoded)
        .map(str::to_string)
        .map_err(|_| NativeUpdateError::Integrity("invalid utf8 in signature".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUBLIC_KEY: &str = "untrusted comment: minisign public key E7620F1842B4E81F\nRWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3\n";
    const SIGNATURE: &str = "untrusted comment: signature from minisign secret key\nRWQf6LRCGA9i59SLOFxz6NxvASXDJeRtuZykwQepbDEGt87ig1BNpWaVWuNrm73YiIiJbq71Wi+dP9eKL8OC351vwIasSSbXxwA=\ntrusted comment: timestamp:1555779966\tfile:test\nQtKMXWyYcwdpZAlPF7tE2ENJkRd1ujvKjlj1m9RtHTBnZPa5WKU5uWRs5GoP5M/VqE81QFuMKI5k/SfNQUaOAA==";

    fn outer_base64(value: &str) -> String {
        base64::engine::general_purpose::STANDARD.encode(value)
    }

    #[test]
    fn verifies_custom_signature_only_with_the_selected_key() {
        let key = outer_base64(PUBLIC_KEY);
        let signature = outer_base64(SIGNATURE);
        verify_minisign_signature_with_key(b"test", &signature, &key)
            .unwrap_or_else(|error| panic!("{error:?}"));
        if let Some(key) = OXIDETERM_UPDATER_PUBKEY {
            validate_minisign_public_key(key).unwrap();
            assert!(verify_minisign_signature_with_key(b"test", &signature, key).is_err());
        }
        assert!(verify_minisign_signature_with_key(b"tampered", &signature, &key).is_err());
    }

    #[test]
    fn configured_public_key_fails_closed_when_missing_or_invalid() {
        assert!(configured_updater_public_key_from(None).is_err());
        assert!(configured_updater_public_key_from(Some("invalid")).is_err());
    }
}
