//! Ethereum Relay Proof
use crate::chain::ethereum::{EthashProof, EthashProofJson};
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// Darwinia eth relay header thing
#[derive(Clone, Debug, Decode, Encode, Default, PartialEq, Eq)]
pub struct EthereumRelayProofs {
    ethash_proof: Vec<EthashProof>,
    mmr_proof: Vec<[u8; 32]>,
}

/// Shadow Proposal Response
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EthereumRelayProofsJson {
    ethash_proof: Vec<EthashProofJson>,
    mmr_proof: Vec<String>,
}

impl Into<EthereumRelayProofs> for EthereumRelayProofsJson {
    fn into(self) -> EthereumRelayProofs {
        EthereumRelayProofs {
            ethash_proof: self
                .ethash_proof
                .iter()
                .map(|p| Into::<EthashProof>::into(p.to_owned()))
                .collect(),
            mmr_proof: self
                .mmr_proof
                .iter()
                .map(|p| bytes!(p.as_str(), 32))
                .collect(),
        }
    }
}
