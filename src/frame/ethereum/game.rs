//! Relayer Game
use crate::chain::{eth::HeaderThing, RelayProposal};
use codec::Encode;
use core::marker::PhantomData;
use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::{module, Store};

/// Ethereum Relay Proposal
pub type RelayProposalT = RelayProposal<[u8; 32], u64, HeaderThing, [u8; 32]>;

/// Ethereum Relay Pallet
#[module]
pub trait EthereumRelayerGame: System {}

/// PendingHeaders Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct PendingHeaders<T: EthereumRelayerGame> {
    #[store(returns = Vec<(u64, u64, HeaderThing)>)]
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

/// Relay Proposals Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct Proposals<T: EthereumRelayerGame> {
    #[store(returns = Vec<RelayProposalT>)]
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
