use crate::types::Chain;
use chrono::{DateTime, Utc};
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as, TimestampSeconds};
use std::fmt;

use super::{Account, Bundle};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Currency {
    Eth,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub currency: Currency,
    pub decimals: u16,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicListingPrice {
    pub current: Price,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemListing {
    /// The hash of the order.
    pub order_hash: String,
    pub chain: Chain,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub price: BasicListingPrice,
    /// The protocol data for the order. Only 'seaport' is currently supported.
    pub protocol_data: SeaportProtocolData,
    /// The contract address of the protocol.
    pub protocol_address: Option<String>,
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
    #[deprecated()]
    pub maker_asset_bundle: Bundle,
    /// Bundle of assets from the taker.
    #[deprecated()]
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

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeaportOrderParameters {
    pub offerer: String,
    pub offer: Vec<Offer>,
    pub consideration: Vec<Consideration>,
    #[serde_as(as = "TimestampSeconds<String>")]
    pub start_time: DateTime<Utc>,
    #[serde_as(as = "TimestampSeconds<String>")]
    pub end_time: DateTime<Utc>,
    pub order_type: ProtocolOrderType,
    pub zone: String,
    pub zone_hash: String,
    pub salt: String,
    pub conduit_key: String,
    pub total_original_consideration_items: u64,
    #[serde(deserialize_with = "Counter::deserialize")]
    pub counter: Counter,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Counter {
    Number(u64),
    Text(String),
}

// Implementing Deserialize for Counter
impl<'de> Deserialize<'de> for Counter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CounterVisitor;

        impl<'de> Visitor<'de> for CounterVisitor {
            type Value = Counter;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a u64 or a string")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Counter, E>
            where
                E: de::Error,
            {
                Ok(Counter::Number(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Counter, E>
            where
                E: de::Error,
            {
                Ok(Counter::Text(value.to_owned()))
            }

            fn visit_string<E>(self, value: String) -> Result<Counter, E>
            where
                E: de::Error,
            {
                Ok(Counter::Text(value))
            }
        }

        deserializer.deserialize_any(CounterVisitor)
    }
}

// Implementing Serialize for Counter
impl Serialize for Counter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Counter::Number(ref num) => serializer.serialize_u64(*num),
            Counter::Text(ref text) => serializer.serialize_str(text),
        }
    }
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

    use crate::types::api::UserId;

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
