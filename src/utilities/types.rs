use crate::StdResp;
use serde::{Deserialize, Serialize};

pub type SupportedIpResponse = StdResp<Option<RegionIpData>>;
pub type ProdIpsResponse = StdResp<Option<IpData>>;
pub type BtcToUsdResponse = StdResp<Option<BtcUsdData>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BtcUsdData {
    #[serde(rename = "btcUsdPrice")]
    pub btc_usd_price: String,
    #[serde(rename = "btcUsdTimestamp")]
    pub btc_usd_timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpData {
    pub ips: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionIpData {
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    #[serde(rename = "isSupported")]
    pub is_supported: bool,
    #[serde(rename = "ipCountry")]
    pub ip_country: String,
    #[serde(rename = "ipRegion")]
    pub ip_region: String,
}
