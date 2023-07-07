use serde::{Deserialize, Serialize};
use crate::StdResp;

pub type WalletInfoResponse = StdResp<Option<WalletData>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletData {
    pub unit: String,
    pub balance: String,
}