//! Ethereum RPC calls
mod header;

use crate::{chain::eth::EthHeader, result::Result, rpc::RPC};
use async_trait::async_trait;
use reqwest::Client;

/// Ethereum rpc set
pub struct EthereumRPC<'r> {
    pub client: &'r Client,
    pub rpc: &'r str,
}

impl EthereumRPC {
    /// New EthereumRPC
    pub fn new(client: &str, rpc: &str) -> Self {
        EthereumRPC { client, rpc }
    }
}

#[async_trait]
impl<'r> RPC for EthereumRPC<'r> {
    type Header = EthHeader;

    async fn get_header_by_number(&self, block: u64) -> Result<Self::Header> {
        Ok(header::EthHeaderRPCResp::get(self.client, self.rpc, block)
            .await?
            .result
            .into())
    }

    async fn get_header_by_hash(&self, block: &str) -> Result<Self::Header> {
        Ok(
            header::EthHeaderRPCResp::get_by_hash(self.client, self.rpc, block)
                .await?
                .result
                .into(),
        )
    }
}
