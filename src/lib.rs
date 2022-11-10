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
