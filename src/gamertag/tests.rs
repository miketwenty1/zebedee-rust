use super::*;
use crate::ZebedeeClient;
use std::env;

#[tokio::test]
async fn test_pay_gamertag() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let payment = GamertagPayment {
        gamertag: String::from("miketwenty1"),
        amount: String::from("1000"),
        ..Default::default()
    };

    let r = zebedee_client.pay_gamertag(&payment).await.unwrap().success;
    assert!(r);
}

#[tokio::test]
async fn test_fetch_charge_from_gamertag() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let payment = GamertagPayment {
        gamertag: String::from("miketwenty1"),
        amount: String::from("1000"),
        ..Default::default()
    };

    let r = zebedee_client
        .fetch_charge_from_gamertag(&payment)
        .await
        .unwrap()
        .success;

    assert!(r);
}

#[tokio::test]
async fn test_get_gamertag_tx() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let transaction_id = String::from("322294d5-c993-4eef-88a8-8c9de099e16b");

    let r = zebedee_client
        .get_gamertag_tx(&transaction_id)
        .await
        .unwrap()
        .success;
    assert!(r);
}

#[tokio::test]
async fn test_get_userid_by_gamertag() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let gamertag = String::from("miketwenty1");

    let r = zebedee_client
        .get_userid_by_gamertag(&gamertag)
        .await
        .unwrap()
        .success;
    assert!(r);
}

#[tokio::test]
async fn test_get_gamertag_by_userid() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let user_id = String::from("0a872b22-d3e2-46c8-84af-139cce32a4c5");

    let r = zebedee_client
        .get_gamertag_by_userid(&user_id)
        .await
        .unwrap()
        .success;
    assert!(r);
}
