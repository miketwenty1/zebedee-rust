use crate::ZebedeeClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBtcUsdRes {
    success: bool,
    data: BtcUsdData,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BtcUsdData {
    #[serde(rename = "btcUsdPrice")]
    btc_usd_price: String,
    #[serde(rename = "btcUsdTimestamp")]
    btc_usd_timestamp: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetProdIpsRes {
    success: bool,
    data: IpData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpData {
    ips: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIsSupportedRegionByIpRes {
    success: bool,
    data: RegionIpData,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionIpData {
    #[serde(rename = "ipAddress")]
    ip_address: String,
    #[serde(rename = "isSupported")]
    is_supported: bool,
}

#[tokio::main]
pub async fn get_is_supported_region_by_ip(
    client: ZebedeeClient,
    ip: String,
) -> Result<GetIsSupportedRegionByIpRes, anyhow::Error> {
    let url = format!("https://api.zebedee.io/v0/is-supported-region/{}", ip);
    let resp = client
        .reqw_cli
        .get(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .send()
        .await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;

    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text.clone(),
            &url,
        ));
    }

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2 = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\n status code: {}",
                e,
                resp_text.clone(),
                status_code
            ))
        }
    };

    Ok(resp_seralized_2)
}

#[tokio::main]
pub async fn get_prod_ips(client: ZebedeeClient) -> Result<GetProdIpsRes, anyhow::Error> {
    let url = format!("https://api.zebedee.io/v0/prod-ips");
    let resp = client
        .reqw_cli
        .get(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .send()
        .await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;

    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text.clone(),
            &url,
        ));
    }

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2 = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\n status code: {}",
                e,
                resp_text.clone(),
                status_code
            ))
        }
    };

    Ok(resp_seralized_2)
}

#[tokio::main]
pub async fn get_btc_usd(client: ZebedeeClient) -> Result<GetBtcUsdRes, anyhow::Error> {
    let url = format!("https://api.zebedee.io/v0/btcusd");
    let resp = client
        .reqw_cli
        .get(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .send()
        .await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;

    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text.clone(),
            &url,
        ));
    }

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2 = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\n status code: {}",
                e,
                resp_text.clone(),
                status_code
            ))
        }
    };

    Ok(resp_seralized_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_is_supported_region_by_ip() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let ip = String::from("3.225.112.64");

        let r = get_is_supported_region_by_ip(zebedee_client, ip)
            .unwrap()
            .success;
        assert_eq!(r, true);
    }

    #[test]
    fn test_get_prod_ips() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let r = get_prod_ips(zebedee_client).unwrap().success;
        assert_eq!(r, true);
    }

    #[test]
    fn test_get_btc_usd() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);
        let r = get_btc_usd(zebedee_client).unwrap().success;
        assert_eq!(r, true);
    }
}
