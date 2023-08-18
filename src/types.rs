pub mod api;

use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{AsRefStr, EnumString};
use thiserror::Error;

/// Error returned by the OpenSea API.
#[derive(Debug, Error)]
pub enum OpenSeaApiError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("{0}")]
    Other(String),
}

/// API endpoints
#[derive(Debug, Clone)]
pub struct ApiUrl {
    pub base: String,
}

impl ApiUrl {
    pub fn get_listings(&self, chain: &Chain) -> String {
        format!("{}/orders/{}/seaport/listings", self.base, chain)
    }

    pub fn get_offers(&self, chain: &Chain) -> String {
        format!("{}/orders/{}/seaport/offers", self.base, chain)
    }

    pub fn fulfill_listing(&self) -> String {
        format!("{}/listings/fulfillment_data", self.base)
    }
}

/// Each of the possible chains that OpenSea supports.
/// https://github.com/ProjectOpenSea/opensea-js/blob/813b9189221024f3761e622bb418264f002fcce5/src/types.ts#L98
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
    // When adding to this list, also add to the is_test_chain method
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

impl Chain {
    pub fn is_test_chain(&self) -> bool {
        use Chain::*;
        matches!(
            self,
            Goerli
                | Sepolia
                | Mumbai
                | Boabab
                | BaseGoerli
                | BSCTestnet
                | ArbitrumGoerli
                | AvalancheFuji
                | OptimismGoerli
                | SolanaDevnet
                | ZoraTestnet
        )
    }

    #[inline]
    pub fn is_live_chain(&self) -> bool {
        !self.is_test_chain()
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
