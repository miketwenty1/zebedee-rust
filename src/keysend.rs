use crate::ZebedeeClient;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysendRes {
    pub success: bool,
    pub data: KeysendData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysendTx {
    pub fee: String,
    pub amount: String,
    pub status: String,
    #[serde(rename = "totalAmount")]
    pub total_amount: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
    pub id: String,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
    pub r#type: String,
    pub invoice: Option<String>,
    #[serde(rename = "confirmedAt")]
    pub confirmed_at: Option<String>,
    pub description: String,
    pub flow: String,
    pub created_at: DateTime<Utc>,
    #[serde(rename = "totalAmountUsd")]
    pub total_amount_usd: String,
    #[serde(rename = "invoiceRequest")]
    pub invoice_request: Option<String>,
    #[serde(rename = "invoiceExpiresAt")]
    pub invoice_expires_at: Option<String>,
    #[serde(rename = "invoiceId")]
    pub invoice_id: Option<String>,
    pub unit: String,
    pub apikey: String,
    #[serde(rename = "entityId")]
    pub entity_d: String,
    #[serde(rename = "createdAt")]
    pub created_at2: DateTime<Utc>,
    #[serde(rename = "staticChargeComment")]
    pub static_charge_comment: Option<String>,
    #[serde(rename = "staticChargeId")]
    pub static_charge_id: Option<String>,
    #[serde(rename = "peerPaymentId")]
    pub peer_payment_id: Option<String>,
    #[serde(rename = "peerPaymentComment")]
    pub peer_payment_comment: Option<String>,
    #[serde(rename = "peerPaymentCounterpartyId")]
    pub peer_payment_counterparty_id: Option<String>,
    #[serde(rename = "peerPaymentImage")]
    pub peer_payment_image: Option<String>,
    #[serde(rename = "systemTopUpAdminId")]
    pub system_top_up_admin_id: Option<String>,
    #[serde(rename = "sourceApp")]
    pub source_app: String,
    #[serde(rename = "sourceCity")]
    pub source_city: String,
    #[serde(rename = "sourceCountry")]
    pub source_country: String,
    #[serde(rename = "sourceIP")]
    pub source_ip: String,
    #[serde(rename = "sourceRegion")]
    pub source_region: String,
    #[serde(rename = "claimedTransactionId")]
    pub claimed_transaction_id: Option<String>,
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
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
        assert_eq!(r, true);
    }
}
