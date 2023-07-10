use crate::StdResp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type FetchChargesResponse = StdResp<Option<Vec<ChargesData>>>;
pub type FetchOneChargeResponse = StdResp<Option<ChargesData>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceData {
    pub request: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChargesData {
    pub id: String, //uuid::Uuid,
    pub unit: String,
    pub amount: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "internalId")]
    pub internal_id: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    pub description: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: DateTime<Utc>,
    #[serde(rename = "confirmedAt")]
    pub confirmed_at: Option<DateTime<Utc>>,
    pub status: String,
    pub invoice: InvoiceData,
}

/// Use this struct to create a well crafted json body for your charge requests
#[derive(Debug, Serialize, Deserialize)]
pub struct Charge {
    #[serde(rename = "expiresIn")]
    pub expires_in: u32,
    pub amount: String,
    pub description: String,
    #[serde(rename = "internalId")]
    pub internal_id: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
}

impl Default for Charge {
    fn default() -> Self {
        Charge {
            expires_in: 300,
            amount: String::from("0"),
            description: String::from("using zebedee rust sdk"),
            internal_id: String::from(""),
            callback_url: String::from(""),
        }
    }
}
