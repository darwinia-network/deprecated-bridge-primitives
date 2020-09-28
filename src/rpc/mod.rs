//! RPC calls for primitives
#![cfg(feature = "rpc")]

mod eth;
use crate::result::Result;
use async_trait::async_trait;

/// RPC trait for bridge primitves
#[async_trait]
pub trait RPC {
    /// Block Header
    type Header;

    /// Geth header by block hash hex
    async fn get_header_by_hash(&self, block: &str) -> Result<Self::Header>
    where
        Self: Sized;

    /// Geth header by block number
    async fn get_header_by_number(&self, block: u64) -> Result<Self::Header>
    where
        Self: Sized;
}

/// Exports
pub use eth::EthereumRPC;
