//! Ethereum Relay

use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::module;

/// Ethereum Relay Pallet
#[module]
pub trait EthereumRelay: System {}
