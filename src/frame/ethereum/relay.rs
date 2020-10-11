//! Ethereum Relay
use codec::Encode;
use core::marker::PhantomData;
use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::{module, Store};

/// Ethereum Relay Pallet
#[module]
pub trait EthereumRelay: System {}

/// PendingHeaders Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ConfirmedBlockNumbers<T: EthereumRelay> {
    #[store(returns = Vec<u64>)]
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
