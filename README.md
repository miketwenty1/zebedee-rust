# zebedee-rust

- Interface easily with ZEBEDEE's public APIs.
- Currently supports most features of the public [ZEBEDEE API](https://zbd.dev/api-reference).
- Accepting PR's and Github issues for needed functionality.


## Example usage:

### Bolt11 Invoice (Creating a Charge)

```rust
use std::env;
use zebedee_rust::{charges::*, ZebedeeClient};

#[tokio::main]
async fn main() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zebedee_client = ZebedeeClient::new(apikey);

    // Create a Bolt 11 Invoice for 5000 msat or 5 sat.
    let charge = Charge {
        amount: String::from("5000"),
        ..Default::default()
    };

    // Create the charge
    let charges_res = zebedee_client.create_charge(&charge).await.unwrap();

    // Print the response
    println!("{:?}", charges_res);
}
```

### Pay a Lightning Address

```rust
use std::env;
use zebedee_rust::{ln_address::*, ZebedeeClient};

#[tokio::main]
async fn main() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zebedee_client = ZebedeeClient::new(apikey);

    // Create a Lightning payment
    let payment = LnPayment {
        ln_address: String::from("dannym@zbd.gg"),
        amount: String::from("1000"),
        ..Default::default()
    };

    // Initiate the payment
    let payment_res = zebedee_client.pay_ln_address(&payment).await.unwrap();
    
    // Print the result
    println!("Internal transfer result: {:?}", payment_res);
}
```

### Send an internal transfer

```rust
use std::env;
use zebedee_rust::{internal_transfer::*, ZebedeeClient};

#[tokio::main]
async fn main() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zebedee_client = ZebedeeClient::new(apikey);

    // Send Internal Transfer
    let internal_transfer_payload = InternalTransfer {
        amount: String::from("1000"),
        receiver_wallet_id: String::from("b2bcc262-186a-4fe8-961e-a5246383516c"),
    };

    // Initiate the internal transfer
    let transfer_res = zebedee_client.internal_transfer(&internal_transfer_payload).await.unwrap();
    
    // Print the result
    println!("Internal transfer result: {:?}", transfer_res);
}
```
