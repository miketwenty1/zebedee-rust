use super::*;
use crate::ZebedeeClient;
use std::env;

#[tokio::test]
async fn test_create_withdrawal_request() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let withdrawal_request = WithdrawalReqest {
        amount: String::from("10000"),
        ..Default::default()
    };

    let r = zebedee_client
        .create_withdrawal_request(&withdrawal_request)
        .await
        .unwrap();
    assert!(r.success);
}
#[tokio::test]
async fn test_get_withdrawal_requests() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let r = zebedee_client.get_withdrawal_requests().await.unwrap();
    assert!(r.success);
}
#[tokio::test]
async fn test_get_withdrawal_request() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let withdrawal_request = WithdrawalReqest {
        amount: String::from("10000"),
        ..Default::default()
    };

    let r = zebedee_client
        .create_withdrawal_request(&withdrawal_request)
        .await
        .unwrap();
    let r2 = zebedee_client
        .get_withdrawal_request(&r.data.unwrap().id)
        .await
        .unwrap();
    assert!(r2.success);
}
