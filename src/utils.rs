use solana_sdk::pubkey::Pubkey;

use crate::constants::TOKEN_PROGRAM;

/// Create an account using a randomly generated seed.
pub fn create_account_from_seed(base_pubkey: &Pubkey) -> (String, Pubkey) {
    // Generate a random seed
    let seed: String = String::from("Dexter");

    // Derive the new public key
    let to_pubkey: Pubkey = Pubkey::create_with_seed(base_pubkey, &seed, &TOKEN_PROGRAM)
        .expect("Failed to create public key with seed");

    (seed, to_pubkey)
}
