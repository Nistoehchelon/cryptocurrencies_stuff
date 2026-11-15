use blake2::digest::consts::{U5, U32};
use blake2::{Blake2b, Digest};

use bip39::Mnemonic;
use ed25519_dalek::SigningKey;
use near_slip10::{BIP32Path, Curve, derive_key_from_path};
use std::str::FromStr;
use zeroize::Zeroizing;

use crate::derive_nano::GetAddrAtIndexError::{
    DerivationFailed, DerivationPathParseError, MnemonicParseError,
};

#[derive(Debug, Clone)]
pub enum GetAddrAtIndexError {
    MnemonicParseError(String),
    DerivationPathParseError(String),
    DerivationFailed(String),
}

const NANO_BASE32: &[u8; 32] = b"13456789abcdefghijkmnopqrstuwxyz";

pub fn get_address_at_index(mnemonic_str: &str, index: u32) -> Result<String, GetAddrAtIndexError> {
    let mnemonic =
        Mnemonic::from_str(mnemonic_str).map_err(|e| MnemonicParseError(e.to_string()))?;
    let seed = Zeroizing::new(mnemonic.to_entropy());

    type Blake2b256 = Blake2b<U32>;
    let mut hasher = Blake2b256::new();
    hasher.update(&*seed);
    hasher.update(&index.to_be_bytes());
    let hash = hasher.finalize();

    let mut private_bytes = Zeroizing::new([0u8; 32]);
    private_bytes.copy_from_slice(&hash);

    let signing_key = SigningKey::from_bytes(&*private_bytes);
    let public_key = signing_key.verifying_key();

    Ok(public_key_to_address(&public_key.to_bytes()))
}

fn public_key_to_address(public_key: &[u8; 32]) -> String {
    let mut buf = [0u8; 33];
    buf[0] = 0x06;
    buf[1..].copy_from_slice(public_key);

    let encoded = nano_base32_encode(&buf);
    let body = &encoded[1..];

    type Blake2b5 = Blake2b<U5>;
    let mut hasher = Blake2b5::new();
    hasher.update(public_key);
    let checksum_hash = hasher.finalize();

    let mut checksum = [0u8; 5];
    checksum.copy_from_slice(&checksum_hash);
    checksum.reverse();

    let checksum_encoded = nano_base32_encode(&checksum);

    format!("nano_{}{}", body, checksum_encoded)
}
fn nano_base32_encode(data: &[u8]) -> String {
    let mut out = String::new();
    let mut buffer: u32 = 0;
    let mut bits: u32 = 0;
    for &byte in data {
        buffer = (buffer << 8) | byte as u32;
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            let idx = ((buffer >> bits) & 0x1f) as usize;
            out.push(NANO_BASE32[idx] as char);
        }
    }
    if bits > 0 {
        let idx = ((buffer << (5 - bits)) & 0x1f) as usize;
        out.push(NANO_BASE32[idx] as char);
    }
    out
}
