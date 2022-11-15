use crate::ZebedeeClient;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysendRes {
    success: bool,
    data: KeysendData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysendTx {
    fee: String,
    amount: String,
    status: String,
    #[serde(rename = "totalAmount")]
    total_amount: String,
    #[serde(rename = "updatedAt")]
    updated_at: DateTime<Utc>,
    id: String,
    #[serde(rename = "walletId")]
    wallet_id: String,
    r#type: String,
    invoice: Option<String>,
    #[serde(rename = "confirmedAt")]
    confirmed_at: Option<String>,
    description: String,
    flow: String,
    created_at: DateTime<Utc>,
    #[serde(rename = "totalAmountUsd")]
    total_amount_usd: String,
    #[serde(rename = "invoiceRequest")]
    invoice_request: Option<String>,
    #[serde(rename = "invoiceExpiresAt")]
    invoice_expires_at: Option<String>,
    #[serde(rename = "invoiceId")]
    invoice_id: Option<String>,
    unit: String,
    apikey: String,
    #[serde(rename = "entityId")]
    entity_d: String,
    #[serde(rename = "createdAt")]
    created_at2: DateTime<Utc>,
    #[serde(rename = "staticChargeComment")]
    static_charge_comment: Option<String>,
    #[serde(rename = "staticChargeId")]
    static_charge_id: Option<String>,
    #[serde(rename = "peerPaymentId")]
    peer_payment_id: Option<String>,
    #[serde(rename = "peerPaymentComment")]
    peer_payment_comment: Option<String>,
    #[serde(rename = "peerPaymentCounterpartyId")]
    peer_payment_counterparty_id: Option<String>,
    #[serde(rename = "peerPaymentImage")]
    peer_payment_image: Option<String>,
    #[serde(rename = "systemTopUpAdminId")]
    system_top_up_admin_id: Option<String>,
    #[serde(rename = "sourceApp")]
    source_app: String,
    #[serde(rename = "sourceCity")]
    source_city: String,
    #[serde(rename = "sourceCountry")]
    source_country: String,
    #[serde(rename = "sourceIP")]
    source_ip: String,
    #[serde(rename = "sourceRegion")]
    source_region: String,
    #[serde(rename = "claimedTransactionId")]
    claimed_transaction_id: Option<String>,
    #[serde(rename = "isInternal")]
    is_internal: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysendData {
    #[serde(rename = "keysendId")]
    keysend_id: String,
    #[serde(rename = "paymentId")]
    payment_id: String,
    transaction: KeysendTx,
}

/// Use this struct to create a well crafted json body for your keysend payments

#[derive(Debug, Serialize, Deserialize)]
pub struct Keysend {
    amount: String,
    pubkey: String,
    tlv_records: Vec<Option<String>>,
    metadata: String,
    #[serde(rename = "callbackUrl")]
    callback_url: String,
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

#[tokio::main]
pub async fn keysend(
    client: ZebedeeClient,
    keysend_payload: Keysend,
) -> Result<KeysendRes, anyhow::Error> {
    let url = format!("https://api.zebedee.io/v0/keysend-payment");
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
    fn test_keysend() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let keysend_payload = Keysend {
            amount: String::from("1000"),
            pubkey: String::from(
                "0332d57355d673e217238ce3e4be8491aa6b2a13f95494133ee243e57df1653ace",
            ),
            ..Default::default()
        };

        let r = keysend(zebedee_client, keysend_payload).unwrap().success;
        assert_eq!(r, true);
    }
}
