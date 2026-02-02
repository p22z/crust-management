use std::{env, str::FromStr};

use anyhow::{anyhow, Result};
use dotenv::dotenv;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    pubkey::Pubkey,
    signature::Keypair,
};
use solana_transaction_status::UiTransactionEncoding;

use crusty_management::{
    client::create_rpc_client,
    transactions::{close_account_tx, create_account_tx},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    // Get private key from .env
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");

    // Create Keypair from private key
    let keypair = Keypair::from_base58_string(&private_key);

    // Get RPC url from .env
    let rpc = env::var("RPC").expect("PRIVATE_KEY must be set");

    // RPC Client
    let client_helius = create_rpc_client(rpc, CommitmentConfig::confirmed())?;

    // Recent Blockhash
    let recent_blockhash = client_helius.get_latest_blockhash().await?;

    // WSOl account that will be used for trading on Raydium
    let wsol_account = Pubkey::from_str("75VUgJ6Zp8e3MUtf9Kwypfyh9bCxyR7mmC8YPF2c9D66")?;

    let transaction_close = close_account_tx(&wsol_account, 1, recent_blockhash, &keypair).await?;
    let transaction_create = create_account_tx(0.05, 100_000, recent_blockhash, &keypair).await?;

    // Send Transaction Config
    let tx_config = RpcSendTransactionConfig {
        skip_preflight: false,
        preflight_commitment: Some(CommitmentLevel::Confirmed),
        encoding: Some(UiTransactionEncoding::Base64),
        max_retries: Some(5),
        min_context_slot: None,
    };
    // Send the transaction
    match client_helius
        .send_and_confirm_transaction_with_spinner_and_config(
            &transaction_create,
            CommitmentConfig::confirmed(),
            tx_config,
        )
        .await
    {
        Ok(sig) => {
            println!("Tx signature: {:?}", sig);
            Ok(())
        }
        Err(e) => Err(anyhow!("Error sending transaction: {:?}", e)),
    }
}
