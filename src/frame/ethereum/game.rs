//! Relayer Game
use codec::{Decode, Encode};
use core::marker::PhantomData;
use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::{module, Store};

/// Ethereum Relay Pallet
#[module]
pub trait EthereumRelayerGame: System {
    /// Ethereum Pending Header
    type PendingHeader: 'static + Encode + Decode + Send + Default;
    /// Ethereum Relay Proposal
    type RelayProposal: 'static + Encode + Decode + Send + Default;
}

/// PendingHeaders Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct PendingHeadersStore<T: EthereumRelayerGame> {
    #[store(returns = Vec<T::PendingHeader>)]
    pub map: ([u8; 32], u64),
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

/// Relay Proposals Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ProposalsStore<T: EthereumRelayerGame> {
    #[store(returns = Vec<T::RelayProposal>)]
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
