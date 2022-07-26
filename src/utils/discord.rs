use std::fmt::{Display, Formatter};

use lazy_static::lazy_static;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::constants;
use crate::utils::snowflake::ApiSnowflake;

lazy_static! {
    static ref HTTP: Client = Client::new();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordApiErrorData {
    pub error: String,
    pub error_description: String,
}

impl Display for DiscordApiErrorData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DiscordApiError(error: {0:?}, description: {1:?})", self.error, self.error_description)
    }
}

#[derive(Error, Debug)]
pub enum DiscordApiError {
    #[error("api error: {0}")]
    ApiError(DiscordApiErrorData),
    #[error("unserializable error, status: {0}, data: `{1}`")]
    Unserializable(u16, String),
    #[error("unknown error, status: `{0}`")]
    Unknown(u16),
}

#[derive(Serialize)]
struct OAuthRequestData<'a> {
    client_id: &'a String,
    client_secret: &'a String,
    redirect_uri: &'a String,
    code: &'a String,
    grant_type: &'a String,
    scope: &'a String,
}

#[derive(Deserialize, Debug)]
pub struct OAuthResponseData {
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
}

pub async fn complete_oauth_flow(oauth_token: String) -> Result<OAuthResponseData, Box<dyn std::error::Error>> {
    let request_data = OAuthRequestData {
        client_id: &constants::DISCORD_ID,
        client_secret: &constants::DISCORD_SECRET,
        code: &oauth_token,
        redirect_uri: &*constants::DISCORD_REDIRECT_URI,
        grant_type: &"authorization_code".to_string(),
        scope: &"identify".to_string(),
    };

    let request = HTTP.post("https://discord.com/api/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(serde_urlencoded::to_string(request_data)?)
        .build()?;

    let response = HTTP.execute(request)
        .await?;

    match response.status() {
        StatusCode::OK => {
            let response_data = response
                .json::<OAuthResponseData>()
                .await?;

            Ok(response_data)
        }
        StatusCode::BAD_REQUEST => {
            let error = match response.json::<DiscordApiErrorData>().await {
                Ok(data) =>
                    DiscordApiError::ApiError(data),
                Err(_) =>
                    DiscordApiError::Unknown(400),
            };
            Err(Box::new(error))
        }
        status => {
            let error = match response.text().await {
                Ok(body) =>
                    DiscordApiError::Unserializable(status.as_u16(), body),
                Err(_) =>
                    DiscordApiError::Unknown(status.as_u16())
            };
            Err(Box::new(error))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordUser {
    pub id: ApiSnowflake,
    pub avatar: String,
    pub username: String,
}

pub async fn get_current_user(authorization: &String) -> Result<DiscordUser, Box<dyn std::error::Error>> {
    let request = HTTP.get("https://discord.com/api/v9/users/@me")
        .header("Authorization", authorization)
        .build()?;

    let response_data = HTTP.execute(request)
        .await?
        .json::<DiscordUser>()
        .await?;

    Ok(response_data)
}
