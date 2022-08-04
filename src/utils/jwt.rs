use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Serialize};

use crate::constants;
use crate::utils::snowflake::ApiSnowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtData {
    #[serde(rename = "userId")]
    pub user_id: ApiSnowflake,
}

#[derive(Debug)]
pub enum JwtDataError {
    Invalid,
    Missing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtData {
    type Error = JwtDataError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.cookies().get("loginInfo") {
            Some(cookie) => {
                match verify_token(cookie.value()) {
                    Ok(data) =>
                        Outcome::Success(data),
                    Err(_) =>
                        Outcome::Failure((Status::Unauthorized, JwtDataError::Invalid)),
                }
            }
            None =>
                Outcome::Failure((Status::Unauthorized, JwtDataError::Missing))
        }
    }
}

pub fn make_token(data: JwtData) -> String {
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };

    Token::new(header, data)
        .sign_with_key(&*constants::JWT_KEY).unwrap()
        .as_str().to_string()
}

pub fn verify_token(token_str: &str) -> Result<JwtData, Box<dyn std::error::Error>> {
    let token: Token<Header, JwtData, _> = Token::parse_unverified(&token_str)?
        .verify_with_key(&*constants::JWT_KEY)?;
    Ok(token.claims().clone())
}
