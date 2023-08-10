pub mod orders;

use std::{collections::HashMap, fmt, str::FromStr};

use chrono::{DateTime, Utc};
use ethers::types::{Bytes, Chain, H160, H256, U256};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use thiserror::Error;

use crate::constants::PROTOCOL_VERSION;

use self::orders::Order;

use super::constants::{API_BASE_MAINNET, API_BASE_TESTNET, SEAPORT_V1, SEAPORT_V4, SEAPORT_V5};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetrieveListingsRequest {
    /// Address of the contract for an NFT
    pub asset_contract_address: Option<String>,
    /// Number of listings to retrieve
    pub limit: Option<u8>,
    /// An array of token IDs to search for (e.g. ?token_ids=1&token_ids=209).
    /// This endpoint will return a list of listings with token_id matching any of the IDs in this array.
    pub token_ids: Vec<String>,
    /// Filter by the order makers wallet address
    pub maker: Option<String>,
    /// Filter by the order takers wallet address
    pub taker: Option<String>,
    /// How to sort the orders. Can be created_date for when they were made,
    /// or eth_price to see the lowest-priced orders first (converted to their ETH values).
    /// eth_price is only supported when asset_contract_address and token_id are also defined.
    pub order_by: Option<String>,
    /// Can be asc or desc for ascending or descending sort. For example, to see the cheapest orders,
    /// do order_direction asc and order_by eth_price.
    pub order_direction: Option<String>,
    /// Only show orders listed after this timestamp. Seconds since the Unix epoch.
    pub listed_after: Option<DateTime<Utc>>,
    /// Only show orders listed before this timestamp. Seconds since the Unix epoch.
    pub listed_before: Option<DateTime<Utc>>,
}

/// Response from OpenSea retrieve listings endpoint containing a list of orders, along with
/// optional pagination information.
///
/// Properties:
///
/// * `next`: An optional string that represents the cursor of the next page of listings. If there is no
/// next page, this field will be None.
/// * `previous`: The `previous` property is an optional string that represents the cursor of the previous
/// page of listings. If there is no previous page, the value will be `None`.
/// * `orders`: The `orders` property is a vector (or array) of `Order` structs. It represents a list of
/// orders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetrieveListingsResponse {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub orders: Vec<Order>,
}

/// Request to fulfill a listing on OpenSea.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FulfillListingRequest {
    pub listing: Listing,
    pub fulfiller: Fulfiller,
}

/// Listing we want to fulfill on OpenSea.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Listing {
    pub hash: H256,
    #[serde(serialize_with = "chain_to_str")]
    pub chain: Chain,
    #[serde(
        rename = "protocol_address",
        serialize_with = "protocol_version_to_str"
    )]
    pub protocol_version: ProtocolVersion,
}

/// Address which will fulfill the listing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fulfiller {
    pub address: H160,
}

/// Response from OpenSea fulfill listing endpoint.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FulfillListingResponse {
    pub protocol: String,
    pub fulfillment_data: FulfillmentData,
}

/// Protocol version for the listing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProtocolVersion {
    V1_1,
    V1_4,
    V1_5,
}

/// Information needed to fulfill the listing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FulfillmentData {
    pub transaction: Transaction,
}

/// Transaction data for onchain fulfillment.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub function: String,
    pub chain: u64,
    pub to: String,
    pub value: u64,
    pub input_data: InputData,
}

/// Additional input data for the transaction.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputData {
    pub parameters: Parameters,
}

/// Parameters for onchain transaction fulfillment.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub consideration_token: H160,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub consideration_identifier: U256,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub consideration_amount: U256,
    pub offerer: H160,
    pub zone: H160,
    pub offer_token: H160,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub offer_identifier: U256,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub offer_amount: U256,
    pub basic_order_type: u8,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub start_time: U256,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub end_time: U256,
    pub zone_hash: H256,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub salt: U256,
    pub offerer_conduit_key: H256,
    pub fulfiller_conduit_key: H256,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub total_original_additional_recipients: U256,
    pub additional_recipients: Vec<AdditionalRecipient>,
    #[serde(deserialize_with = "bytes_from_str")]
    pub signature: Bytes,
}

/// Additional recipient for onchain transaction fulfillment.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdditionalRecipient {
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub amount: U256,
    pub recipient: H160,
}

/// Error returned by the OpenSea API.
#[derive(Debug, Error)]
pub enum OpenSeaApiError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

/// Helper function to convert a chain to a string.
fn chain_to_str<S: Serializer>(chain: &Chain, serializer: S) -> Result<S::Ok, S::Error> {
    let chain_str = match chain {
        Chain::Mainnet => "ethereum",
        _ => Err(serde::ser::Error::custom("Unsupported chain"))?,
    };
    serializer.serialize_str(chain_str)
}

/// Helper function to convert a protocol version to a string.
fn protocol_version_to_str<S: Serializer>(
    protocol_version: &ProtocolVersion,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let protocol_version_str = match protocol_version {
        ProtocolVersion::V1_1 => SEAPORT_V1,
        ProtocolVersion::V1_4 => SEAPORT_V4,
        ProtocolVersion::V1_5 => SEAPORT_V5,
    };
    serializer.serialize_str(protocol_version_str)
}

