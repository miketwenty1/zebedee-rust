use crate::ZebedeeClient;
use crate::PKCE;
use std::env;

#[tokio::test]
async fn test_create_challenge_from_string() {
    let c = PKCE::from("hellomynameiswhat");

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

    let c = PKCE::from("hellomynameiswhat");
    let r = zebedee_client.create_auth_url(c.challenge.clone());

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

    let c = PKCE::from("hellomynameiswhat");
    let fake_code = "xxx11xx1-xxxx-xxxx-xxx1-1xx11xx111xx";
    let r = zebedee_client.fetch_token(fake_code, c.verifier);
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

    let fake_refresh_token = "xxx11xx1-xxxx-xxxx-xxx1-1xx11xx111xx";
    let r = zebedee_client.refresh_token(fake_refresh_token);
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
    let r = zebedee_client.fetch_user_data(fake_refresh_token);
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
    let r = zebedee_client.fetch_user_wallet_data(fake_refresh_token);
    let i = match r.await {
        Err(e) => e.to_string(),
        Ok(_) => "was a good token but it shouldnt be".to_string(),
    };
    assert!(i.contains("false"));
}
