use std::collections::HashMap;

use crate::ZebedeeClient;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagUserIdRes {
    pub success: Option<bool>,
    pub data: Option<HashMap<String, String>>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagPaymentRes {
    pub success: bool,
    pub data: GamertagPaymentData,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagPaymentData {
    #[serde(rename = "receiverId")]
    pub receiver_id: String,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    pub amount: String,
    pub comment: String,
}

/// Use this struct to create a well crafted json body for your gamertag payments
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GamertagPayment {
    #[validate(length(min = 1))]
    pub gamertag: String,
    #[validate(length(min = 4))]
    pub amount: String,
    pub description: String,
}
impl Default for GamertagPayment {
    fn default() -> Self {
        GamertagPayment {
            gamertag: String::from(""),
            amount: String::from(""),
            description: String::from("using zebedee rust sdk"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagChargeRes {
    pub success: Option<bool>,
    pub data: Option<GamertagChargeData>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagChargeData {
    #[serde(rename = "invoiceRequest")]
    pub invoice_request: String,
    #[serde(rename = "invoiceExpiresAt")]
    pub invoice_expires_at: DateTime<Utc>,
    pub unit: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    pub status: String,
    #[serde(rename = "internalId")]
    pub internal_id: Option<String>,
    pub amount: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagTxRes {
    pub message: Option<String>,
    pub success: Option<bool>,
    pub data: Option<GamertagTxData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagTxData {
    pub id: String,
    #[serde(rename = "receiverId")]
    pub receiver_id: String,
    pub amount: String,
    pub fee: String,
    pub unit: String,
    #[serde(rename = "processedAt")]
    pub processed_at: Option<DateTime<Utc>>,
    #[serde(rename = "confirmedAt")]
    pub confirmed_at: Option<DateTime<Utc>>,
    pub comment: String,
    pub status: String,
}

pub async fn pay_gamertag(
    client: ZebedeeClient,
    payment: GamertagPayment,
) -> Result<GamertagPaymentRes, anyhow::Error> {
    match payment.validate() {
        Ok(_) => (),
        Err(e) => return Err(anyhow::anyhow!("Bad Gamertag Payment format {}", e)),
    };

    let url = "https://api.zebedee.io/v0/gamertag/send-payment".to_string();
    let resp = client
        .reqw_cli
        .post(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .json(&payment)
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

/// Create a bolt 11 invoice so you can pay a specified gamertag
pub async fn fetch_charge_from_gamertag(
    client: ZebedeeClient,
    payment: GamertagPayment,
) -> Result<GamertagChargeRes, anyhow::Error> {
    match payment.validate() {
        Ok(_) => (),
        Err(e) => return Err(anyhow::anyhow!("Bad data your payload: {}", e)),
    };

    let url = "https://api.zebedee.io/v0/gamertag/charges".to_string();
    let resp = client
        .reqw_cli
        .post(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .json(&payment)
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

pub async fn get_gamertag_tx(
    client: ZebedeeClient,
    transaction_id: String,
) -> Result<GamertagTxRes, anyhow::Error> {
    let url = format!(
        "https://api.zebedee.io/v0/gamertag/transaction/{}",
        transaction_id
    );
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

pub async fn get_userid_by_gamertag(
    client: ZebedeeClient,
    gamertag: String,
) -> Result<GamertagUserIdRes, anyhow::Error> {
    let url = format!("https://api.zebedee.io/v0/user-id/gamertag/{}", gamertag);
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

pub async fn get_gamertag_by_userid(
    client: ZebedeeClient,
    user_id: String,
) -> Result<GamertagUserIdRes, anyhow::Error> {
    let url = format!("https://api.zebedee.io/v0/gamertag/user-id/{}", user_id);
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
    async fn test_pay_gamertag() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let payment = GamertagPayment {
            gamertag: String::from("miketwenty1"),
            amount: String::from("1000"),
            ..Default::default()
        };

        let r = pay_gamertag(zebedee_client, payment).await.unwrap().success;
        assert!(r);
    }

    #[tokio::test]
    async fn test_fetch_charge_from_gamertag() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let payment = GamertagPayment {
            gamertag: String::from("miketwenty1"),
            amount: String::from("1000"),
            ..Default::default()
        };

        let r = fetch_charge_from_gamertag(zebedee_client, payment)
            .await
            .unwrap()
            .success;

        assert!(r.unwrap());
    }

    #[tokio::test]
    async fn test_get_gamertag_tx() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let transaction_id = String::from("322294d5-c993-4eef-88a8-8c9de099e16b");

        let r = get_gamertag_tx(zebedee_client, transaction_id)
            .await
            .unwrap()
            .success;
        assert!(r.unwrap());
    }

    #[tokio::test]
    async fn test_get_userid_by_gamertag() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let gamertag = String::from("miketwenty1");

        let r = get_userid_by_gamertag(zebedee_client, gamertag)
            .await
            .unwrap()
            .success;
        assert!(r.unwrap());
    }

    #[tokio::test]
    async fn test_get_gamertag_by_userid() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let user_id = String::from("0a872b22-d3e2-46c8-84af-139cce32a4c5");

        let r = get_gamertag_by_userid(zebedee_client, user_id)
            .await
            .unwrap()
            .success;
        assert!(r.unwrap());
    }
}
