use anyhow::Result;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    hash::Hash,
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_instruction,
    transaction::VersionedTransaction,
};
use spl_token::instruction::{close_account, initialize_account};

use crate::{
    constants::{TOKEN_PROGRAM, WSOL_TOKEN_PROGRAM},
    utils::create_account_from_seed,
};

pub async fn create_account_tx(
    amount_in: f64,
    compute_unit_price: u64,
    latest_blockhash: Hash,
    wallet: &Keypair,
) -> Result<VersionedTransaction> {
    // Compute Unit Limit - Try to adjust as close to the transaction cost as possible
    let cul = ComputeBudgetInstruction::set_compute_unit_limit(4_500); // 450 is the cost of a transfer

    // Compute Unit Price
    let cup = ComputeBudgetInstruction::set_compute_unit_price(compute_unit_price);

    // Set Loaded Account Data Size Limit
    let ladsl = ComputeBudgetInstruction::set_loaded_accounts_data_size_limit(135_000);

    // Create new account
    let fund_amount = (amount_in * 1_000_000_000.0) as u64;
    let (seed, mint_pubkey) = create_account_from_seed(&wallet.pubkey());
    let caws = system_instruction::create_account_with_seed(
        &wallet.pubkey(), // from_pubkey: sender wallet
        &mint_pubkey,     // to_pubkey: the account generated from the seed
        &wallet.pubkey(), // base: sender wallet
        &seed,            // seed: the seed string used to generate the account
        fund_amount,      // lamports: amount of lamports to fund the account with
        165,              // space: 165 in a Raydium swap
        &TOKEN_PROGRAM,   // owner: token program
    );

    // Initialize Account
    let ia = initialize_account(
        &TOKEN_PROGRAM,      // token_prograM: token program
        &mint_pubkey,        // account_pubkey: the account generated from the seed (WSOL account)
        &WSOL_TOKEN_PROGRAM, // mint_pubkey: wsol token program
        &wallet.pubkey(),    // owner_pubkey: sender wallet
    )?;

    // Create Transaction Instructions
    let instructions = vec![cup, cul, caws, ia];

    // Create Transaction Message
    let message = Message::try_compile(&wallet.pubkey(), &instructions, &vec![], latest_blockhash)?;

    // Create a VersionedTransaction
    let versioned_transaction = VersionedTransaction {
        message: VersionedMessage::V0(message.clone()), // Clone the message
        signatures: vec![wallet.sign_message(&message.serialize())],
    };

    Ok(versioned_transaction)
}

pub async fn close_account_tx(
    account: &Pubkey,
    compute_unit_price: u64,
    latest_blockhash: Hash,
    wallet: &Keypair,
) -> Result<VersionedTransaction> {
    // Compute Unit Limit - Try to adjust as close to the transaction cost as possible
    let cul = ComputeBudgetInstruction::set_compute_unit_limit(4_000); // 450 is the cost of a transfer

    // Compute Unit Price
    let cup = ComputeBudgetInstruction::set_compute_unit_price(compute_unit_price);

    // Set Loaded Account Data Size Limit
    let ladsl = ComputeBudgetInstruction::set_loaded_accounts_data_size_limit(135_000);

    // Close Account Instruction
    let ca = close_account(
        &TOKEN_PROGRAM,
        &account,
        &wallet.pubkey(),
        &wallet.pubkey(),
        &[&wallet.pubkey()],
    )?;

    // Create Transaction Instructions
    let instructions = vec![cup, cul, ca];

    // Create Transaction Message
    let message = Message::try_compile(&wallet.pubkey(), &instructions, &[], latest_blockhash)?;

    // Create VersionedTransaction
    let versioned_transaction: VersionedTransaction = VersionedTransaction {
        message: VersionedMessage::V0(message.clone()), // Clone the message
        signatures: vec![wallet.sign_message(&message.serialize())],
    };

    Ok(versioned_transaction)
}
