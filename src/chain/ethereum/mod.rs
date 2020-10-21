//! Ethereum types
// mod confirmation;
mod ethash;
mod header;
mod mmr;
mod parcel;
mod receipt;
mod runtime;

pub use self::{
    ethash::{EthashProof, EthashProofJson},
    header::{EthereumHeader, EthereumHeaderJson, EthereumHeaderRPC},
    mmr::{HeaderStuff, HeaderStuffJson, MMRProof, MMRProofJson},
    parcel::{
        EthereumHeaderThing, EthereumHeaderThingJson, EthereumHeaderThingWithConfirmationJson,
    },
    receipt::{
        EthereumReceiptProof, EthereumReceiptProofJson, EthereumReceiptProofThing,
        EthereumReceiptProofThingJson, RedeemFor,
    },
};
