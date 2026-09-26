// Copyright (C) 2026 AnalyseDeCircuit
// SPDX-License-Identifier: GPL-3.0-only

use base64::Engine as _;
use minisign_verify::{PublicKey, Signature};

use crate::NativeUpdateError;

pub const OXIDETERM_UPDATER_PUBKEY: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IDM2RTE5RDY5OTJCNTdFQkIKUldTN2ZyV1NhWjNoTnJFZ3p6T2s0WEtNaTVTWUhpUW1LdnRjTlpEaGZsTTAzaTJOSll1bVhPem4K";

pub fn verify_minisign_signature(
    data: &[u8],
    release_signature: &str,
) -> Result<(), NativeUpdateError> {
    verify_minisign_signature_with_key(data, release_signature, OXIDETERM_UPDATER_PUBKEY)
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
        validate_minisign_public_key(OXIDETERM_UPDATER_PUBKEY).unwrap();
        assert!(
            verify_minisign_signature_with_key(b"test", &signature, OXIDETERM_UPDATER_PUBKEY,)
                .is_err()
        );
        assert!(verify_minisign_signature_with_key(b"tampered", &signature, &key).is_err());
    }
}
