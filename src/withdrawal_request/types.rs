use crate::StdResp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type CreateWithdrawalResponse = StdResp<Option<WithdrawalRequestsData>>;
pub type FetchWithdrawalsResponse = StdResp<Option<Vec<WithdrawalRequestsData>>>;
pub type FetchOneWithdrawalResponse = StdResp<Option<WithdrawalRequestsData>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceData {
    pub request: String,
    #[serde(rename = "fastRequest")]
    pub fast_request: String,
    pub uri: String,
    #[serde(rename = "fastUri")]
    pub fast_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalRequestsData {
    pub id: String,
    pub unit: String,
    pub amount: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "expiresAt")]
    pub expires_at: DateTime<Utc>,
    #[serde(rename = "internalId")]
    pub internal_id: String,
    pub description: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    pub status: String,
    pub invoice: InvoiceData,
}

/// Use this struct to create a well crafted json body for withdrawal requests
#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalReqest {
    #[serde(rename = "expiresIn")]
    pub expires_in: u32,
    pub amount: String,
    pub description: String,
    #[serde(rename = "internalId")]
    pub internal_id: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
}

impl Default for WithdrawalReqest {
    fn default() -> WithdrawalReqest {
        WithdrawalReqest {
            expires_in: 300,
            amount: String::from("0"),
            description: String::from("using zebedee rust sdk"),
            internal_id: String::from(""),
            callback_url: String::from(""),
        }
    }
}
