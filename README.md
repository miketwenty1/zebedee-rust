# zebedee-rust
crappy noob attempt at making a rust sdk for zebedee api

```rust
pub fn main() {
    let apikey = String::from("asdfasdfasd");
    let zebedee_client = ZebedeeClient::new(apikey);
    let charge = Charge {
        ..Default::default()
    };
    let c = create_charge(zebedee_client, charge);
    println!("{:?}", c)
}
```