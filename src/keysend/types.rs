use crate::StdResp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type KeysendResponse = StdResp<Option<KeysendData>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysendTx {
    pub id: String,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
    pub r#type: Option<String>,
    #[serde(rename = "totalAmount")]
    pub total_amount: String,
    pub fee: String,
    pub amount: String,
    pub description: Option<String>,
    pub status: String,
    #[serde(rename = "confirmedAt")]
    pub confirmed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysendData {
    #[serde(rename = "keysendId")]
    pub keysend_id: String,
    #[serde(rename = "paymentId")]
    pub payment_id: String,
    pub transaction: KeysendTx,
}

/// Use this struct to create a well crafted json body for your keysend payments

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Keysend {
    pub amount: String,
    pub pubkey: String,
    #[serde(rename = "tlvRecords")]
    pub tlv_records: Vec<TlvRecord>,
    pub metadata: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TlvRecord {
    #[serde(rename = "type")]
    pub record_type: u32,
    pub value: String, // Must be HEX-string encoded
}
