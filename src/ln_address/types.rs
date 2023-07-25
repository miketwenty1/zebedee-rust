use crate::StdResp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::{Validate, ValidationErrors};

pub type PayLnAddressResponse = StdResp<Option<LnSendPaymentData>>;
pub type FetchLnChargeResponse = StdResp<Option<LnFetchChargeData>>;
pub type ValidateLnAddrResponse = StdResp<Option<LnValidateData>>;

#[derive(Debug, Validate, Deserialize)]
pub struct LnAddress {
    #[validate(email)]
    pub address: String,
}

impl LnAddress {
    /// Verifies that the given lightning address is in a valid email format.
    /// Returns `Ok(())` if the format is valid, and `Err(ValidationErrors)` otherwise.
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        Validate::validate(&self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LnPayerData {
    pub name: HashMap<String, bool>,
    pub identifier: HashMap<String, bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LnValidateMetadata {
    #[serde(rename = "minSendable")]
    pub min_sendable: u64,
    #[serde(rename = "maxSendable")]
    pub max_sendable: u64,
    #[serde(rename = "commentAllowed")]
    pub comment_allowed: u64,
    pub tag: String,
    pub metadata: String,
    pub callback: String,
    #[serde(rename = "payerData")]
    pub payer_data: LnPayerData,
    pub disposable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LnValidateData {
    pub valid: bool,
    pub metadata: LnValidateMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LnInvoice {
    pub uri: String,
    pub request: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LnFetchChargeData {
    #[serde(rename = "lnaddress")]
    pub ln_address: String,
    pub amount: String,
    pub invoice: LnInvoice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LnSendPaymentData {
    pub id: String,
    pub fee: Option<String>,
    pub unit: String,
    pub amount: String,
    pub preimage: Option<String>,
    pub status: String,
    pub invoice: String,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "processedAt")]
    pub processed_at: DateTime<Utc>,
    #[serde(rename = "callbackURL")]
    pub callback_url: Option<String>,
    #[serde(rename = "internalId")]
    pub internal_id: Option<String>,
}

/// Use this struct to create a well crafted json body for your Lightning Address payments
#[derive(Debug, Serialize, Deserialize)]
pub struct LnPayment {
    #[serde(rename = "lnAddress")]
    pub ln_address: String,
    pub amount: String,
    pub comment: String,
}

impl Default for LnPayment {
    fn default() -> Self {
        LnPayment {
            ln_address: String::from(""),
            amount: String::from(""),
            comment: String::from("using zebedee rust sdk"),
        }
    }
}

/// Use this struct to create a well crafted json body for creating charges for Ligthning Addresses
#[derive(Debug, Serialize, Deserialize)]
pub struct LnFetchCharge {
    #[serde(rename = "lnaddress")]
    pub ln_address: String,
    pub amount: String,
    pub description: String,
}

impl Default for LnFetchCharge {
    fn default() -> Self {
        LnFetchCharge {
            ln_address: String::from(""),
            amount: String::from(""),
            description: String::from("using zebedee rust sdk"),
        }
    }
}
