use bip39::Mnemonic;
use blake2::{Blake2b, Digest};
use data_encoding::BASE32_NOPAD;
use ed25519_dalek::SigningKey;
use near_slip10::{Curve, BIP32Path, derive_key_from_path};
use std::str::FromStr;
use zeroize::Zeroizing;

use crate::derive_nano::GetAddrAtIndexError::{DerivationFailed, DerivationPathParseError, MnemonicParseError};

pub enum GetAddrAtIndexError {
    MnemonicParseError(String),
    DerivationPathParseError(String),
    DerivationFailed(String)
}

const NANO_BASE32: &str = "13456789abcdefghijkmnopqrstuwxyz";

pub fn get_address_at_index(mnemonic_str: &str, index: u32) -> Result<String, GetAddrAtIndexError> {
let mnemonic = Mnemonic::from_str(mnemonic_str).map_err(|e| MnemonicParseError(e.to_string()))?;
let seed = Zeroizing::new(mnemonic.to_seed(""));

let path = BIP32Path::from_str(&format!("m/44'/165'/{index}")).map_err(|e| DerivationPathParseError(e.to_string()))?;

let derived = derive_key_from_path(&*seed, Curve::Ed25519, &path).map_err(|e| DerivationFailed(e.to_string()))?;

let private_bytes = Zeroizing::new(derived.key);
let signing_key = SigningKey::from_bytes(&*private_bytes);
let public_key = signing_key.verifying_key();

Ok(public_key_to_address(&public_key.to_bytes()))


}

fn public_key_to_address(public_key: &[u8; 32]) -> String {
    let mut encoded_bytes = [0u8; 33];
    encoded_bytes[0] = 0x06;
}