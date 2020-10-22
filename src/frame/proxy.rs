use codec::{Decode, Encode};
use core::marker::PhantomData;
use substrate_subxt::{
    system::{System, SystemEventsDecoder},
    Encoded,
};
use sp_runtime::traits::Member;
use frame_support::Parameter;
use substrate_subxt_proc_macro::{module, Call, Store};

#[module]
pub trait Proxy: System {
    type ProxyType: Parameter + Member + Ord + PartialOrd + Default;
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct ProxyCall<'a, T: Proxy> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// The account that the proxy will make a call on behalf of.
    pub real: <T as System>::AccountId,
    /// Specify the exact proxy type to be used and checked for this call.
    pub force_proxy_type: Option<T::ProxyType>,
    /// The call to be made by the `real` account.
    pub call: &'a Encoded,
}
