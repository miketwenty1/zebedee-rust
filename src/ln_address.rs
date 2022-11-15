use crate::ZebedeeClient;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct LnAddress {
    #[validate(email)]
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LnPayerData {
    name: HashMap<String, bool>,
    identifier: HashMap<String, bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LnValidateMetadata {
    #[serde(rename = "minSendable")]
    min_sendable: u64,
    #[serde(rename = "maxSendable")]
    max_sendable: u64,
    #[serde(rename = "commentAllowed")]
    comment_allowed: u64,
    tag: String,
    metadata: String,
    callback: String,
    #[serde(rename = "payerData")]
    payer_data: LnPayerData,
    disposable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LnValidateData {
    valid: bool,
    metadata: LnValidateMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LnValidateRes {
    success: bool,
    data: LnValidateData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LnFetchChargeRes {
    success: bool,
    data: LnFetchChargeData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LnInvoice {
    uri: String,
    request: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LnFetchChargeData {
    #[serde(rename = "lnaddress")]
    ln_address: String,
    amount: String,
    invoice: LnInvoice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LnSendPaymentData {
    id: String,
    fee: Option<String>,
    unit: String,
    amount: String,
    preimage: Option<String>,
    status: String,
    invoice: String,
    #[serde(rename = "walletId")]
    wallet_id: String,
    #[serde(rename = "transactionId")]
    transaction_id: String,
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
    #[serde(rename = "processedAt")]
    processed_at: DateTime<Utc>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LnSendPaymentRes {
    success: bool,
    data: LnSendPaymentData,
    message: String,
}

/// Use this struct to create a well crafted json body for your Lightning Address payments
#[derive(Debug, Serialize, Deserialize)]
pub struct LnPayment {
    #[serde(rename = "lnAddress")]
    ln_address: String,
    amount: String,
    comment: String,
}

impl Default for LnPayment {
    fn default() -> Self {
        LnPayment {
            ln_address: String::from(""),
            amount: String::from(""),
            comment: String::from("using zebedee rust sdk"),
        }
    }
}

/// Use this struct to create a well crafted json body for creating charges for Ligthning Addresses
#[derive(Debug, Serialize, Deserialize)]
pub struct LnFetchCharge {
    #[serde(rename = "lnaddress")]
    ln_address: String,
    amount: String,
    description: String,
}

impl Default for LnFetchCharge {
    fn default() -> Self {
        LnFetchCharge {
            ln_address: String::from(""),
            amount: String::from(""),
            description: String::from("using zebedee rust sdk"),
        }
    }
}

#[tokio::main]
pub async fn pay_ln_address(
    client: ZebedeeClient,
    payment: LnPayment,
) -> Result<LnSendPaymentRes, anyhow::Error> {
    let url = format!("https://api.zebedee.io/v0/ln-address/send-payment");
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

    let resp_seralized_2: LnSendPaymentRes = match resp_serialized {
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
pub async fn fetch_charge_ln_address(
    client: ZebedeeClient,
    payment: LnFetchCharge,
) -> Result<LnFetchChargeRes, anyhow::Error> {
    let url = format!("https://api.zebedee.io/v0/ln-address/fetch-charge");
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

    let resp_seralized_2: LnFetchChargeRes = match resp_serialized {
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
pub async fn validate_ln_address(
    client: ZebedeeClient,
    lightning_address: LnAddress,
) -> Result<LnValidateRes, anyhow::Error> {
    match lightning_address.validate() {
        Ok(_) => (),
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Bad LN Address {}, ValidationError {}",
                &lightning_address.address,
                e,
            ))
        }
    };

    let url = format!(
        "https://api.zebedee.io/v0/ln-address/validate/{}",
        lightning_address.address
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_pay_ln_address() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let payment = LnPayment {
            ln_address: String::from("miketwenty1@zbd.gg"),
            amount: String::from("1000"),
            ..Default::default()
        };
        let r = pay_ln_address(zebedee_client, payment).unwrap().success;
        assert_eq!(r, true);
    }
    #[test]
    fn test_fetch_charge_ln_address() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let payment = LnFetchCharge {
            ln_address: String::from("miketwenty1@zbd.gg"),
            amount: String::from("1000"),
            ..Default::default()
        };
        let r = fetch_charge_ln_address(zebedee_client, payment)
            .unwrap()
            .success;
        assert_eq!(r, true);
    }

    #[test]
    fn test_validate_ln_address() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let ln_address = String::from("andre@zbd.gg");

        let r = validate_ln_address(
            zebedee_client,
            LnAddress {
                address: ln_address,
            },
        )
        .unwrap()
        .success;
        assert_eq!(r, true);
    }
}
