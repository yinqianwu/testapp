use chrono::{DateTime, Utc};
use serde::Serialize;
use rocket_contrib::json::JsonValue;
use crate::common::market_types::{MarketType};
use crate::common::const_types;
use crate::common::event_types::EventType;

#[derive(Queryable)]
pub struct StockInfo {
    pub id: String,
    pub stock_id: String,
    pub stock_cn_name: String,
    pub stock_en_name: String,
    pub corps_cn_name: String,
    pub corps_en_name: String,
    pub description: String,
    pub market_type: i32,
    pub head_location: String,
    pub founding_time : String,
    pub listing_time : String,
    pub section_tag_list: Vec<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}