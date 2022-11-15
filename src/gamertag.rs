use std::collections::HashMap;

use crate::ZebedeeClient;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagUserIdRes {
    success: bool,
    data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagPaymentRes {
    success: bool,
    data: GamertagPaymentData,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagPaymentData {
    #[serde(rename = "receiverId")]
    receiver_id: String,
    #[serde(rename = "transactionId")]
    transaction_id: String,
    amount: String,
    comment: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GamertagPayment {
    #[validate(length(min = 1))]
    gamertag: String,
    #[validate(length(min = 4))]
    amount: String,
    description: String,
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
    success: bool,
    data: GamertagChargeData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagChargeData {
    #[serde(rename = "invoiceRequest")]
    invoice_request: String,
    #[serde(rename = "invoiceExpiresAt")]
    invoice_expires_at: DateTime<Utc>,
    unit: String,
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
    status: String,
    #[serde(rename = "internalId")]
    internal_id: Option<String>,
    amount: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagTxRes {
    success: bool,
    data: GamertagTxData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamertagTxData {
    id: String,
    #[serde(rename = "receiverId")]
    receiver_id: String,
    amount: String,
    fee: String,
    unit: String,
    #[serde(rename = "processedAt")]
    processed_at: Option<DateTime<Utc>>,
    #[serde(rename = "confirmedAt")]
    confirmed_at: Option<DateTime<Utc>>,
    comment: String,
    status: String,
}

#[tokio::main]
pub async fn pay_gamertag(
    client: ZebedeeClient,
    payment: GamertagPayment,
) -> Result<GamertagPaymentRes, anyhow::Error> {
    match payment.validate() {
        Ok(_) => (),
        Err(e) => return Err(anyhow::anyhow!("Bad Gamertag Payment format {}", e)),
    };

    let url = format!("https://api.zebedee.io/v0/gamertag/send-payment");
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
pub async fn pay_gamertag_charge(
    client: ZebedeeClient,
    payment: GamertagPayment,
) -> Result<GamertagChargeRes, anyhow::Error> {
    match payment.validate() {
        Ok(_) => (),
        Err(e) => return Err(anyhow::anyhow!("Bad data your payload: {}", e)),
    };

    let url = format!("https://api.zebedee.io/v0/gamertag/charges");
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
    fn test_pay_gamertag() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let payment = GamertagPayment {
            gamertag: String::from("miketwenty1"),
            amount: String::from("1000"),
            ..Default::default()
        };

        let r = pay_gamertag(zebedee_client, payment).unwrap().success;
        assert_eq!(r, true);
    }

    #[test]
    fn test_pay_gamertag_charge() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let payment = GamertagPayment {
            gamertag: String::from("miketwenty1"),
            amount: String::from("1000"),
            ..Default::default()
        };

        let r = pay_gamertag_charge(zebedee_client, payment)
            .unwrap()
            .success;

        assert_eq!(r, true);
    }

    #[test]
    fn test_get_gamertag_tx() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let transaction_id = String::from("322294d5-c993-4eef-88a8-8c9de099e16b");

        let r = get_gamertag_tx(zebedee_client, transaction_id)
            .unwrap()
            .success;
        assert_eq!(r, true);
    }

    #[test]
    fn test_get_userid_by_gamertag() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let gamertag = String::from("miketwenty1");

        let r = get_userid_by_gamertag(zebedee_client, gamertag)
            .unwrap()
            .success;
        assert_eq!(r, true);
    }

    #[test]
    fn test_get_gamertag_by_userid() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let user_id = String::from("0a872b22-d3e2-46c8-84af-139cce32a4c5");

        let r = get_gamertag_by_userid(zebedee_client, user_id)
            .unwrap()
            .success;
        assert_eq!(r, true);
    }
}
