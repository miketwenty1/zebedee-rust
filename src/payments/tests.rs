use super::*;
use crate::ZebedeeClient;
use std::env;

#[tokio::test]
async fn test_pay_invoice() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let payment = Payment {
        invoice: String::from("lnbc120n1p0tdjwmpp5ycws0d788cjeqp9rn2wwxfymrekj9n80wy2yrk66tuu3ga5wukfsdzq2pshjmt9de6zqen0wgsrzv3qwp5hsetvwvsxzapqwdshgmmndp5hxtnsd3skxefwxqzjccqp2sp5vnsvmjlu6hrfegcdjs47njrga36g3x45wfmqjjjlerwgagj62yysrzjq2v4aw4gy7m93en32dcaplym056zezcljdjshyk8yakwtsp2h4yvcz9atuqqhtsqqqqqqqlgqqqqqqgqjq9qy9qsqhykfacrdy06cuyegvt4p50su53qwgrqn5jf6d83fd0upsa4frpxqnm2zl323zuvmz5ypv9gh9nr3jav6u2ccwkpd56h3n6l3ja5q7wgpxudlv4"),
        ..Default::default()
    };
    // expected to get a 400 error
    let r = zebedee_client.pay_invoice(&payment).await.err().unwrap();
    assert!(r.to_string().contains("error"));
}

#[tokio::test]
async fn test_get_payments() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();
    let rrr = zebedee_client.get_payments().await;
    println!("{:#?}", rrr);
    let r = zebedee_client.get_payments().await.unwrap();
    assert!(r.success);
}

#[tokio::test]
async fn test_get_payment() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let payment_id = String::from("5d88b2e0-e491-40e1-a8a8-a81ae68f2297");

    let r = zebedee_client.get_payment(&payment_id).await.err().unwrap();
    assert!(r.to_string().contains("No Payment"));
}
