pub mod charges;
pub mod ln_address;
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

// TODO: generic way to resolve API requests and handle errors without losing the ability to parse specific types in the response.
// pub fn resolve_api_request<T>(response: T) {

// }
