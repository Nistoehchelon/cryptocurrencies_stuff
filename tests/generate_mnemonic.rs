use cryptocurrencies_stuff::generate_mnemonic;

#[test]
pub fn test_gen_mne() {
    println!(
        "{}",
        generate_mnemonic::generate_mnemonic(bip39::WordCount::Words12)
    );
}
