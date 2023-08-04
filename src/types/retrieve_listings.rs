use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::serde_as;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetrieveListingsResponse {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub orders: Vec<Order>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub created_date: String,
    pub closing_date: Option<String>,
    pub listing_time: i64,
    pub expiration_time: i64,
    pub order_hash: Option<String>,
    pub protocol_data: ProtocolData,
    pub protocol_address: Option<String>,
    pub current_price: String,
    pub maker: Account,
    pub taker: Option<Account>,
    pub maker_fees: Vec<Fees>,
    pub taker_fees: Vec<Fees>,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub cancelled: bool,
    pub finalized: bool,
    pub marked_invalid: bool,
    pub remaining_quantity: i64,
    pub client_signature: Option<String>,
    pub relay_id: String,
    pub criteria_proof: Option<String>,
    pub maker_asset_bundle: Bundle,
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
    pub order_type: i64,
    pub zone: String,
    pub zone_hash: String,
    pub salt: String,
    pub conduit_key: String,
    pub total_original_consideration_items: i64,
    pub counter: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    pub item_type: i64,
    pub token: String,
    pub identifier_or_criteria: String,
    /// XXX deserialize to U256 ?
    pub start_amount: String,
    pub end_amount: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consideration {
    pub item_type: i64,
    pub token: String,
    pub identifier_or_criteria: String,
    /// XXX deserialize to U256 ?
    pub start_amount: String,
    pub end_amount: String,
    pub recipient: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fee {
    pub account: Account,
    pub basis_points: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub user: Option<String>,
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
    pub id: i64,
    pub token_id: String,
    pub num_sales: i64,
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
    pub owner: i64,
    pub schema_name: String,
    pub symbol: String,
    pub total_supply: String,
    pub description: String,
    pub external_link: Option<String>,
    pub image_url: Option<String>,
    pub default_to_fiat: bool,
    pub dev_buyer_fee_basis_points: i64,
    pub dev_seller_fee_basis_points: i64,
    pub only_proxied_transfers: bool,
    pub opensea_buyer_fee_basis_points: i64,
    pub opensea_seller_fee_basis_points: i64,
    pub buyer_fee_basis_points: i64,
    pub seller_fee_basis_points: i64,
    pub payout_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    pub banner_image_url: Option<String>,
    pub chat_url: Value,
    pub created_date: String,
    pub default_to_fiat: bool,
    pub description: String,
    pub dev_buyer_fee_basis_points: String,
    pub dev_seller_fee_basis_points: String,
    pub discord_url: Option<String>,
    pub display_data: DisplayData,
    pub external_url: Option<String>,
    pub featured: bool,
    pub featured_image_url: String,
    pub hidden: bool,
    pub safelist_request_status: String,
    pub image_url: String,
    pub is_subject_to_whitelist: bool,
    pub large_image_url: String,
    pub medium_username: Option<String>,
    pub name: String,
    pub only_proxied_transfers: bool,
    pub opensea_buyer_fee_basis_points: String,
    pub opensea_seller_fee_basis_points: i64,
    pub payout_address: Option<String>,
    pub require_email: bool,
    pub short_description: Value,
    pub slug: String,
    pub telegram_url: Value,
    pub twitter_username: Option<String>,
    pub instagram_username: Option<String>,
    pub wiki_url: Value,
    pub is_nsfw: bool,
    pub fees: Fees,
    pub is_rarity_enabled: bool,
    pub is_creator_fees_enforced: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayData {
    pub card_display_style: String,
    pub images: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fees {
    pub seller_fees: HashMap<String, u64>,
    pub opensea_fees: HashMap<String, u64>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn can_deserialize_account() {
        let res = r#"{
            "user": 14210173,
            "profile_img_url": "https://storage.googleapis.com/opensea-static/opensea-profile/25.png",
            "address": "0x193d3eda0dbabd55453de814ef08a6255446c911",
            "config": ""
          }"#;
        let res: Account = serde_json::from_str(&res).unwrap();
        assert_eq!(res.user, Some("14210173".to_string()));
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
