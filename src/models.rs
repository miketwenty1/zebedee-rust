use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UnitType {
    #[serde(rename = "msats")]
    Msats,
    #[serde(rename = "sats")]
    Sats,
}
