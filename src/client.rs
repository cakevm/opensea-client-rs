use reqwest::{
    header::{self, HeaderMap},
    Client, ClientBuilder,
};

use crate::{
    constants::{API_BASE_MAINNET, API_BASE_TESTNET, PROTOCOL_VERSION},
    types::{
        api::{
            CollectionResponse, FulfillListingRequest, FulfillListingResponse, GetAllListingsRequest, GetAllListingsResponse,
            OpenSeaDetailedErrorCode::{OrderCannotBeFulfilled, OrderHashDoesNotExist},
            OpenSeaErrorResponse, RetrieveListingsRequest, RetrieveListingsResponse,
        },
        ApiUrl, Chain, OpenSeaApiError,
    },
};

//. A partial implementation of the OpenSea API v2, supporting the fulfill listing endpoint.
#[derive(Debug, Clone)]
pub struct OpenSeaV2Client {
    client: Client,
    chain: Chain,
    url: ApiUrl,
}

/// Configuration for the OpenSea API client.
#[derive(Debug, Clone, Default)]
pub struct OpenSeaApiConfig {
    pub api_key: Option<String>,
    pub chain: Chain,
}

impl OpenSeaV2Client {
    /// Create a new client with the given configuration.
    pub fn new(cfg: OpenSeaApiConfig) -> Self {
        let mut builder = ClientBuilder::new();
        let mut headers = HeaderMap::new();

        if let Some(ref api_key) = cfg.api_key {
            headers.insert("X-API-KEY", header::HeaderValue::from_str(api_key).unwrap());
        }

        builder = builder.default_headers(headers);
        let client = builder.build().unwrap();

        let base_url = if cfg.chain.is_test_chain() { API_BASE_TESTNET } else { API_BASE_MAINNET };

        let base_url = format!("{base_url}/{PROTOCOL_VERSION}");

        Self { client, chain: cfg.chain, url: ApiUrl { base: base_url } }
    }
    pub async fn get_collection_by_slug(&self, collection_slug: String) -> Result<CollectionResponse, OpenSeaApiError> {
        let res = self.client.get(self.url.get_collection(collection_slug)).send().await?.json::<CollectionResponse>().await?;
        Ok(res)
    }

    pub async fn retrieve_listings(&self, req: RetrieveListingsRequest) -> Result<RetrieveListingsResponse, OpenSeaApiError> {
        let res = self
            .client
            .get(self.url.get_listings(&self.chain))
            .query(&req.to_qs_vec()?)
            .send()
            .await?
            .json::<RetrieveListingsResponse>()
            .await?;
        Ok(res)
    }

    /// Call the fulfill listing endpoint, which returns the arguments necessary
    /// to fulfill an order onchain.
    pub async fn fulfill_listing(&self, req: FulfillListingRequest) -> Result<FulfillListingResponse, OpenSeaApiError> {
        let res = self.client.post(self.url.fulfill_listing()).json(&req).send().await;
        match res {
            Ok(res) => {
                if res.status() == 400 {
                    let res = res.json::<OpenSeaErrorResponse>().await?;
                    let first_error = res.errors.first();
                    if let Some(first_error) = first_error {
                        match first_error.as_str() {
                            "The order_hash you provided does not exist" => {
                                return Err(OpenSeaApiError::OpenSeaDetailedError(OrderHashDoesNotExist));
                            }
                            "This order can not be fulfilled at this time." => {
                                return Err(OpenSeaApiError::OpenSeaDetailedError(OrderCannotBeFulfilled));
                            }
                            &_ => {}
                        }
                    }
                    return Err(OpenSeaApiError::OpenSeaError(res));
                }

                let res = res.json::<FulfillListingResponse>().await?;
                Ok(res)
            }
            Err(e) => Err(OpenSeaApiError::Reqwest(e)),
        }
    }

    pub async fn get_collection(&self, collection_slug: String) -> Result<CollectionResponse, OpenSeaApiError> {
        let res = self.client.get(self.url.get_collection(collection_slug)).send().await?.json::<CollectionResponse>().await?;
        Ok(res)
    }

    pub async fn get_all_listings(
        &self,
        collection_slug: String,
        params: GetAllListingsRequest,
    ) -> Result<GetAllListingsResponse, OpenSeaApiError> {
        let query_parameters = serde_url_params::to_string(&params).unwrap();
        let res = self
            .client
            .get(self.url.get_all_listings(collection_slug, query_parameters))
            .send()
            .await?
            .json::<GetAllListingsResponse>()
            .await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::api::orders::{Counter, Currency};
    use alloy_primitives::U256;
    use chrono::DateTime;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn can_deserialize_get_all_listings_response() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/response_get_all_listings.json");
        println!("{}", d.display());
        let res = std::fs::read_to_string(d).unwrap();
        let res: GetAllListingsResponse = serde_json::from_str(&res).unwrap();
        assert_eq!(res.listings.first().unwrap().order_hash, "0x541a9eb3962494caffeda36a495cc978c7ecc21c6b714aaabc678187d3da9ac7");
        assert_eq!(res.listings.first().unwrap().price.current.currency, Currency::Other("USD".to_string()));
        assert_eq!(
            res.listings.first().unwrap().protocol_data.parameters.start_time,
            DateTime::parse_from_rfc3339("2023-10-29T04:50:26Z").unwrap()
        );
        assert_eq!(res.listings.get(0).unwrap().price.current.value, "25000000000000000000");
        assert_eq!(res.listings.get(0).unwrap().protocol_data.parameters.counter, Counter::Number(0));
        assert_eq!(res.listings.get(0).unwrap().price.current.currency, Currency::Other("USD".to_string()));
    }

    #[test]
    fn can_deserialize_fulfill_listing_v6_response() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/response_fulfill_listing_1.6.json");
        println!("{}", d.display());
        let res = std::fs::read_to_string(d).unwrap();
        let res: FulfillListingResponse = serde_json::from_str(&res).unwrap();
        assert_eq!(res.protocol, "seaport1.6");
        assert_eq!(res.fulfillment_data.transaction.value, U256::from_str("23690000000000000000").unwrap());
    }

    #[test]
    fn can_deserialize_fulfill_listing_v5_response() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/response_fulfill_listing_1.5.json");
        println!("{}", d.display());
        let res = std::fs::read_to_string(d).unwrap();
        let res: FulfillListingResponse = serde_json::from_str(&res).unwrap();
        assert_eq!(res.protocol, "seaport1.5");
        assert_eq!(res.fulfillment_data.transaction.value, U256::from_str("20000000000000000").unwrap());
    }

    #[test]
    fn can_deserialize_fulfill_listing_v4_response() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/response_fulfill_listing_1.4.json");
        println!("{}", d.display());
        let res = std::fs::read_to_string(d).unwrap();
        let res: FulfillListingResponse = serde_json::from_str(&res).unwrap();
        assert_eq!(res.protocol, "seaport1.4");
        assert_eq!(res.fulfillment_data.transaction.value, U256::from_str("1780000000000000000").unwrap());
    }
}
