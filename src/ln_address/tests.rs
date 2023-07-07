use crate::ZebedeeClient;
use std::env;

use super::*;

#[tokio::test]
async fn test_pay_ln_address() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let payment = LnPayment {
        ln_address: String::from("miketwenty1@zbd.gg"),
        amount: String::from("1000"),
        ..Default::default()
    };
    let r = zebedee_client
        .pay_ln_address(&payment)
        .await
        .unwrap()
        .success;
    assert!(r);
}
#[tokio::test]
async fn test_fetch_charge_ln_address() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let payment = LnFetchCharge {
        ln_address: String::from("miketwenty1@zbd.gg"),
        amount: String::from("1000"),
        ..Default::default()
    };
    let r = zebedee_client
        .fetch_charge_ln_address(&payment)
        .await
        .unwrap()
        .success;
    assert!(r);
}

#[tokio::test]
async fn test_validate_ln_address() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let ln_address = String::from("andre@zbd.gg");

    let r = zebedee_client
        .validate_ln_address(&LnAddress {
            address: ln_address,
        })
        .await
        .unwrap()
        .success;
    assert!(r);
}
