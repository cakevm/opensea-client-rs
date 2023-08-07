use std::collections::HashMap;
use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// The `RetrieveListingsResponse` struct represents a response containing a list of orders, along with
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

/// The latest OpenSea Order schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    /// The date the order was created.
    pub created_date: String,
    /// The date the order was closed.
    pub closing_date: Option<String>,
    /// The date the order was listed. Order can be created before the listing time.
    pub listing_time: u64,
    /// The date the order expires.
    pub expiration_time: u64,
    /// The hash of the order.
    pub order_hash: Option<String>,
    /// The protocol data for the order. Only 'seaport' is currently supported.
    pub protocol_data: ProtocolData,
    /// The contract address of the protocol.
    pub protocol_address: Option<String>,
    /// The current price of the order.
    // XXX U256
    pub current_price: String,
    /// The account that created the order.
    pub maker: Account,
    /// The account that filled the order.
    pub taker: Option<Account>,
    /// The maker fees for the order.
    pub maker_fees: Vec<OrderFee>,
    /// The taker fees for the order.
    pub taker_fees: Vec<OrderFee>,
    /// The side of the order. Ask/Bid
    pub side: OrderSide,
    /// The type of the order. Basic/Dutch/English/Criteria
    pub order_type: OrderType,
    /// Whether or not the maker has cancelled the order.
    pub cancelled: bool,
    /// Whether or not the order is finalized.
    pub finalized: bool,
    /// Whether or not the order is marked invalid and therefore not fillable.
    pub marked_invalid: bool,
    /// Amount of items left in the order which can be taken.
    pub remaining_quantity: u64,
    /// The signature the order is signed with.
    pub client_signature: Option<String>,
    pub relay_id: String,
    pub criteria_proof: Option<String>,
    /// Bundle of assets from the maker.
    pub maker_asset_bundle: Bundle,
    /// Bundle of assets from the taker.
    pub taker_asset_bundle: Bundle,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Ask,
    Bid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Basic,
    Dutch,
    English,
    Criteria,
}

#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ItemType {
    Native,
    ERC20,
    ERC721,
    ERC1155,
    ERC721WithCriteria,
    ERC1155WithCriteria,
}

#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ProtocolOrderType {
    /// No partial fills, anyone can execute
    FullOpen,
    /// Partial fills supported, anyone can execute
    PartialOpen,
    /// No partial fills, only offerer or zone can execute
    FullRestricted,
    /// Partial fills supported, only offerer or zone can execute
    PartialRestricted,
}

// XXX This type is described in seaport-js
// https://github.com/ProjectOpenSea/seaport-js/blob/3939e3b4ce052783849ce667d8ec2d32c6905d6c/src/types.ts#L187C13-L187C29
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtocolData {
    pub parameters: Parameters,
    pub signature: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub offerer: String,
    pub offer: Vec<Offer>,
    pub consideration: Vec<Consideration>,
    /// XXX deserialize to chrono::DateTime ?
    pub start_time: String,
    pub end_time: String,
    pub order_type: ProtocolOrderType,
    pub zone: String,
    pub zone_hash: String,
    pub salt: String,
    pub conduit_key: String,
    pub total_original_consideration_items: u64,
    pub counter: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    pub item_type: ItemType,
    pub token: String,
    pub identifier_or_criteria: String,
    /// XXX deserialize to U256 ?
    pub start_amount: String,
    pub end_amount: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consideration {
    pub item_type: ItemType,
    pub token: String,
    pub identifier_or_criteria: String,
    /// XXX deserialize to U256 ?
    pub start_amount: String,
    pub end_amount: String,
    pub recipient: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderFee {
    pub account: Account,
    pub basis_points: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub user: Option<UserId>,
    pub profile_img_url: String,
    pub address: String,
    pub config: String,
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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UserId(String);

impl<'de> Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IdVisitor;

        impl<'de> Visitor<'de> for IdVisitor {
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

#[cfg(test)]
mod tests {

    use super::*;
    use std::path::PathBuf;

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
    fn can_deserialize_fees() {
        let fees = r#"{
          "account": {
            "user": 14210173,
            "profile_img_url": "https://storage.googleapis.com/opensea-static/opensea-profile/25.png",
            "address": "0x193d3eda0dbabd55453de814ef08a6255446c911",
            "config": ""
          },
          "basis_points": "600"
        }"#;

        let fees: OrderFee = serde_json::from_str(fees).unwrap();
        assert_eq!(fees.account.user, Some(UserId("14210173".to_string())));
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
