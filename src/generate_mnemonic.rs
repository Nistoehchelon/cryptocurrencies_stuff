use bip39::{Mnemonic, WordCount};

pub fn generate_mnemonic(word_count: WordCount) -> String {
    Mnemonic::generate(word_count)
        .expect("valid word count")
        .to_string()
}
