use super::*;
use crate::ZebedeeClient;
use std::env;

#[tokio::test]
async fn test_internal_transfer() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new(apikey).domain(zbdenv);

    let internal_transfer_payload = InternalTransfer {
        amount: String::from("10000"),
        receiver_wallet_id: String::from("b904ee02-ec0b-4fd4-b99f-1f2d3d0001a6"),
    };

    let r = zebedee_client
        .internal_transfer(&internal_transfer_payload)
        .await;

    let i = match r {
        Err(e) => e.to_string(),
        Ok(_) => "was a good transfer but it shouldnt be".to_string(),
    };
    assert!(i.contains("Error processing transfer."));
}
