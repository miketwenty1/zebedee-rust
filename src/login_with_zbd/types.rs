use crate::ZebedeeClient;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FetchPostRes {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Clone, Validate, Deserialize, Debug)]
pub struct AuthURL<'a> {
    #[validate(url)]
    pub url: Cow<'a, str>,
}

impl<'a> AuthURL<'a> {
    pub fn new<T>(url: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self { url: url.into() }
    }
}

/// Use this struct to create a well crafted json body for token management with ZBD Oauth
#[derive(Serialize, Clone, Validate, Deserialize, Debug)]
pub struct FetchTokenBody<'a> {
    #[validate(length(equal = 36))]
    pub client_id: Cow<'a, str>,
    #[validate(length(equal = 36))]
    pub client_secret: Cow<'a, str>,
    #[validate(length(equal = 36))]
    pub code: Cow<'a, str>,
    #[validate(length(equal = 43))]
    pub code_verifier: Cow<'a, str>,
    #[validate(length(min = 1))]
    pub grant_type: Cow<'a, str>,
    #[validate(url)]
    pub redirect_uri: Cow<'a, str>,
}

impl<'a> FetchTokenBody<'a> {
    pub fn new<A, B>(zc: &'a ZebedeeClient, code: A, code_verifier: B) -> Self
    where
        A: Into<Cow<'a, str>>,
        B: Into<Cow<'a, str>>,
    {
        FetchTokenBody {
            client_id: zc.oauth.client_id.as_str().into(),
            client_secret: zc.oauth.secret.as_str().into(),
            code: code.into(),
            code_verifier: code_verifier.into(),
            grant_type: "authorization_code".into(),
            redirect_uri: zc.oauth.redirect_uri.as_str().into(),
        }
    }
}
// COMMENTED OUT BECAUSE API MAY BE UPDATED TO LOOK LIKE THIS PER DOCS.
// #[derive(Serialize, Validate, Deserialize, Debug)]
// pub struct FetchAccessTokenRes {
//     #[serde(rename = "accessToken")]
//     pub access_token: String,
//     #[serde(rename = "usertoken")]
//     token_type: String,
//     #[serde(rename = "accessTokenExpirationDate")]
//     pub access_token_expiration_date: Option<DateTime<Utc>>,
//     #[serde(rename = "additionalParameters")]
//     additional_parameters: FetchATAdditionalParams,
//     #[serde(rename = "idToken")]
//     id_token: Option<String>,
//     #[serde(rename = "refreshToken")]
//     refresh_token: String,

// }
// #[derive(Serialize, Validate, Deserialize, Debug)]
// pub struct FetchATAdditionalParams {
//     pub refresh_token_expires_in: i32
// }

#[derive(Serialize, Validate, Deserialize, Debug)]
pub struct FetchAccessTokenRes {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub refresh_token_expires_in: u32,
    pub scope: String,
}

/// Use this struct to create a well crafted json body for token refreshes with ZBD Oauth
#[derive(Serialize, Validate, Deserialize, Debug)]
pub struct FetchRefresh {
    #[validate(length(equal = 36))]
    pub client_id: String,
    #[validate(length(equal = 36))]
    pub client_secret: String,
    #[validate(length(equal = 36))]
    pub refresh_token: String,
    #[validate(length(min = 1))]
    pub grant_type: String,
    #[validate(url)]
    pub redirect_uri: String,
}

impl FetchRefresh {
    pub fn new(zc: ZebedeeClient, refresh_token: String) -> Self {
        FetchRefresh {
            client_id: zc.oauth.client_id,
            client_secret: zc.oauth.secret,
            grant_type: String::from("refresh_token"),
            redirect_uri: zc.oauth.redirect_uri,
            refresh_token,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZBDUserData {
    pub id: String,
    pub email: String,
    pub gamertag: String,
    pub image: Option<String>,
    #[serde(rename = "isVerified")]
    pub is_verified: bool,
    #[serde(rename = "lightningAddress")]
    pub lightning_address: String,
    #[serde(rename = "publicBio")]
    pub public_bio: String,
    #[serde(rename = "publicStaticCharge")]
    pub public_static_charge: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZBDUserWalletData {
    pub balance: String,
    #[serde(rename = "remainingAmountLimits")]
    pub remaining_amount_limits: ZBDUserWalletDataLimits,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZBDUserWalletDataLimits {
    pub daily: String,
    #[serde(rename = "maxCredit")]
    pub max_credit: String,
    pub monthly: String,
    pub weekly: String,
}
