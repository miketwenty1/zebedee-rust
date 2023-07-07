use crate::StdResp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type InternalTransferResponse = StdResp<InternalTransferData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalTransferData {
    pub id: String,
    pub status: String,
    pub amount: String,
    #[serde(rename = "senderWalletId")]
    pub sender_wallet_id: String,
    #[serde(rename = "receiverWalletId")]
    pub receiver_wallet_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "sendTxId")]
    pub send_tx_id: String,
    #[serde(rename = "receiveTxId")]
    pub receive_tx_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
/// Use this struct to create a well crafted json body for your internal transfers
#[derive(Debug, Serialize, Deserialize)]
pub struct InternalTransfer {
    pub amount: String,
    #[serde(rename = "receiverWalletId")]
    pub receiver_wallet_id: String,
}
