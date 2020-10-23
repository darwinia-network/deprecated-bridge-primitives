//! Relayable chains
mod affirmation;
pub mod ethereum;
pub mod proxy_type;

pub use self::affirmation::{RelayAffirmation, RelayAffirmationId};
