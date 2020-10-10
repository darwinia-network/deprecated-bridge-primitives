//! Ethereum types
// mod confirmation;
mod ethash_proof;
mod header;
mod mmr_proof;

pub use self::{
    ethash_proof::{EthashProof, EthashProofJson},
    header::{EthHeader, EthHeaderJson, EthHeaderRPC},
    mmr_proof::{HeaderStuffs, MMRProof, MMRProofJson},
};
use codec::{Decode, Encode};

/// Ethereum HeaderThing
#[derive(Encode, Decode)]
pub struct HeaderThing {
    header: EthHeader,
    mmr_root: [u8; 32],
}

/// PendingHeader
pub type PendingHeader = (u64, u64, HeaderThing);
