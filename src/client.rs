use std::sync::Arc;

use solana_client::{client_error::ClientError, nonblocking::rpc_client::RpcClient};
use solana_sdk::commitment_config::CommitmentConfig;

pub fn create_rpc_client(
    rpc_url: String,
    commitment: CommitmentConfig,
) -> Result<Arc<RpcClient>, ClientError> {
    Ok(Arc::new(RpcClient::new_with_commitment(
        rpc_url, commitment,
    )))
}
