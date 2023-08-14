pub mod api;

use super::constants::{API_BASE_MAINNET, API_BASE_TESTNET};
use crate::constants::PROTOCOL_VERSION;
use ethers::types::Chain;
use std::fmt;

/// API endpoints
pub enum ApiUrl {
    Mainnet,
    Testnet,
}

impl fmt::Display for ApiUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mainnet => write!(f, "{}/{}", API_BASE_MAINNET, PROTOCOL_VERSION),
            Self::Testnet => write!(f, "{}/{}", API_BASE_TESTNET, PROTOCOL_VERSION),
        }
    }
}

impl ApiUrl {
    pub fn base(&self) -> String {
        self.to_string()
    }

    pub fn get_listings(&self, chain: &Chain) -> String {
        format!("{}/orders/{}/seaport/listings", self.base(), chain)
    }

    pub fn get_offers(&self, chain: &Chain) -> String {
        format!("{}/orders/{}/seaport/offers", self.base(), chain)
    }

    pub fn fulfill_listing(&self) -> String {
        format!("{}/listings/fulfillment_data", self.base())
    }
}
