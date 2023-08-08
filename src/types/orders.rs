use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{Account, Bundle};

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
    pub protocol_data: SeaportProtocolData,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderFee {
    pub account: Account,
    pub basis_points: String,
}

// SEAPORT types
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
pub struct SeaportProtocolData {
    pub parameters: SeaportOrderParameters,
    pub signature: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeaportOrderParameters {
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
pub struct Consideration {
    pub item_type: ItemType,
    pub token: String,
    pub identifier_or_criteria: String,
    /// XXX deserialize to U256 ?
    pub start_amount: String,
    pub end_amount: String,
    pub recipient: String,
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

#[cfg(test)]
mod tests {

    use crate::types::UserId;

    use super::*;

    #[test]
    fn can_deserialize_order_fees() {
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
}
