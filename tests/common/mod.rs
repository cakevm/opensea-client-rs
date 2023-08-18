use opensea_v2::{types::Chain, OpenSeaApiConfig, OpenSeaV2Client};

pub fn test_client() -> OpenSeaV2Client {
    let cfg = OpenSeaApiConfig {
        chain: Chain::Goerli,
        ..Default::default()
    };

    OpenSeaV2Client::new(cfg)
}

#[allow(dead_code)]
pub fn live_client() -> OpenSeaV2Client {
    let cfg = OpenSeaApiConfig {
        ..Default::default()
    };

    OpenSeaV2Client::new(cfg)
}
