use super::*;
use crate::ZebedeeClient;
use std::env;

#[tokio::test]
async fn test_keysend() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new(apikey).domain(zbdenv);
    let tlvs = vec![TlvRecord {
        record_type: 123123123,
        value: String::from("00ABCDEF"),
    }];

    let keysend_payload = Keysend {
        amount: String::from("1000"),
        pubkey: String::from("033e514ff30be0ea421f9512da0ed1aea52ea541275654d034bde3470a61269285"),
        tlv_records: tlvs,
        ..Default::default()
    };

    let r = zebedee_client
        .keysend(&keysend_payload)
        .await
        .unwrap()
        .success;
    assert!(r);
}
