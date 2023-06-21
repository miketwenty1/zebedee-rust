use crate::{StdResp, ZebedeeClient};

use anyhow::Result;
use serde::{Deserialize, Serialize};
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
struct AuthURL {
    #[validate(url)]
    pub url: String,
}

/// Use this struct to create a well crafted json body for token management with ZBD Oauth
#[derive(Serialize, Validate, Deserialize, Debug)]
pub struct FetchTokenBody {
    #[validate(length(equal = 36))]
    pub client_id: String,
    #[validate(length(equal = 36))]
    pub client_secret: String,
    #[validate(length(equal = 36))]
    pub code: String,
    #[validate(length(equal = 43))]
    pub code_verifier: String,
    #[validate(length(min = 1))]
    pub grant_type: String,
    #[validate(url)]
    pub redirect_uri: String,
}

impl FetchTokenBody {
    pub fn new(zc: ZebedeeClient, code: String, code_verifier: String) -> Self {
        FetchTokenBody {
            client_id: zc.oauth.client_id,
            client_secret: zc.oauth.secret,
            code,
            code_verifier,
            grant_type: String::from("authorization_code"),
            redirect_uri: zc.oauth.redirect_uri,
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

pub async fn create_auth_url(
    client: ZebedeeClient,
    challenge: String,
) -> Result<String, anyhow::Error> {
    let url = format!("{}/v1/oauth2/authorize", client.domain);
    let auth_url = client
        .reqw_cli
        .get(url)
        .header("Content-Type", "application/json")
        .query(&[("client_id", client.oauth.client_id)])
        .query(&[("response_type", "code")])
        .query(&[("redirect_uri", client.oauth.redirect_uri)])
        .query(&[("code_challenge_method", "S256")])
        .query(&[("code_challenge", challenge)])
        .query(&[("scope", client.oauth.scope)])
        .query(&[("state", client.oauth.state)])
        .build()
        .unwrap()
        .url()
        .to_string();

    let valid_url = AuthURL {
        url: auth_url.clone(),
    };
    valid_url.validate()?;

    Ok(auth_url)
}

pub async fn fetch_token(
    client: ZebedeeClient,
    payload: FetchTokenBody,
) -> Result<FetchAccessTokenRes, anyhow::Error> {
    payload.validate()?;

    let url = format!("{}/v1/oauth2/token", client.domain);
    let resp = client
        .reqw_cli
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;

    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text,
            &url,
        ));
    }

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2 = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\n status code: {}",
                e,
                resp_text,
                status_code
            ))
        }
    };

    Ok(resp_seralized_2)
}

pub async fn refresh_token(
    client: ZebedeeClient,
    payload: FetchRefresh,
) -> Result<FetchPostRes, anyhow::Error> {
    payload.validate()?;

    let url = format!("{}/v1/oauth2/token", client.domain);
    let resp = client
        .reqw_cli
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;

    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text,
            &url,
        ));
    }

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2 = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\n status code: {}",
                e,
                resp_text,
                status_code
            ))
        }
    };

    Ok(resp_seralized_2)
}

pub async fn fetch_user_data(
    client: ZebedeeClient,
    token: String,
) -> Result<StdResp<ZBDUserData>, anyhow::Error> {
    //let mut token_header_string: String = "Bearer ".to_owned();
    //token_header_string.push_str(&bearer_token);

    let url = format!("{}/v1/oauth2/user", client.domain);
    let req = client
        .reqw_cli
        .get(&url)
        .header("Content-Type", "application/json")
        .header("usertoken", token)
        .header("apikey", client.apikey);
    let resp = req.send().await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;
    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text,
            &url,
        ));
    }

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2 = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\nstatus code: {}\n url: {}",
                e,
                resp_text,
                status_code,
                &url,
            ))
        }
    };

    Ok(resp_seralized_2)
}

