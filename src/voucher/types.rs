use crate::{
    custom_deserializer::{deserialize_from_m_string, deserialize_from_string},
    models::UnitType,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoucherData {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub amount: u64,
    pub code: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "createTransactionId")]
    pub create_transaction_id: String,
    pub description: String,
    #[serde(deserialize_with = "deserialize_from_m_string")]
    pub fee: Option<u64>,
    pub id: String,
    pub unit: UnitType,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}
