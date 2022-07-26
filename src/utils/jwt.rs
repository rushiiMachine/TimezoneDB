use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};

use crate::constants;
use crate::utils::snowflake::ApiSnowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtData {
    pub user_id: ApiSnowflake,
    pub avatar_hash: String,
    pub username: String,
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

pub fn verify_token(token_str: &String) -> Result<JwtData, Box<dyn std::error::Error>> {
    let token: Token<Header, JwtData, _> = Token::parse_unverified(&token_str)?
        .verify_with_key(&*constants::JWT_KEY)?;
    Ok(token.claims().clone())
}
