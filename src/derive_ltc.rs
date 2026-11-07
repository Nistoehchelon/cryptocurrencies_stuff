use bech32::{self, Hrp};
use bip32::ChildNumber;
use bip39::Mnemonic;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use std::str::FromStr;


/// Derives a BIP84 (native SegWit / P2WPKH) Litecoin address.
///
/// Derivation path: m/84'/2'/0'/0/{index}
///   - Purpose  84'  — BIP84 (native SegWit)
///   - Coin     2'   — Litecoin (SLIP-0044)
///   - Account  0'
///   - Change   0    — external chain
///   - Index    {index}
///
/// The resulting address uses the "ltc" bech32 HRP (Litecoin mainnet).
pub fn derive_ltc_bip84_address(master_seed: &str, index: u64) -> Result<String, String> {
    // 1. Parse mnemonic and derive BIP39 seed (no passphrase)
    let mnemonic = Mnemonic::from_str(master_seed).map_err(|e| format!("Invalid mnemonic: {e}"))?;
    let seed = mnemonic.to_seed("");

    // 2. Build the master extended private key from the 64-byte seed
    let master =
        bip32::XPrv::new(seed).map_err(|e| format!("Master key derivation failed: {e}"))?;

    // 3. Derive m/84'/2'/0'/0/{index} step by step
    let child = master
        .derive_child(ChildNumber::new(84, true).unwrap()) // 84'  purpose
        .map_err(|e| format!("Derive 84' failed: {e}"))?
        .derive_child(ChildNumber::new(2, true).unwrap()) // 2'   coin (LTC)
        .map_err(|e| format!("Derive 2' failed: {e}"))?
        .derive_child(ChildNumber::new(0, true).unwrap()) // 0'   account
        .map_err(|e| format!("Derive 0' failed: {e}"))?
        .derive_child(ChildNumber::new(0, false).unwrap()) // 0    external chain
        .map_err(|e| format!("Derive 0 failed: {e}"))?
        .derive_child(ChildNumber::new(index as u32, false).unwrap()) // index
        .map_err(|e| format!("Derive index {index} failed: {e}"))?;

    // 4. Compressed public key (33 bytes)
    let pubkey_bytes = child.public_key().to_bytes();

    // 5. Hash160 = RIPEMD160(SHA256(pubkey))
    let sha256 = Sha256::digest(pubkey_bytes);
    let hash160 = Ripemd160::digest(sha256);

    // 6. Encode as bech32 P2WPKH ("ltc1q…")
    // bech32 v0.11: encode takes an Hrp and raw witness program bytes;
    // use encode_v0 (segwit v0) which handles the witness version + base32 internally.
    let hrp = Hrp::parse("ltc").map_err(|e| format!("Invalid HRP: {e}"))?;
    bech32::segwit::encode_v0(hrp, &hash160).map_err(|e| format!("bech32 encode failed: {e}"))
}