/// Helper function to convert a string to bytes.
fn bytes_from_str<'de, D>(deserializer: D) -> Result<Bytes, D::Error>
where
    D: de::Deserializer<'de>,
{
    let val = String::deserialize(deserializer)?;
    Bytes::from_str(&val).map_err(de::Error::custom)
}

/// Helper function to convert a decimal string to a U256.
fn u256_from_dec_str<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: de::Deserializer<'de>,
{
    let val = String::deserialize(deserializer)?;
    U256::from_dec_str(&val).map_err(de::Error::custom)
}

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub user: Option<UserId>,
    pub profile_img_url: String,
    pub address: String,
    pub config: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UserId(String);

impl<'de> Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IdVisitor;

        impl<'de> de::Visitor<'de> for IdVisitor {
            type Value = UserId;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("user ID as a number or string")
            }

            fn visit_u64<E>(self, id: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UserId(id.to_string()))
            }

            fn visit_str<E>(self, id: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UserId(id.to_string()))
            }
        }

        deserializer.deserialize_any(IdVisitor)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    pub banner_image_url: Option<String>,
    pub chat_url: Option<String>,
    pub created_date: String,
    pub default_to_fiat: bool,
    pub description: Option<String>,
    pub dev_buyer_fee_basis_points: String,
    pub dev_seller_fee_basis_points: String,
    pub discord_url: Option<String>,
    pub display_data: Value,
    pub external_url: Option<String>,
    pub featured: bool,
    pub featured_image_url: Option<String>,
    pub hidden: bool,
    pub safelist_request_status: String,
    pub image_url: Option<String>,
    pub is_subject_to_whitelist: bool,
    pub large_image_url: Option<String>,
    pub medium_username: Option<String>,
    pub name: String,
    pub only_proxied_transfers: bool,
    pub opensea_buyer_fee_basis_points: String,
    pub opensea_seller_fee_basis_points: u64,
    pub payout_address: Option<String>,
    pub require_email: bool,
    pub short_description: Value,
    pub slug: String,
    pub telegram_url: Value,
    pub twitter_username: Option<String>,
    pub instagram_username: Option<String>,
    pub wiki_url: Value,
    pub is_nsfw: bool,
    pub fees: CollectionFees,
    pub is_rarity_enabled: bool,
    pub is_creator_fees_enforced: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionFees {
    pub seller_fees: HashMap<String, u64>,
    pub opensea_fees: HashMap<String, u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub id: u64,
    pub token_id: String,
    pub num_sales: u64,
    pub background_color: Value,
    pub image_url: String,
    pub image_preview_url: String,
    pub image_thumbnail_url: String,
    pub image_original_url: Option<String>,
    pub animation_url: Value,
    pub animation_original_url: Value,
    pub name: String,
    pub description: Option<String>,
    pub external_link: Option<String>,
    pub asset_contract: AssetContract,
    pub permalink: String,
    pub collection: Collection,
    pub decimals: Value,
    pub token_metadata: Option<String>,
    pub is_nsfw: bool,
    pub owner: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetContract {
    pub address: String,
    pub asset_contract_type: String,
    pub chain_identifier: String,
    pub created_date: String,
    pub name: String,
    pub nft_version: Value,
    pub opensea_version: Option<String>,
    pub owner: Option<u64>,
    pub schema_name: String,
    pub symbol: String,
    pub total_supply: Option<String>,
    pub description: Option<String>,
    pub external_link: Option<String>,
    pub image_url: Option<String>,
    pub default_to_fiat: bool,
    pub dev_buyer_fee_basis_points: u64,
    pub dev_seller_fee_basis_points: u64,
    pub only_proxied_transfers: bool,
    pub opensea_buyer_fee_basis_points: u64,
    pub opensea_seller_fee_basis_points: u64,
    pub buyer_fee_basis_points: u64,
    pub seller_fee_basis_points: u64,
    pub payout_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bundle {
    pub assets: Vec<Asset>,
    pub maker: Value,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub external_link: Option<String>,
    pub asset_contract: Value,
    pub permalink: Option<String>,
    pub seaport_sell_orders: Value,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn can_deserialize_account() {
        let account = r#"{
            "user": 14210173,
            "profile_img_url": "https://storage.googleapis.com/opensea-static/opensea-profile/25.png",
            "address": "0x193d3eda0dbabd55453de814ef08a6255446c911",
            "config": ""
          }"#;
        let account: Account = serde_json::from_str(account).unwrap();
        assert_eq!(account.user, Some(UserId("14210173".to_string())));
    }

    #[test]
    fn can_deserialize_response() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/response_get_listings.json");
        println!("{}", d.display());
        let res = std::fs::read_to_string(d).unwrap();
        let res: RetrieveListingsResponse = serde_json::from_str(&res).unwrap();
        assert_eq!(res.next, Some("LXBrPTExNTE5Njk3NjYw".to_string()));
    }
}
