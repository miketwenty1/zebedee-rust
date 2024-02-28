use crate::ZebedeeClient;
use std::env;

#[tokio::test]
async fn test_wallet_details() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new(apikey).domain(zbdenv);
    let any_balance = 0..; //u64::MAX;
    let r = zebedee_client
        .get_wallet_details()
        .await
        .unwrap()
        .data
        .unwrap()
        .balance;
    let r2: u64 = r.parse().unwrap();
    assert!(any_balance.contains(&r2));
}
