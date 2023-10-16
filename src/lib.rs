pub mod charges;
pub mod errors;
pub mod gamertag;
pub mod internal_transfer;
pub mod keysend;
pub mod ln_address;
pub mod login_with_zbd;
pub mod payments;
pub mod utilities;
pub mod wallet;
pub mod withdrawal_request;

use charges::*;
use errors::*;
use gamertag::*;
use internal_transfer::*;
use keysend::*;
use ln_address::*;
use login_with_zbd::*;
use payments::*;
use rand::Rng;
use reqwest::{RequestBuilder, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use utilities::*;
use validator::Validate;
use wallet::*;
use withdrawal_request::*;

pub type Result<T, E = errors::ZebedeeError> = std::result::Result<T, E>;

#[derive(Clone, Debug)]
pub struct ZebedeeClient {
    domain: String,
    reqw_cli: reqwest::Client,
    apikey: String,
    oauth: ZebedeeOauth,
}

impl ZebedeeClient {
    pub fn new() -> Self {
        ZebedeeClient::default()
    }

    /// Zebedee REST API url
    pub fn domain(mut self, domain: String) -> Self {
        self.domain = domain;
        self
    }

    /// Project API key
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
        scope: String,
    ) -> Self {
        let oauth = ZebedeeOauth::new(client_id, secret, redirect_uri, state, scope);
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

    async fn parse_response<T>(&self, resp: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let is_success = resp.status().is_success();
        // parse the resp body
        let body = resp.json::<Value>().await?;

        // based on success or error choose the appropriate data structure to deserialize
        match is_success {
            true => {
                let body = serde_json::from_value::<T>(body)?;
                Ok(body)
            }
            false => {
                let err_body: ApiError = serde_json::from_value(body)?;
                Err(err_body.into())
            }
        }
    }

    fn add_headers(&self, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder
            .header("Content-Type", "application/json")
            .header("apikey", &self.apikey)
    }

    /// Retrieves the total balance of a given Project Wallet.
    pub async fn get_wallet_details(&self) -> Result<WalletInfoResponse> {
        let url = format!("{}/v0/wallet", &self.domain);
        let resp = self.add_headers(self.reqw_cli.get(&url)).send().await?;
        self.parse_response(resp).await
    }

    /// Make payment directly to a Lightning Network node Public Key, without the need for a Payment Request / Charge.
    pub async fn keysend(&self, keysend_payload: &Keysend) -> Result<KeysendResponse> {
        let url = format!("{}/v0/keysend-payment", &self.domain);

        let resp = self
            .add_headers(self.reqw_cli.post(&url))
            .json(keysend_payload)
            .send()
            .await?;

        self.parse_response(resp).await
    }

    /// Creates a new Charge / Payment Request in the Bitcoin Lightning Network, payable by any Lightning Network wallet.
    /// These payment requests are single-use, fixed-amount QR codes. If you're looking for multi-use and multi-amount
    /// payment requests you want Static Charges.
    pub async fn create_charge(&self, charge: &Charge) -> Result<FetchOneChargeResponse> {
        let url = format!("{}/v0/charges", &self.domain);

        let resp = self
            .add_headers(self.reqw_cli.post(&url))
            .json(&charge)
            .send()
            .await?;

        self.parse_response(resp).await
    }

    pub async fn get_charges(&self) -> Result<FetchChargesResponse> {
        let url = format!("{}/v0/charges", &self.domain);
        let resp = self.add_headers(self.reqw_cli.get(&url)).send().await?;
        self.parse_response(resp).await
    }

    /// Retrieves all information relating a specific Charge / Payment Request.
    pub async fn get_charge<T>(&self, charge_id: T) -> Result<FetchOneChargeResponse>
    where
        T: AsRef<str>,
    {
        let url = format!("{}/v0/charges/{}", &self.domain, charge_id.as_ref());
        let resp = self.add_headers(self.reqw_cli.get(&url)).send().await?;
        self.parse_response(resp).await
    }

    /// Send Bitcoin payments directly to a user's ZBD Gamertag
    pub async fn pay_gamertag(&self, payment: &GamertagPayment) -> Result<GamertagPayResponse> {
        payment
            .validate()
            .map_err(|e| ErrorMsg::BadGamerTagFormat(e.to_string()))?;

        let url = format!("{}/v0/gamertag/send-payment", &self.domain);

        let resp = self
            .add_headers(self.reqw_cli.post(&url))
            .json(payment)
            .send()
            .await?;

        self.parse_response(resp).await
    }

    /// Create a bolt 11 invoice so you can pay a specified gamertag
    pub async fn fetch_charge_from_gamertag(
        &self,
        payment: &GamertagPayment,
    ) -> Result<GamertagChargeResponse> {
        payment
            .validate()
            .map_err(|e| ErrorMsg::BadPayloadData(e.to_string()))?;

        let url = format!("{}/v0/gamertag/charges", &self.domain);

        let resp = self
            .add_headers(self.reqw_cli.post(&url))
            .json(payment)
            .send()
            .await?;

        self.parse_response(resp).await
    }

    /// Get data on payments sent to ZBD Gamertags.
    /// The data payload returned will inform you of the status of that transaction as well as any associated fees.
    pub async fn get_gamertag_tx<T>(&self, transaction_id: T) -> Result<GamertagTxResponse>
    where
        T: AsRef<str>,
    {
        let url = format!(
            "{}/v0/gamertag/transaction/{}",
            &self.domain,
            transaction_id.as_ref()
        );

        let resp = self.add_headers(self.reqw_cli.get(&url)).send().await?;
        self.parse_response(resp).await
    }

    /// Get a given User's ID when provided with a ZBD Gamertag.
    pub async fn get_userid_by_gamertag<T>(&self, gamertag: T) -> Result<IdFromGamertagResponse>
    where
        T: AsRef<str>,
    {
        let url = format!("{}/v0/user-id/gamertag/{}", &self.domain, gamertag.as_ref());
        let resp = self.add_headers(self.reqw_cli.get(&url)).send().await?;
        self.parse_response(resp).await
    }

    /// Get a given user's ZBD Gamertag from user id
    pub async fn get_gamertag_by_userid<T>(&self, user_id: T) -> Result<GamertagUserIdResponse>
    where
        T: AsRef<str>,
    {
        let url = format!("{}/v0/gamertag/user-id/{}", &self.domain, user_id.as_ref());
        let resp = self.add_headers(self.reqw_cli.get(&url)).send().await?;
        self.parse_response(resp).await
    }

    /// Initiates a transfer of funds between two Project Wallets you own.
    pub async fn internal_transfer(
        &self,
        internal_transfer_payload: &InternalTransfer,
    ) -> Result<InternalTransferResponse> {
        let url = format!("{}/v0/internal-transfer", &self.domain);
        let resp = self
            .add_headers(self.reqw_cli.post(&url))
            .json(internal_transfer_payload)
            .send()
            .await?;

        self.parse_response(resp).await
    }

    /// Send Bitcoin payments directly to a Lightning Address.
    pub async fn pay_ln_address(&self, payment: &LnPayment) -> Result<PayLnAddressResponse> {
        let url = format!("{}/v0/ln-address/send-payment", &self.domain);
        let resp = self
            .add_headers(self.reqw_cli.post(&url))
            .json(payment)
            .send()
            .await?;

        self.parse_response(resp).await
    }

    /// Create a Charge / Payment Request QR code for a Lightning Address
    pub async fn fetch_charge_ln_address(
        &self,
        payment: &LnFetchCharge,
    ) -> Result<FetchLnChargeResponse> {
        let url = format!("{}/v0/ln-address/fetch-charge", &self.domain);

        let resp = self
            .add_headers(self.reqw_cli.post(&url))
            .json(payment)
            .send()
            .await?;

        self.parse_response(resp).await
    }

    /// Validate whether a user's entered Lightning Address is indeed a real Lightning Address
    pub async fn validate_ln_address(
        &self,
        lightning_address: &LnAddress,
    ) -> Result<ValidateLnAddrResponse> {
        lightning_address.validate().map_err(|e| {
            ErrorMsg::BadLnAddress(lightning_address.address.clone(), e.to_string())
        })?;

        let url = format!(
            "{}/v0/ln-address/validate/{}",
            &self.domain, &lightning_address.address
        );

        let resp = self.add_headers(self.reqw_cli.get(&url)).send().await?;

        self.parse_response(resp).await
    }

    /// Pays a Charge / Payment Request in the Bitcoin Lightning Network
    pub async fn pay_invoice(&self, payment: &Payment) -> Result<PaymentInvoiceResponse> {
        let url = format!("{}/v0/payments", &self.domain);

        let resp = self
            .add_headers(self.reqw_cli.post(&url))
            .json(&payment)
            .send()
            .await?;

        self.parse_response(resp).await
    }

    pub async fn get_payments(&self) -> Result<FetchPaymentsResponse> {
        let url = format!("{}/v0/payments", &self.domain);
        let resp = self.add_headers(self.reqw_cli.get(&url)).send().await?;
        println!("{:#?}", resp);
        self.parse_response(resp).await
    }

    /// Retrieves all the information related to a specific Payment
    pub async fn get_payment<T>(&self, payment_id: T) -> Result<FetchOnePaymentsResponse>
    where
        T: AsRef<str>,
    {
        let url = format!("{}/v0/payments/{}", &self.domain, payment_id.as_ref());
        let resp = self.add_headers(self.reqw_cli.get(&url)).send().await?;
        self.parse_response(resp).await
    }

    /// Check if provided ip address will be [supported](https://zebedee.io/countries) by Zebedee REST API
    pub async fn get_is_supported_region_by_ip<T>(&self, ip: T) -> Result<SupportedIpResponse>
    where
        T: AsRef<str>,
    {
        let url = format!("{}/v0/is-supported-region/{}", &self.domain, ip.as_ref());
        let resp = self.add_headers(self.reqw_cli.get(&url)).send().await?;
        self.parse_response(resp).await
    }

    /// Check if callback response is from legit Zebedee ip address
    pub async fn get_prod_ips(&self) -> Result<ProdIpsResponse> {
        let url = format!("{}/v0/prod-ips", &self.domain);
        let resp = self.add_headers(self.reqw_cli.get(&url)).send().await?;
        self.parse_response(resp).await
    }

    /// Get the latest price for Bitcoin in US Dollars.
    /// The exchange rate feed is refreshed every 5 seconds and is based upon a combination of industry-leading
    /// partner exchange providers's price feeds.
    pub async fn get_btc_usd(&self) -> Result<BtcToUsdResponse> {
        let url = format!("{}/v0/btcusd", &self.domain);
        let resp = self.reqw_cli.get(&url).send().await?;
        self.parse_response(resp).await
    }

    /// Withdrawal Requests can be thought of as exact opposites to Charges.
    /// Charges in the ZEBEDEE API are QR codes that represent Payment Requests in the Bitcoin Lightning Network.
    /// These QR codes expect that a payer will scan and perform a payment against it.
    /// ***
    /// `Charges`: Lightning QR codes that YOU SPEND
    /// ***
    /// `Withdrawal Requests`: Lightning QR codes that YOU RECEIVE
    pub async fn create_withdrawal_request(
        &self,
        withdrawal_request: &WithdrawalReqest,
    ) -> Result<CreateWithdrawalResponse> {
        let url = format!("{}/v0/withdrawal-requests", &self.domain);

        let resp = self
            .add_headers(self.reqw_cli.post(&url))
            .json(&withdrawal_request)
            .send()
            .await?;

        self.parse_response(resp).await
    }

    pub async fn get_withdrawal_requests(&self) -> Result<FetchWithdrawalsResponse> {
        let url = format!("{}/v0/withdrawal-requests", &self.domain);
        let resp = self.add_headers(self.reqw_cli.get(&url)).send().await?;
        self.parse_response(resp).await
    }

    /// Retrieves details about a specific Withdrawal Request.
    pub async fn get_withdrawal_request<T>(
        &self,
        withdrawal_id: T,
    ) -> Result<FetchOneWithdrawalResponse>
    where
        T: AsRef<str>,
    {
        let url = format!(
            "{}/v0/withdrawal-requests/{}",
            &self.domain,
            withdrawal_id.as_ref()
        );
        let resp = self.add_headers(self.reqw_cli.get(&url)).send().await?;
        self.parse_response(resp).await
    }

    pub async fn create_auth_url<T>(&self, challenge: T) -> Result<String>
    where
        T: AsRef<str>,
    {
        let url = format!("{}/v1/oauth2/authorize", &self.domain);

        let auth_url = self
            .reqw_cli
            .get(url)
            .header("Content-Type", "application/json")
            .query(&[("client_id", &self.oauth.client_id)])
            .query(&[("response_type", "code")])
            .query(&[("redirect_uri", &self.oauth.redirect_uri)])
            .query(&[("code_challenge_method", "S256")])
            .query(&[("code_challenge", challenge.as_ref())])
            .query(&[("scope", &self.oauth.scope)])
            .query(&[("state", &self.oauth.state)])
            .build()
            .unwrap()
            .url()
            .to_string();

        AuthURL::new(&auth_url).validate()?;

        Ok(auth_url)
    }

    pub async fn fetch_token<A, B>(&self, code: A, verifier: B) -> Result<FetchAccessTokenRes>
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        let payload = FetchTokenBody::new(self, code.as_ref(), verifier.as_ref());
        payload.validate()?;

        let url = format!("{}/v1/oauth2/token", &self.domain);

        let resp = self
            .reqw_cli
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        self.parse_response(resp).await
    }

    /// In order to fetch a new accessToken for a given ZBD User, make sure to use the refreshToken using the token endpoint.
    pub async fn refresh_token<T>(&self, refresh_token: T) -> Result<FetchPostRes>
    where
        T: AsRef<str>,
    {
        let payload = FetchRefresh::new(self, refresh_token.as_ref());
        payload.validate()?;

        let url = format!("{}/v1/oauth2/token", &self.domain);
        let resp = self
            .reqw_cli
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        self.parse_response(resp).await
    }

    /// You can use this API endpoint to fetch information about a given ZBD User, granted you can pass the provided accessToken.

    pub async fn fetch_user_data<T>(&self, token: T) -> Result<StdResp<ZBDUserData>>
    where
        T: AsRef<str>,
    {
        //let mut token_header_string: String = "Bearer ".to_owned();
        //token_header_string.push_str(&bearer_token);

        let url = format!("{}/v1/oauth2/user", &self.domain);

        let resp = self
            .add_headers(self.reqw_cli.get(&url))
            .header("usertoken", token.as_ref())
            .send()
            .await?;

        self.parse_response(resp).await
    }

    /// You can use this API endpoint to fetch information about a given ZBD User's Wallet, granted you can pass the provided accessToken.
    pub async fn fetch_user_wallet_data<T>(&self, token: T) -> Result<StdResp<ZBDUserWalletData>>
    where
        T: AsRef<str>,
    {
        //let mut token_header_string: String = "Bearer ".to_owned();
        //token_header_string.push_str(&bearer_token);

        let url = format!("{}/v1/oauth2/wallet", &self.domain);

        let resp = self
            .add_headers(self.reqw_cli.get(&url))
            .header("usertoken", token.as_ref())
            .send()
            .await?;

        self.parse_response(resp).await
    }
}

#[derive(Default, Clone, Validate, Deserialize, Debug)]
pub struct ZebedeeOauth {
    #[validate(length(equal = 36))]
    client_id: String,
    #[validate(length(equal = 36))]
    secret: String,
    #[validate(url)]
    redirect_uri: String,
    #[validate(length(equal = 36))]
    state: String,
    scope: String,
}

impl ZebedeeOauth {
    fn new(
        client_id: String,
        secret: String,
        redirect_uri: String,
        state: String,
        scope: String,
    ) -> Self {
        ZebedeeOauth {
            client_id,
            secret,
            redirect_uri,
            state,
            scope,
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

    pub fn new_rand() -> Self {
        let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
        Self::new(random_bytes)
    }
}

impl From<&str> for PKCE {
    fn from(value: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(value);
        let hash_result: [u8; 32] = hasher
            .finalize()
            .as_slice()
            .try_into()
            .expect("hashing went wrong from string");
        Self::new(hash_result)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StdResp<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
}
