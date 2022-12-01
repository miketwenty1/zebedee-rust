# zebedee-rust
- Interface easily with ZEBEDEE's public API's. 
- Currently supports every feature of the public zebedee API. 

Cargo.toml:
```
[dependencies]
zebedee_rust = "0.1.4"
```

### Example usage of some of the functions:
```rust
use std::env;
use zebedee_rust::{
    charges::*, gamertag::*, keysend::*, wallet::*, withdrawal_request::*, ZebedeeClient,
};

pub fn main() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zebedee_client = ZebedeeClient::new(apikey);
    let balance = get_wallet_details(zebedee_client.clone())
        .unwrap()
        .data
        .balance;
    println!("I have {} sats", balance);

    // My friend owes me money when for lunch let me create an invoice.
    let charge = Charge {
        amount: 21_000_000.to_string(), // denominated in mili sats. 21,000,000 msats = 21,000 sats.
        ..Default::default()
    };

    let c = create_charge(zebedee_client.clone(), charge);
    println!("I can send bolt 11 invoice {:#?}", c.unwrap().data.invoice);


    // Let's do a keysend payment

    let keysend_payload = Keysend {
        amount: String::from("1000"),
        pubkey: String::from("0333333310101010101010101010101010101010101010101010101010101010"),
        callback_url: "https://somewebhookwebsitecallback.zyzx".to_string(),
        ..Default::default()
    };

    let res = keysend(zebedee_client.clone(), keysend_payload).unwrap();
    println!("keysend response {:#?} ", res);

    // Withdrawal requests are sort of like creating a voucher for someone to redeem bitcoin. For instance you can print these out on QR codes and people can claim the sats within the ZEBEDEE app.

    let withdrawal_request = WithdrawalReqest {
        amount: String::from(100_000.to_string()),
        description: String::from("Wow you found the secret bitcoin I put in a public park!"),
        ..Default::default()
    };

    let voucher = create_withdrawal_request(zebedee_client.clone(), withdrawal_request).unwrap();
    // let's check on this request to see if it's been claimed or when it expires.
    let voucher_lookup = get_withdrawal_request(zebedee_client, voucher.data.id).unwrap();
}
```

### Examples for login with zebedee/oauth2
```rust
// Create Client this way with "set_oauth" function.
use std::env;
use zebedee_rust::{login_with_zbd::*, ZebedeeClient, PKCE};

pub fn main() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let oauth_client_id: String = env::var("ZBD_OAUTH_CLIENT_ID").unwrap();
    let oauth_secret: String = env::var("ZBD_OAUTH_SECRET").unwrap();
    let redirect_uri: String = env::var("ZBD_REDIRECT_URI").unwrap();
    let state: String = env::var("ZBD_OAUTH_STATE").unwrap();

    let zebedee_client = ZebedeeClient::new(apikey)
        .set_oauth(oauth_client_id, oauth_secret, redirect_uri, state)
        .unwrap();

    let pkce = PKCE::new_from_string(String::from("hellomynameiswhat"));
    let url_for_user_auth: String =
        create_auth_url(zebedee_client.clone(), pkce.challenge.clone()).unwrap();
    // after user logs in with ZBD via the url from "url_for_user_auth" the callback url will get a code.
    let fake_code = String::from("xxx11xx1-xxxx-xxxx-xxx1-1xx11xx111xx"); // for example we will plant this here.

    // fetching auth token for the user.
    let fetchbody = FetchPost::new(zebedee_client.clone(), fake_code, pkce.verifier);
    let token_for_user = fetch_token(zebedee_client.clone(), fetchbody)
        .unwrap()
        .access_token;

    // using fake_refresh_token as an example:
    let fake_refresh_token = String::from("eyAAAAyomommagotocollegeAAAxxxXXAAAAasdfasdfsas");
    // now with "user_data" you can get infromation like user_id, gamertag, email.
    let user_data = fetch_user_data(zebedee_client.clone(), fake_refresh_token)
        .unwrap()
        .data;
}
```
