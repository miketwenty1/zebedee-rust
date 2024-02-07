use crate::ZebedeeClient;
use std::env;

use super::*;

#[tokio::test]
async fn test_pay_email() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zbdenv: String =
        env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
    let email = env::var("EMAIL").unwrap();

    let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

    let email_payment_req = EmailPaymentReqest {
        email,
        amount: "1000".to_owned(),
        comment: "from rust sdk test".to_owned(),
    };

    println!("email test: {:#?}", email_payment_req);

    let r = zebedee_client.pay_email(&email_payment_req).await.unwrap();
    assert!(r.success);

    match r.data {
        EmailPaymentRes::ExistingZbdAccount(data) => {
            assert!(r.message.map(|a| a.contains("Payment")).unwrap_or_default());
            println!("Account exists");
            println!("{data:#?}");
        }
        EmailPaymentRes::Voucher(data) => {
            assert!(r.message.map(|a| a.contains("Voucher")).unwrap_or_default());
            println!("Account does not exists");
            println!("{data:#?}");
        }
    }
}
