use crate::ZebedeeClient;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysendRes {
    pub success: Option<bool>,
    pub data: Option<KeysendData>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysendTx {
    pub id: String,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
    pub r#type: Option<String>,
    #[serde(rename = "totalAmount")]
    pub total_amount: String,
    pub fee: String,
    pub amount: String,
    pub description: Option<String>,
    pub status: String,
    #[serde(rename = "confirmedAt")]
    pub confirmed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysendData {
    #[serde(rename = "keysendId")]
    pub keysend_id: String,
    #[serde(rename = "paymentId")]
    pub payment_id: String,
    pub transaction: KeysendTx,
}

/// Use this struct to create a well crafted json body for your keysend payments

#[derive(Debug, Serialize, Deserialize)]
pub struct Keysend {
    pub amount: String,
    pub pubkey: String,
    pub tlv_records: Vec<Option<String>>,
    pub metadata: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
}

impl Default for Keysend {
    fn default() -> Self {
        Keysend {
            amount: String::from(""),
            pubkey: String::from(""),
            tlv_records: vec![],
            metadata: String::from(""),
            callback_url: String::from(""),
        }
    }
}

pub async fn keysend(
    client: ZebedeeClient,
    keysend_payload: Keysend,
) -> Result<KeysendRes, anyhow::Error> {
    let url = "https://api.zebedee.io/v0/keysend-payment".to_string();
    let resp = client
        .reqw_cli
        .post(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .json(&keysend_payload)
        .send()
        .await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;

    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text,
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
                resp_text,
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

    #[tokio::test]
    async fn test_keysend() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let keysend_payload = Keysend {
            amount: String::from("1000"),
            pubkey: String::from(
                "0332d57355d673e217238ce3e4be8491aa6b2a13f95494133ee243e57df1653ace",
            ),
            ..Default::default()
        };

        let r = keysend(zebedee_client, keysend_payload)
            .await
            .unwrap()
            .success;
        assert!(r.unwrap());
    }
}
