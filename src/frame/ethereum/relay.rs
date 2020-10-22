//! Ethereum Relay
use crate::chain::ethereum::{EthereumRelayHeaderParcel, EthereumRelayProofs};
use codec::Encode;
use core::marker::PhantomData;
use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::{module, Call, Store};

/// Ethereum Relay Pallet
#[module]
pub trait EthereumRelay: System {}

/// Affirm Call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct Affirm<T: EthereumRelay> {
    ethereum_relay_header_parcel: EthereumRelayHeaderParcel,
    ethereum_relay_proofs: Option<EthereumRelayProofs>,
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

/// PendingHeaders Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ConfirmedBlockNumbers<T: EthereumRelay> {
    #[store(returns = Vec<u64>)]
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

// /// Submit proposal call
// #[derive(Clone, Debug, PartialEq, Call, Encode)]
// pub struct SubmitProposal<T: EthereumRelay> {
//     /// Ethereum Headerthings
//     pub proposal: Vec<EthereumRelayHeaderParcel>,
//     /// Runtime marker
//     pub _runtime: PhantomData<T>,
// }

/// Approve pending header call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ApprovePendingRelayHeaderParcel<T: EthereumRelay> {
    /// pending block number
    pub pending: u64,
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

/// Reject pending header call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct RejectPendingRelayHeaderParcel<T: EthereumRelay> {
    /// pending block number
    pub pending: u64,
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
