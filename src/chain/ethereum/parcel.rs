//! Ethereum EthereumHeaderParcel
use crate::{
    bytes,
    chain::ethereum::{EthereumHeader, EthereumHeaderJson},
    hex,
};
use codec::{Decode, Encode};

/// Ethereum EthereumHeaderParcel
#[derive(Encode, Decode, Debug, Default)]
pub struct EthereumHeaderParcel {
    /// Ethereum header
    pub header: EthereumHeader,
    /// MMR root
    pub mmr_root: [u8; 32],
}

/// Ethereum EthereumHeaderParcel JSON
#[derive(Default, Deserialize)]
pub struct EthereumHeaderParcelJson {
    /// Ethereum header
    pub header: EthereumHeaderJson,
    /// MMR root
    pub mmr_root: String,
}

impl Into<EthereumHeaderParcel> for EthereumHeaderParcelJson {
    fn into(self) -> EthereumHeaderParcel {
        EthereumHeaderParcel {
            header: self.header.into(),
            mmr_root: bytes!(self.mmr_root.as_str(), 32),
        }
    }
}

impl Into<EthereumHeaderParcelJson> for EthereumHeaderParcel {
    fn into(self) -> EthereumHeaderParcelJson {
        EthereumHeaderParcelJson {
            header: self.header.into(),
            mmr_root: hex!(&self.mmr_root),
        }
    }
}

/// Ethereum EthereumHeaderParcel with proof JSON
#[derive(Default, Deserialize)]
pub struct EthereumHeaderParcelWithConfirmationJson {
    /// Ethereum header parcel
    pub parcel: EthereumHeaderParcelJson,
    /// MMR root
    pub confirmation: u64,
}
