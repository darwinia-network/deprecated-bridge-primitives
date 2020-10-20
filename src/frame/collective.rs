//! Darwinia Collective

use codec::{Decode, Encode};
use core::marker::PhantomData;
use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::{module, Store};

/// The subset of the `frame_sudo::Trait` that a client must implement.
#[module]
pub trait Council: System {}

/// Get the sudo AccountId
#[derive(Clone, Debug, Eq, PartialEq, Store, Decode, Encode)]
pub struct Members<T: Council> {
    #[store(returns = Vec<<T as System>::AccountId>)]
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
}
