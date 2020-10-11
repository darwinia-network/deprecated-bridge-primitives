//! Ethereum receipt
use crate::{bytes, hex};
use codec::Encode;

/// Redeem for
#[derive(Clone, Debug, Encode, PartialEq, Eq)]
pub enum RedeemFor {
    /// Redeem for token
    Token,
    /// Redeem for deopsit
    Deposit,
}

impl Default for RedeemFor {
    fn default() -> Self {
        RedeemFor::Token
    }
}

/// Ethereum Receipt Proof
#[derive(Clone, Debug, Default, Encode, PartialEq, Eq)]
pub struct EthereumReceiptProof {
    /// Proof index
    pub index: u64,
    /// Receipt Proof
    pub proof: Vec<u8>,
    /// Ethereum Header Hash
    pub header_hash: [u8; 32],
}

/// Ethereum Receipt Proof Json
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EthereumReceiptProofJson {
    /// Proof index
    pub index: String,
    /// Receipt Proof
    pub proof: String,
    /// Ethereum Header Hash
    pub header_hash: String,
}

impl Into<EthereumReceiptProofJson> for EthereumReceiptProof {
    fn into(self) -> EthereumReceiptProofJson {
        EthereumReceiptProofJson {
            index: format!("{:x}", self.index),
            proof: hex!(self.proof),
            header_hash: hex!(self.header_hash.to_vec()),
        }
    }
}

impl Into<EthereumReceiptProof> for EthereumReceiptProofJson {
    fn into(self) -> EthereumReceiptProof {
        let index = if self.index.starts_with("0x") {
            "00"
        } else {
            &self.index[2..]
        };

        EthereumReceiptProof {
            index: u64::from_str_radix(index, 16).unwrap_or(0),
            proof: bytes!(self.proof.as_str()),
            header_hash: bytes!(self.header_hash.as_str(), 32),
        }
    }
}
