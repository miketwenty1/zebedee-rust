use crate::StdResp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type PaymentInvoiceResponse = StdResp<Option<PaymentsData>>;
pub type FetchPaymentsResponse = StdResp<Option<Vec<PaymentsData>>>;
pub type FetchOnePaymentsResponse = StdResp<Option<PaymentsData>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsData {
    pub id: String,
    pub fee: Option<String>,
    pub unit: String,
    pub amount: String,
    pub invoice: Option<String>,
    pub preimage: Option<String>,
    #[serde(rename = "internalId")]
    pub internal_id: Option<String>,
    #[serde(rename = "processedAt")]
    pub processed_at: Option<DateTime<Utc>>,
    #[serde(rename = "confirmedAt")]
    pub confirmed_at: Option<DateTime<Utc>>,
    pub description: String,
    pub status: String,
}

/// Use this struct to create a well crafted json body for normal ligthning bolt 11 payments
#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub description: String,
    #[serde(rename = "internalId")]
    pub internal_id: String,
    pub invoice: String,
}

impl Default for Payment {
    fn default() -> Payment {
        Payment {
            description: String::from("using zebedee rust sdk"),
            internal_id: String::from(""),
            invoice: String::from(""),
        }
    }
}
