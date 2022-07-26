use std::env;

use reqwest::Client;

static HTTP: Client = Client::new();

#[derive(Serialize)]
struct OAuthRequestData {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
    redirect_uri: String,
    scope: String,
}

#[derive(Serialize, Deserialize)]
struct OAuthResponseData {
    access_token: String,
    expires_in: i32,
    refresh_token: String,
    scope: String,
    token_type: String,
}

pub async fn complete_oauth_flow(oauth_token: String) -> OAuthResponseData {
    let request_data = OAuthRequestData {
        client_id: env::var("DISCORD_ID").unwrap(),
        client_secret: env::var("DISCORD_SECRET").unwrap(),
        code: oauth_token,
        grant_type: "authorization_code".to_string(),
        redirect_uri: "http://localhost:8000/api/auth".to_string(),
        scope: "identify".to_string(),
    };

    HTTP.post("https://discord.com/api/oauth2/token")
        .body(serde_urlencoded::to_string(request_data).unwrap())
        .build()
        .unwrap()
        .json::<OAuthResponseData>()
}
