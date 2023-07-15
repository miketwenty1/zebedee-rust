use crate::StdResp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

pub type GamertagPayResponse = StdResp<GamertagPaymentData>;
pub type GamertagChargeResponse = StdResp<Option<GamertagChargeData>>;
pub type GamertagTxResoonse = StdResp<Option<GamertagTxData>>;
pub type GamertagUserIdResponse = StdResp<Option<HashMap<String, String>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagPaymentData {
    #[serde(rename = "receiverId")]
    pub receiver_id: String,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    pub amount: String,
    pub comment: String,
    #[serde(rename = "settledAt")]
    pub settled_at: DateTime<Utc>,
    pub status: String,
    pub id: String
}

/// Use this struct to create a well crafted json body for your gamertag payments
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GamertagPayment {
    #[validate(length(min = 1))]
    pub gamertag: String,
    #[validate(length(min = 4))]
    pub amount: String,
    pub description: String,
}
impl Default for GamertagPayment {
    fn default() -> Self {
        GamertagPayment {
            gamertag: String::from(""),
            amount: String::from(""),
            description: String::from("using zebedee rust sdk"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagChargeData {
    #[serde(rename = "invoiceRequest")]
    pub invoice_request: String,
    #[serde(rename = "invoiceExpiresAt")]
    pub invoice_expires_at: DateTime<Utc>,
    pub unit: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    pub status: String,
    #[serde(rename = "internalId")]
    pub internal_id: Option<String>,
    pub amount: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagTxData {
    pub id: String,
    #[serde(rename = "receiverId")]
    pub receiver_id: String,
    pub amount: String,
    pub fee: String,
    pub unit: String,
    #[serde(rename = "processedAt")]
    pub processed_at: Option<DateTime<Utc>>,
    #[serde(rename = "confirmedAt")]
    pub confirmed_at: Option<DateTime<Utc>>,
    pub comment: String,
    pub status: String,
}
