pub mod atomic_unit;
pub mod currencies;
pub mod derive_ltc;
pub mod derive_nano;
pub mod generate_mnemonic;

#[cfg(test)]
mod tests {
    use crate::derive_nano::get_address_at_index;

    use super::*;

    const MNEMONIC: &str = "edge defense waste choose enrich upon flee junk siren film clown \
                            finish luggage leader kid quick brick print evidence swap drill \
                            paddle truly occur";

    #[test]
    fn address_has_valid_shape() {
        let addr = get_address_at_index(MNEMONIC, 0).unwrap();
        // "nano_" + 60 chars
        assert_eq!(addr.len(), 5 + 60, "unexpected length: {addr}");
        assert!(addr.starts_with("nano_"), "missing prefix: {addr}");

        // Body must use ONLY the Nano alphabet.
        const ALPHABET: &str = "13456789abcdefghijkmnopqrstuwxyz";
        for (i, c) in addr["nano_".len()..].chars().enumerate() {
            assert!(
                ALPHABET.contains(c),
                "invalid char {c:?} at position {i} in {addr}"
            );
        }
    }

    #[test]
    fn derivation_is_deterministic() {
        let a = get_address_at_index(MNEMONIC, 7).unwrap();
        let b = get_address_at_index(MNEMONIC, 7).unwrap();
        assert_eq!(a, b, "same input must yield same address");
    }

    #[test]
    fn different_indices_yield_different_addresses() {
        let a = get_address_at_index(MNEMONIC, 0).unwrap();
        let b = get_address_at_index(MNEMONIC, 1).unwrap();
        let c = get_address_at_index(MNEMONIC, 2).unwrap();
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(a, c);
    }

    #[test]
    fn invalid_mnemonic_is_rejected() {
        assert!(get_address_at_index("not a valid mnemonic at all", 0).is_err());
    }
    #[tracing_test::traced_test]
    #[test]
    fn wallets() {
        let mnemonic = std::env::var("MNEMONIC").unwrap();
        let equality_addr = std::env::var("ADDR").unwrap();
        let index = std::env::var("INDEX").unwrap().parse().unwrap();
        let gened_addr = get_address_at_index(mnemonic.as_str(), index).unwrap();
        tracing::debug!("gened_addr: {gened_addr}, equality_addr: {equality_addr}");
        assert!(gened_addr == equality_addr);
    }
}
