use crate::ZebedeeClient;
use std::env;

#[tokio::test]
async fn test_get_is_supported_region_by_ip() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let ip = "3.225.112.64";

    let r = zebedee_client
        .get_is_supported_region_by_ip(&ip)
        .await
        .unwrap()
        .success;
    assert!(r);
}

#[tokio::test]
async fn test_get_prod_ips() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let r = zebedee_client.get_prod_ips().await.unwrap().success;
    assert!(r);
}

#[tokio::test]
async fn test_get_btc_usd() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();
    let r = zebedee_client.get_btc_usd().await.unwrap().success;
    assert!(r);
}
