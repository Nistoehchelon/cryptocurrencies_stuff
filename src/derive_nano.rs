//! Derive a Nano address from a BIP39 mnemonic + account index.
//!
//! Cargo.toml dependencies:
//!
//! [dependencies]
//! bip39 = "2"
//! blake2 = "0.10"
//! curve25519-dalek = "4"
//! hmac = "0.12"
//! sha2 = "0.10"
//!
//! Two derivation schemes exist in the wild:
//!
//! 1. `nano_address_from_mnemonic`  (Nault / Natrium / official Nano wallets)
//!    The 24-word mnemonic's entropy IS the 32-byte Nano seed.
//!    private_key = Blake2b-256(seed || index_u32_be)
//!
//! 2. `nano_address_from_mnemonic_ledger` (Ledger / BIP44 style)
//!    BIP39 seed (PBKDF2) -> SLIP-0010 ed25519 at m/44'/165'/index'
//!
//! Pick the one matching the wallet the mnemonic came from.

use bip39::Mnemonic;
use blake2::digest::consts::U32;
use blake2::digest::{Update, VariableOutput};
use blake2::{Blake2b, Blake2b512, Blake2bVar, Digest};
use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;
use curve25519_dalek::scalar::Scalar;
//use hmac::{Hmac, Mac};
//use sha2::Sha512;

use crate::derive_nano::NanoAddressFromMnemonicError::{EntropyToSeedError, MnemonicParseError};

const NANO_ALPHABET: &[u8; 32] = b"13456789abcdefghijkmnopqrstuwxyz";

#[derive(Debug, Clone)]
pub enum NanoAddressFromMnemonicError {
    MnemonicParseError(String),
    EntropyToSeedError(String),
}

/// Nault/Natrium-style: mnemonic entropy is the Nano seed (needs 24 words).
pub fn nano_address_from_mnemonic(
    mnemonic: &str,
    index: u32,
) -> Result<String, NanoAddressFromMnemonicError> {
    let m = Mnemonic::parse(mnemonic)
        .map_err(|e| MnemonicParseError(format!("invalid mnemonic: {e}")))?;
    let entropy = m.to_entropy();
    let seed: [u8; 32] = entropy.as_slice().try_into().map_err(|e| {
        EntropyToSeedError(format!(
            "{e:#?}. Nano seeds need a 24-word mnemonic (32 bytes of entropy)"
        ))
    })?;

    let mut hasher = Blake2b::<U32>::new();
    Digest::update(&mut hasher, seed);
    Digest::update(&mut hasher, index.to_be_bytes());
    let private_key: [u8; 32] = hasher.finalize().into();

    Ok(address_from_public_key(&public_key_from_private(
        &private_key,
    )))
}

/// Nano-flavoured ed25519: the secret is hashed with Blake2b-512 (not SHA-512).
fn public_key_from_private(private_key: &[u8; 32]) -> [u8; 32] {
    let h = Blake2b512::digest(private_key);
    let mut a = [0u8; 32];
    a.copy_from_slice(&h[..32]);
    a[0] &= 248;
    a[31] &= 127;
    a[31] |= 64;
    let scalar = Scalar::from_bytes_mod_order(a);
    (ED25519_BASEPOINT_POINT * scalar).compress().to_bytes()
}

/// Base32 encode with the Nano alphabet. `pad_bits` zero bits are prepended.
fn nano_base32(bytes: &[u8], pad_bits: u32) -> String {
    let mut out = String::new();
    let mut acc: u32 = 0;
    let mut nbits: u32 = pad_bits;
    for &b in bytes {
        acc = (acc << 8) | b as u32;
        nbits += 8;
        while nbits >= 5 {
            let idx = (acc >> (nbits - 5)) & 0x1f;
            out.push(NANO_ALPHABET[idx as usize] as char);
            nbits -= 5;
        }
        acc &= (1u32 << nbits) - 1;
    }
    out
}

fn address_from_public_key(pk: &[u8; 32]) -> String {
    // 260 bits (4 zero pad + 256) -> 52 chars
    let body = nano_base32(pk, 4);

    // 5-byte Blake2b checksum, byte-reversed -> 8 chars
    let mut hasher = Blake2bVar::new(5).expect("valid output size");
    hasher.update(pk);
    let mut chk = [0u8; 5];
    hasher
        .finalize_variable(&mut chk)
        .expect("buffer size matches");
    chk.reverse();
    let checksum = nano_base32(&chk, 0);

    format!("nano_{body}{checksum}")
}

// TODO: Fix if I ever want this, currently broken.
// /// Ledger-style: BIP39 seed + SLIP-0010 ed25519, path m/44'/165'/index'.
// /// Works with 12 or 24 words. `passphrase` is the optional BIP39 passphrase.
// pub fn nano_address_from_mnemonic_ledger(
//     mnemonic: &str,
//     passphrase: &str,
//     index: u32,
// ) -> Result<String, String> {
//     type HmacSha512 = Hmac<Sha512>;

//     let m = Mnemonic::parse(mnemonic).map_err(|e| format!("invalid mnemonic: {e}"))?;
//     let seed = m.to_seed(passphrase);

//     let hmac = |key: &[u8], data: &[u8]| -> ([u8; 32], [u8; 32]) {
//         let mut mac = HmacSha512::new_from_slice(key).unwrap();
//         mac.update(data);
//         let out = mac.finalize().into_bytes();
//         let (mut k, mut c) = ([0u8; 32], [0u8; 32]);
//         k.copy_from_slice(&out[..32]);
//         c.copy_from_slice(&out[32..]);
//         (k, c)
//     };

//     let (mut key, mut chain) = hmac(b"ed25519 seed", &seed);
//     for i in [44u32, 165, index] {
//         let mut data = Vec::with_capacity(37);
//         data.push(0u8);
//         data.extend_from_slice(&key);
//         data.extend_from_slice(&(i | 0x8000_0000).to_be_bytes());
//         let (k, c) = hmac(&chain, &data);
//         key = k;
//         chain = c;
//     }

//     Ok(address_from_public_key(&public_key_from_private(&key)))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_seed_index_zero() {
        // Known Nano test vector: seed = 32 zero bytes, index 0.
        let sk = {
            let mut h = Blake2b::<U32>::new();
            Digest::update(&mut h, [0u8; 32]);
            Digest::update(&mut h, 0u32.to_be_bytes());
            let out: [u8; 32] = h.finalize().into();
            out
        };
        let addr = address_from_public_key(&public_key_from_private(&sk));
        assert_eq!(
            addr,
            "nano_3i1aq1cchnmbn9x5rsbap8b15akfh7wj7pwskuzi7ahz8oq6cobd99d4r3b7"
        );
    }
    #[test]
    #[tracing_test::traced_test]
    fn wallet() {
        let mnemonic = std::env::var("MNEMONIC").unwrap();
        let index: u32 = std::env::var("INDEX").unwrap().parse().unwrap();
        let mut i = 0;
        while i < 10 {
            let addr = nano_address_from_mnemonic(&mnemonic, index + i).unwrap();
            tracing::debug!("Addr: {addr}");
            i = i + 1;
        }
    }
}
