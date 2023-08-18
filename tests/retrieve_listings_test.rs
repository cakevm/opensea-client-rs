use opensea_v2::{
    types::{
        api::{OrderOpeningOption, RetrieveListingsRequest},
        Chain,
    },
    OpenSeaApiConfig, OpenSeaV2Client,
};

fn test_client() -> OpenSeaV2Client {
    let cfg = OpenSeaApiConfig {
        chain: Chain::Goerli,
        ..Default::default()
    };

    OpenSeaV2Client::new(cfg)
}

fn live_client() -> OpenSeaV2Client {
    let cfg = OpenSeaApiConfig {
        ..Default::default()
    };

    OpenSeaV2Client::new(cfg)
}

#[tokio::test]
async fn can_retrieve_listing() {
    let client = test_client();

    let req = RetrieveListingsRequest {
        limit: Some(3),
        order_by: Some(OrderOpeningOption::CreatedDate),
        ..Default::default()
    };

    let res = client.retrieve_listings(req).await.unwrap();

    assert_eq!(res.orders.len(), 3);
    assert!(res.next.is_some());
}
