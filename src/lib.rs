use charges::{create_charge, Charge};

pub mod charges;
pub mod payments;
pub mod wallet;
pub mod withdrawal_request;

#[derive(Clone, Debug)]
pub struct ZebedeeClient {
    pub reqw_cli: reqwest::Client,
    pub apikey: String,
}

impl ZebedeeClient {
    pub fn new(apikey: String) -> Self {
        ZebedeeClient {
            reqw_cli: reqwest::Client::new(),
            apikey,
        }
    }
}

pub fn main() {
    let apikey = String::from("asdfasdfasd");
    let zebedee_client = ZebedeeClient::new(apikey);
    let charge = Charge {
        ..Default::default()
    };
    let c = create_charge(zebedee_client, charge);
    println!("{:?}", c)
}
