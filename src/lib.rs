pub mod charges;
pub mod gamertag;
pub mod keysend;
pub mod ln_address;
pub mod login_with_zbd;
pub mod payments;
pub mod utilities;
pub mod wallet;
pub mod withdrawal_request;

use anyhow::Result;
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct ZebedeeClient {
    pub reqw_cli: reqwest::Client,
    pub apikey: String,
    pub oauth: ZebedeeOauth,
}

impl ZebedeeClient {
    pub fn new(apikey: String) -> Self {
        ZebedeeClient {
            reqw_cli: reqwest::Client::new(),
            apikey,
            oauth: Default::default(),
        }
    }
}

#[derive(Clone, Validate, Deserialize, Debug)]
pub struct ZebedeeOauth {
    #[validate(length(equal = 36))]
    pub client_id: String,
    #[validate(length(equal = 36))]
    pub secret: String,
    #[validate(url)]
    pub redirect_uri: String,
}

impl Default for ZebedeeOauth {
    fn default() -> Self {
        ZebedeeOauth {
            client_id: Default::default(),
            secret: Default::default(),
            redirect_uri: Default::default(),
        }
    }
}

impl ZebedeeClient {
    pub fn set_oauth(
        self: Self,
        client_id: String,
        secret: String,
        redirect_uri: String,
    ) -> Result<Self, anyhow::Error> {
        Uuid::parse_str(&client_id)?;
        Uuid::parse_str(&secret)?;

        let zeb_oauth = ZebedeeOauth {
            client_id,
            secret,
            redirect_uri,
        };

        match zeb_oauth.validate() {
            Ok(_) => (),
            Err(e) => return Err(anyhow::anyhow!("Bad oauth inputs {}", e)),
        };

        Ok(ZebedeeClient {
            reqw_cli: self.reqw_cli,
            apikey: self.apikey,
            oauth: zeb_oauth,
        })
    }
}

#[derive(Clone, Debug, Validate, Deserialize)]
pub struct PKCE {
    #[validate(length(equal = 43))]
    verifier: String,
    #[validate(length(equal = 43))]
    challenge: String,
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
// }
// pub fn initialize_stream(&mut self) {
//     self.stream = Some(TcpStream::connect("127.0.0.1:34254").unwrap());
// }
// pub struct get_32b_secret (s: String) -> Buffer {

// }
// TODO: generic way to resolve API requests and handle errors without losing the ability to parse specific types in the response.
// pub fn resolve_api_request<T>(response: T) {

// }
