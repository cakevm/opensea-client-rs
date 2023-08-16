pub mod api;

use super::constants::{API_BASE_MAINNET, API_BASE_TESTNET};
use crate::constants::PROTOCOL_VERSION;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{AsRefStr, EnumString};

/// API endpoints
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, AsRefStr, Default)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Chain {
    // Mainnet Chains
    #[default]
    #[strum(to_string = "ethereum", serialize = "mainnet")]
    #[serde(alias = "mainnet")]
    Ethereum,
    #[strum(to_string = "matic", serialize = "polygon")]
    #[serde(rename = "matic", alias = "polygon")]
    Polygon,
    Klaytn,
    Base,
    BSC,
    Arbitrum,
    ArbitrumNova,
    Avalanche,
    Optimism,
    Solana,
    Zora,

    // Testnet Chains
    Goerli,
    Sepolia,
    Mumbai,
    Boabab,
    BaseGoerli,
    BSCTestnet,
    ArbitrumGoerli,
    #[strum(to_string = "avalanche_fuji", serialize = "fuji")]
    #[serde(alias = "fuji")]
    AvalancheFuji,
    OptimismGoerli,
    SolanaDevnet,
    ZoraTestnet,
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.as_ref())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::Value;

    #[test]
    fn can_display_and_parse_chain() {
        let chain = Chain::Polygon;
        assert_eq!(format!("{chain}"), "matic");

        let chain: Chain = Default::default();
        assert_eq!(format!("{chain}"), "ethereum");

        let chain = Chain::AvalancheFuji;
        assert_eq!(format!("{chain}"), "avalanche_fuji");

        let chain: Chain = "polygon".parse().unwrap();
        assert_eq!(chain, Chain::Polygon);
    }

    #[test]
    fn can_serialize_chain() {
        let chain = Chain::Polygon;
        let value = serde_json::to_value(&chain).unwrap();
        assert_eq!(Value::String("matic".to_string()), value);

        let chain: Chain = Default::default();
        let value = serde_json::to_value(&chain).unwrap();
        assert_eq!(Value::String("ethereum".to_string()), value);
    }

    #[test]
    fn can_deserialize_chain() {
        #[derive(Deserialize)]
        struct ChainTest {
            chain: Chain,
        }

        let data: ChainTest = serde_json::from_str(r#"{ "chain": "matic" }"#).unwrap();
        assert_eq!(data.chain, Chain::Polygon);

        let data: ChainTest = serde_json::from_str(r#"{ "chain": "mainnet" }"#).unwrap();
        assert_eq!(data.chain, Chain::Ethereum);

        let data: ChainTest = serde_json::from_str(r#"{ "chain": "ethereum" }"#).unwrap();
        assert_eq!(data.chain, Chain::Ethereum);
    }
}
