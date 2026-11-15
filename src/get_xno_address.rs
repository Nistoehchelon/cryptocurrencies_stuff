// Made by Claude
// Previous version was made by Deepseek but I could not get it to work
use bip39::Mnemonic;
use blake2::{Blake2b, Blake2b512, Digest};
use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;
use curve25519_dalek::scalar::Scalar;
//use hmac::{Hmac, Mac};
//use sha2::Sha512;

use crate::get_xno_address::NanoAddressFromMnemonicError::{
    EntropyToSeedError, MnemonicParseError,
};

const NANO_ALPHABET: &[u8; 32] = b"13456789abcdefghijkmnopqrstuwxyz";

#[derive(Debug, Clone)]
pub enum NanoAddressFromMnemonicError {
    MnemonicParseError(String),
    EntropyToSeedError(String),
}

/// Nault/Natrium-style: mnemonic entropy is the Nano seed (needs 24 words).
pub fn get_address_from_mnemonic_and_index(
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

    let sk = private_key_from_seed(&seed, index);

    Ok(address_from_public_key(&public_key_from_private(
        &sk,
    )))
}

fn private_key_from_seed(seed: &[u8; 32], index: u32) -> [u8; 32] {
    let mut hasher = Blake2b::<blake2::digest::consts::U32>::new();
    Digest::update(&mut hasher, seed);
    Digest::update(&mut hasher, index.to_be_bytes());
    let out = hasher.finalize();
    let mut sk = [0u8; 32];
    sk.copy_from_slice(&out);
    sk
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
    let digest = Blake2b::<blake2::digest::consts::U5>::digest(pk);
    let mut chk = [0u8; 5];
    chk.copy_from_slice(&digest);
    chk.reverse();
    let checksum = nano_base32(&chk, 0);

    format!("nano_{body}{checksum}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_seed_index_zero() {
        // Known Nano test vector: seed = 32 zero bytes, index 0.
        let sk = {
            let mut h = Blake2b::<blake2::digest::consts::U32>::new();
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
            let addr = get_address_from_mnemonic_and_index(&mnemonic, index + i).unwrap();
            tracing::debug!("Addr: {addr}");
            i += 1;
        }
    }
}
