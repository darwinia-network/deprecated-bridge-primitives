use codec::{Decode, Encode};

/// A value defining the subset of calls that it is allowed to make.
#[derive(Clone, Encode, Decode)]
pub enum ProxyType {
    Any,
    NonTransfer,
    Staking,
    IdentityJudgement,
    EthereumBridge,
}

/// default value
impl Default for ProxyType {
    fn default() -> ProxyType {
        ProxyType::EthereumBridge
    }
}
