# zebedee-rust

- Interface easily with ZEBEDEE's public API's.
- Currently supports every feature of the public zebedee API.

Cargo.toml:

```
[dependencies]
zebedee_rust = "0.3.4"
```

## Example usage of some of the functions:

### Create a charge

```rust
use std::env;
use zebedee_rust::{charges::*, ZebedeeClient};

#[tokio::main]
async fn main() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zebedee_client = ZebedeeClient::new().apikey(apikey).build();

    // Create a Bolt 11 Invoice for 5000 msat or 5 sat.
    let charge = Charge {
        amount: String::from("5000"),
        ..Default::default()
    };

    // Create the charge
    let charges_res = create_charge(zebedee_client, charge).await.unwrap();

    // Print the response
    println!("{:?}", charges_res);
}
```

### Pay a Lightning Address

```rust
use std::env;
use zebedee_rust::{ln_address::{LnPayment, pay_ln_address}, ZebedeeClient};

#[tokio::main]
async fn main() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zebedee_client = ZebedeeClient::new().apikey(apikey).build();

    // Create a Lightning payment
    let payment = LnPayment {
        ln_address: String::from("dannym@zbd.gg"),
        amount: String::from("10000"),
        ..Default::default()
    };

    // Initiate the payment
    let payment_res = pay_ln_address(zebedee_client, payment).await.unwrap();
    
    // Print the result
    println!("Internal transfer result: {:?}", payment_res);
}
```

### Send an internal transfer

```rust
use std::env;
use zebedee_rust::{internal_transfer::{{InternalTransfer, internal_transfer}}, ZebedeeClient};

#[tokio::main]
async fn main() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zebedee_client = ZebedeeClient::new().apikey(apikey).build();

    // Send Internal Transfer
    let internal_transfer_payload = InternalTransfer {
        amount: String::from("10000"),
        receiver_wallet_id: String::from("b2bcc262-186a-4fe8-961e-a5246383516c"),
    };

    // Initiate the internal transfer
    let transfer_res = internal_transfer(zebedee_client, internal_transfer_payload).await.unwrap();
    
    // Print the result
    println!("Internal transfer result: {:?}", transfer_res);
}
```
