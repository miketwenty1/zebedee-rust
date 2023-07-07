use super::*;
use crate::ZebedeeClient;
use std::env;

#[tokio::test]
async fn test_keysend() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let keysend_payload = Keysend {
        amount: String::from("1000"),
        pubkey: String::from("0332d57355d673e217238ce3e4be8491aa6b2a13f95494133ee243e57df1653ace"),
        ..Default::default()
    };

    let r = zebedee_client
        .keysend(&keysend_payload)
        .await
        .unwrap()
        .success;
    assert!(r);
}
