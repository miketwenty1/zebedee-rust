pub mod charges;
pub mod gamertag;
pub mod keysend;
pub mod ln_address;
pub mod login_with_zbd;
pub mod payments;
pub mod utilities;
pub mod wallet;
pub mod withdrawal_request;

use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use validator::Validate;

#[derive(Clone, Debug)]
pub struct ZebedeeClient {
    pub domain: String,
    pub reqw_cli: reqwest::Client,
    pub apikey: String,
    pub oauth: ZebedeeOauth,
}

impl ZebedeeClient {
    pub fn new() -> Self {
        ZebedeeClient::default()
    }
    pub fn domain(mut self, domain: String) -> Self {
        self.domain = domain;
        self
    }
    pub fn apikey(mut self, apikey: String) -> Self {
        self.apikey = apikey;
        self
    }
    pub fn reqw_cli(mut self, reqw_cli: reqwest::Client) -> Self {
        self.reqw_cli = reqw_cli;
        self
    }
    pub fn oauth(
        mut self,
        client_id: String,
        secret: String,
        redirect_uri: String,
        state: String,
    ) -> Self {
        let oauth = ZebedeeOauth::new(client_id, secret, redirect_uri, state);
        self.oauth = oauth;
        self
    }

    pub fn build(self) -> Self {
        ZebedeeClient {
            domain: self.domain,
            reqw_cli: self.reqw_cli,
            apikey: self.apikey,
            oauth: self.oauth,
        }
    }
}

#[derive(Default, Clone, Validate, Deserialize, Debug)]
pub struct ZebedeeOauth {
    #[validate(length(equal = 36))]
    pub client_id: String,
    #[validate(length(equal = 36))]
    pub secret: String,
    #[validate(url)]
    pub redirect_uri: String,
    #[validate(length(equal = 36))]
    pub state: String,
}

impl ZebedeeOauth {
    fn new(client_id: String, secret: String, redirect_uri: String, state: String) -> Self {
        ZebedeeOauth {
            client_id,
            secret,
            redirect_uri,
            state,
        }
    }
}

impl Default for ZebedeeClient {
    fn default() -> Self {
        ZebedeeClient {
            domain: String::from("https://api.zebedee.io"),
            reqw_cli: reqwest::Client::new(),
            apikey: String::from("errornotset"),
            oauth: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Validate, Deserialize)]
pub struct PKCE {
    #[validate(length(equal = 43))]
    pub verifier: String,
    #[validate(length(equal = 43))]
    pub challenge: String,
}

impl PKCE {
    pub fn new(input: [u8; 32]) -> Self {
        let verifier = base64_url::encode(&input);

        let mut hasher_2 = Sha256::new();
        hasher_2.update(verifier.clone());
        let hash_result_2 = hasher_2.finalize();
        let challenge = base64_url::encode(&hash_result_2);

        let p = PKCE {
            verifier,
            challenge,
        };
        p.validate().unwrap();
        p
    }
    pub fn new_from_string(input: String) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(input);
        let hash_result: [u8; 32] = hasher
            .finalize()
            .as_slice()
            .try_into()
            .expect("hashing went wrong from string");
        Self::new(hash_result)
    }
    pub fn new_rand() -> Self {
        let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
        Self::new(random_bytes)
    }
}
