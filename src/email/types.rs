use crate::{custom_deserializer::deserialize_from_string, StdResp, VoucherData};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type EmailPaymentResponse = StdResp<EmailPaymentRes>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmailPaymentRes {
    /// Email is associated with an existing ZBD App account, then that user account will be credited the sats.
    ExistingZbdAccount(EmailPaymentData),
    /// Email not associated to a ZBD account, the sats will be issued to the email in the form of a valid ZBD Voucher that the recipient can redeem in the ZBD Mobile App.
    Voucher(VoucherData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailPaymentData {
    pub id: String,
    // TODO: Convert from string to a proper Enum
    pub status: String,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub amount: u64,
    pub comment: String,
    #[serde(rename = "receiverId")]
    pub receiver_id: String,
    #[serde(rename = "senderTxId")]
    pub sender_tx_id: String,
    #[serde(rename = "settledAt")]
    pub settled_at: DateTime<Utc>,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

/// Send instant Bitcoin payments to any email
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailPaymentReqest {
    /// Recipient email to send payment to.
    pub email: String,
    /// Total amount of satoshis to send (in millisatoshis).
    pub amount: String,
    /// comment to be sent with the payment (max 150 characters).
    pub comment: String,
}
