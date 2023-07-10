use super::*;
use crate::ZebedeeClient;
use std::env;

#[tokio::test]
async fn test_create_charge() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();
    let charge = Charge {
        amount: String::from("1000"),
        ..Default::default()
    };

    let r = zebedee_client.create_charge(&charge).await.unwrap();
    assert!(r.success);
}

#[tokio::test]
async fn test_get_charges() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let r = zebedee_client.get_charges().await.unwrap();
    assert!(r.success);
}
#[tokio::test]
async fn test_get_charge() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let charge = Charge {
        amount: String::from("1000"),
        ..Default::default()
    };

    let r = zebedee_client.create_charge(&charge).await.unwrap();
    let r2 = zebedee_client
        .get_charge(&r.data.unwrap().id)
        .await
        .unwrap();
    assert!(r2.success);
}
