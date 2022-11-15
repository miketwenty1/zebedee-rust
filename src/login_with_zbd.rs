use crate::ZebedeeClient;

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
pub struct FetchPost {
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

impl FetchPost {
    pub fn new(zc: ZebedeeClient, code: String, code_verifier: String) -> Self {
        FetchPost {
            client_id: zc.oauth.client_id,
            client_secret: zc.oauth.secret,
            code,
            code_verifier,
            grant_type: String::from("authorization_code"),
            redirect_uri: zc.oauth.redirect_uri,
        }
    }
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
pub struct FetchUserRes {
    pub success: bool,
    pub data: FetchUserData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FetchUserData {
    pub id: String,
    pub email: String,
    pub gamertag: String,
    pub image: String,
    #[serde(rename = "isVerified")]
    pub is_verified: bool,
    #[serde(rename = "lightningAddress")]
    pub lightning_address: String,
    #[serde(rename = "publicBio")]
    pub public_bio: String,
    #[serde(rename = "publicStaticCharge")]
    pub public_static_charge: String,
}

#[tokio::main]
pub async fn create_auth_url(
    client: ZebedeeClient,
    challenge: String,
) -> Result<String, anyhow::Error> {
    let url = String::from("https://api.zebedee.io/v0/oauth2/authorize");
    let auth_url = client
        .reqw_cli
        .get(&url)
        .header("Content-Type", "application/json")
        .query(&[("client_id", client.oauth.clone().client_id)])
        .query(&[("response_type", "code")])
        .query(&[("redirect_uri", client.oauth.redirect_uri)])
        .query(&[("code_challenge_method", "S256")])
        .query(&[("code_challenge", challenge)])
        .query(&[("scope", "user")])
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

#[tokio::main]
pub async fn fetch_token(
    client: ZebedeeClient,
    payload: FetchPost,
) -> Result<FetchPostRes, anyhow::Error> {
    payload.validate()?;

    let url = format!("https://api.zebedee.io/v0/oauth2/token");
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
            resp_text.clone(),
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
                resp_text.clone(),
                status_code
            ))
        }
    };

    Ok(resp_seralized_2)
}

#[tokio::main]
pub async fn refresh_token(
    client: ZebedeeClient,
    payload: FetchRefresh,
) -> Result<FetchPostRes, anyhow::Error> {
    payload.validate()?;

    let url = format!("https://api.zebedee.io/v0/oauth2/token");
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
            resp_text.clone(),
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
                resp_text.clone(),
                status_code
            ))
        }
    };

    Ok(resp_seralized_2)
}

#[tokio::main]
pub async fn fetch_user_data(
    client: ZebedeeClient,
    bearer_token: String,
) -> Result<FetchUserRes, anyhow::Error> {
    let mut token_header_string: String = "Bearer ".to_owned();
    token_header_string.push_str(&bearer_token);

    let url = String::from("https://api.zebedee.io/v0/oauth2/user");
    let resp = client
        .reqw_cli
        .get(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", token_header_string)
        .send()
        .await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;

    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text.clone(),
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
                resp_text.clone(),
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

    #[test]
    fn test_create_challenge_from_string() {
        let c = PKCE::new_from_string(String::from("hellomynameiswhat"));

        assert_eq!(
            c.challenge,
            String::from("mBc-M8x_JG5qARgND5Vzx7fPu1EjZlapL_dVg4BjrkU")
        );
    }
    #[test]
    fn test_create_challenge_rand() {
        let c = PKCE::new_rand();

        assert_eq!(c.challenge, c.challenge);
    }
    #[test]
    fn test_create_oauth_auth_url() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let oauth_client_id: String = env::var("ZBD_OAUTH_CLIENT_ID").unwrap();
        let oauth_secret: String = env::var("ZBD_OAUTH_SECRET").unwrap();
        let redirect_uri: String = env::var("ZBD_REDIRECT_URI").unwrap();

        let zebedee_client = ZebedeeClient::new(apikey)
            .set_oauth(oauth_client_id, oauth_secret, redirect_uri)
            .unwrap();

        let c = PKCE::new_from_string(String::from("hellomynameiswhat"));
        let r = create_auth_url(zebedee_client, c.challenge.clone());

        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_fetch_token() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let oauth_client_id: String = env::var("ZBD_OAUTH_CLIENT_ID").unwrap();
        let oauth_secret: String = env::var("ZBD_OAUTH_SECRET").unwrap();
        let redirect_uri: String = env::var("ZBD_REDIRECT_URI").unwrap();

        let zebedee_client = ZebedeeClient::new(apikey)
            .set_oauth(oauth_client_id, oauth_secret, redirect_uri)
            .unwrap();

        let c = PKCE::new_from_string(String::from("hellomynameiswhat"));
        let fake_code = String::from("xxx11xx1-xxxx-xxxx-xxx1-1xx11xx111xx");
        let fetchbody = FetchPost::new(zebedee_client.clone(), fake_code, c.verifier);
        let r = fetch_token(zebedee_client, fetchbody);
        let mut i = String::from("");
        match r {
            Err(e) => {
                i = e.to_string();
            }
            _ => (),
        }
        assert_eq!(i.contains("Error validating challenge"), true);
    }
    #[test]
    fn test_refresh_token() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let oauth_client_id: String = env::var("ZBD_OAUTH_CLIENT_ID").unwrap();
        let oauth_secret: String = env::var("ZBD_OAUTH_SECRET").unwrap();
        let redirect_uri: String = env::var("ZBD_REDIRECT_URI").unwrap();

        let zebedee_client = ZebedeeClient::new(apikey)
            .set_oauth(oauth_client_id, oauth_secret, redirect_uri)
            .unwrap();

        let fake_refresh_token = String::from("xxx11xx1-xxxx-xxxx-xxx1-1xx11xx111xx");
        let fetchbody = FetchRefresh::new(zebedee_client.clone(), fake_refresh_token);
        let r = refresh_token(zebedee_client, fetchbody);
        let mut i = String::from("");
        match r {
            Err(e) => {
                i = e.to_string();
            }
            _ => (),
        }
        assert_eq!(i.contains("Error requesting token"), true);
    }
    #[test]
    fn test_fetch_user_data() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let oauth_client_id: String = env::var("ZBD_OAUTH_CLIENT_ID").unwrap();
        let oauth_secret: String = env::var("ZBD_OAUTH_SECRET").unwrap();
        let redirect_uri: String = env::var("ZBD_REDIRECT_URI").unwrap();

        let zebedee_client = ZebedeeClient::new(apikey)
            .set_oauth(oauth_client_id, oauth_secret, redirect_uri)
            .unwrap();

        let fake_refresh_token = String::from("eyAAAAyomommagotocollegeAAAxxxXXAAAAasdfasdfsas");
        let r = fetch_user_data(zebedee_client, fake_refresh_token);
        let mut i = String::from("");
        match r {
            Err(e) => {
                i = e.to_string();
                println!("{}", i);
            }
            _ => (),
        }

        assert_eq!(i.contains("Bad token"), true);
        //assert_eq!(r.unwrap().data.email, "tyler@z.com");
    }
}
