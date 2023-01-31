# zebedee-rust
- Interface easily with ZEBEDEE's public API's. 
- Currently supports every feature of the public zebedee API. 

Cargo.toml:
```
[dependencies]
zebedee_rust = "0.3.1"
```

### Example usage of some of the functions:
```rust
use std::env;
use zebedee_rust::{
    charges::*, ZebedeeClient,
};

pub fn main() {
    let apikey: String = env::var("ZBD_API_KEY").unwrap();
    let zebedee_client = ZebedeeClient::new().apikey(apikey).build();

    // Create a Bolt 11 Invoice for 5000 msat or 5 sat.
    let invoice = Charge {
        amount: String::from("5000"),
        ..Default::default()
    };
}
```