pub async fn fetch_user_wallet_data(
    client: ZebedeeClient,
    token: String,
) -> Result<StdResp<ZBDUserWalletData>, anyhow::Error> {
    //let mut token_header_string: String = "Bearer ".to_owned();
    //token_header_string.push_str(&bearer_token);

    let url = format!("{}/v1/oauth2/wallet", client.domain);
    let req = client
        .reqw_cli
        .get(&url)
        .header("Content-Type", "application/json")
        .header("usertoken", token)
        .header("apikey", client.apikey);
    let resp = req.send().await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;

    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text,
            &url,
        ));
    }

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2 = match resp_serialized {
            Ok(c) => c,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\nstatus code: {}\n url: {}",
                    e,
                    resp_text,
                    status_code,
                    &url,
                ))
            }
        };

    Ok(resp_seralized_2)
}
#[cfg(test)]
mod tests {
    use crate::PKCE;

    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_create_challenge_from_string() {
        let c = PKCE::new_from_string(String::from("hellomynameiswhat"));

        assert_eq!(
            c.challenge,
            String::from("mBc-M8x_JG5qARgND5Vzx7fPu1EjZlapL_dVg4BjrkU")
        );
    }
    #[tokio::test]
    async fn test_create_challenge_rand() {
        let c = PKCE::new_rand();

        assert_eq!(c.challenge, c.challenge);
    }
    #[tokio::test]
    async fn test_create_oauth_auth_url() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let oauth_client_id: String = env::var("ZBD_OAUTH_CLIENT_ID").unwrap();
        let oauth_secret: String = env::var("ZBD_OAUTH_SECRET").unwrap();
        let redirect_uri: String = env::var("ZBD_REDIRECT_URI").unwrap();
        let state: String = env::var("ZBD_OAUTH_STATE").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));

        let zebedee_client = ZebedeeClient::new()
            .domain(zbdenv)
            .apikey(apikey)
            .oauth(
                oauth_client_id,
                oauth_secret,
                redirect_uri,
                state,
                String::from("user"),
            )
            .build();

        let c = PKCE::new_from_string(String::from("hellomynameiswhat"));
        let r = create_auth_url(zebedee_client, c.challenge.clone());

        assert!(r.await.is_ok());
    }
    #[tokio::test]
    async fn test_fetch_token() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let oauth_client_id: String = env::var("ZBD_OAUTH_CLIENT_ID").unwrap();
        let oauth_secret: String = env::var("ZBD_OAUTH_SECRET").unwrap();
        let redirect_uri: String = env::var("ZBD_REDIRECT_URI").unwrap();
        let state: String = env::var("ZBD_OAUTH_STATE").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));

        let zebedee_client = ZebedeeClient::new()
            .domain(zbdenv)
            .apikey(apikey)
            .oauth(
                oauth_client_id,
                oauth_secret,
                redirect_uri,
                state,
                String::from("user"),
            )
            .build();

        let c = PKCE::new_from_string(String::from("hellomynameiswhat"));
        let fake_code = String::from("xxx11xx1-xxxx-xxxx-xxx1-1xx11xx111xx");
        let fetchbody = FetchTokenBody::new(zebedee_client.clone(), fake_code, c.verifier);
        let r = fetch_token(zebedee_client, fetchbody);
        //let mut i = String::from("");
        let i = match r.await {
            Err(e) => e.to_string(),

            Ok(_) => "it worked but how?".to_string(),
        };
        assert!(i.is_ascii());
    }
    #[tokio::test]
    async fn test_refresh_token() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let oauth_client_id: String = env::var("ZBD_OAUTH_CLIENT_ID").unwrap();
        let oauth_secret: String = env::var("ZBD_OAUTH_SECRET").unwrap();
        let redirect_uri: String = env::var("ZBD_REDIRECT_URI").unwrap();
        let state: String = env::var("ZBD_OAUTH_STATE").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));

        let zebedee_client = ZebedeeClient::new()
            .domain(zbdenv)
            .apikey(apikey)
            .oauth(
                oauth_client_id,
                oauth_secret,
                redirect_uri,
                state,
                String::from("user"),
            )
            .build();

        let fake_refresh_token = String::from("xxx11xx1-xxxx-xxxx-xxx1-1xx11xx111xx");
        let fetchbody = FetchRefresh::new(zebedee_client.clone(), fake_refresh_token);
        let r = refresh_token(zebedee_client, fetchbody);
        let i = match r.await {
            Err(e) => e.to_string(),
            Ok(_) => "this worked but how?".to_string(),
        };

        assert!(i.is_ascii());
    }
    #[tokio::test]
    async fn test_fetch_user_data() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let oauth_client_id: String = env::var("ZBD_OAUTH_CLIENT_ID").unwrap();
        let oauth_secret: String = env::var("ZBD_OAUTH_SECRET").unwrap();
        let redirect_uri: String = env::var("ZBD_REDIRECT_URI").unwrap();
        let state: String = env::var("ZBD_OAUTH_STATE").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));

        let zebedee_client = ZebedeeClient::new()
            .domain(zbdenv)
            .apikey(apikey)
            .oauth(
                oauth_client_id,
                oauth_secret,
                redirect_uri,
                state,
                String::from("user"),
            )
            .build();

        let fake_refresh_token = String::from("eyAAAAyomommagotocollegeAAAxxxXXAAAAasdfasdfsas");
        let r = fetch_user_data(zebedee_client, fake_refresh_token);
        let i = match r.await {
            Err(e) => e.to_string(),
            Ok(_) => "was a good token but it shouldnt be".to_string(),
        };
        assert!(i.contains("Token is either invalid or expired"));
    }
    #[tokio::test]
    async fn test_fetch_user_wallet_data() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let oauth_client_id: String = env::var("ZBD_OAUTH_CLIENT_ID").unwrap();
        let oauth_secret: String = env::var("ZBD_OAUTH_SECRET").unwrap();
        let redirect_uri: String = env::var("ZBD_REDIRECT_URI").unwrap();
        let state: String = env::var("ZBD_OAUTH_STATE").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));

        let zebedee_client = ZebedeeClient::new()
            .domain(zbdenv)
            .apikey(apikey)
            .oauth(
                oauth_client_id,
                oauth_secret,
                redirect_uri,
                state,
                String::from("user,wallet"),
            )
            .build();

        let fake_refresh_token = String::from("eyAAAAyomommagotocollegeAAAxxxXXAAAAasdfasdfsas");
        let r = fetch_user_wallet_data(zebedee_client, fake_refresh_token);
        let i = match r.await {
            Err(e) => e.to_string(),
            Ok(_) => "was a good token but it shouldnt be".to_string(),
        };
        assert!(i.contains("401"));
    }
}
