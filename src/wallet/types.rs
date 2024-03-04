use crate::StdResp;
use serde::{Deserialize, Serialize};

pub type WalletInfoResponse = StdResp<Option<WalletData>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletData {
    pub unit: String,
    pub balance: String,
}
