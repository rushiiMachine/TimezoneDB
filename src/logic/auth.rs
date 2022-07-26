use crate::utils;
use crate::utils::discord;
use crate::utils::jwt::JwtData;

/// 1. Gets an access token for an oauth return code
/// 2. Fetch the current user from Discord
/// 3. Insert/update in DB
/// 5. Make + return JWT token
pub async fn login_user(oauth_code: String) -> Result<String, Box<dyn std::error::Error>> {
    let oauth_data = discord::complete_oauth_flow(oauth_code).await?;

    let authorization = format!("{0} {1}", oauth_data.token_type, oauth_data.access_token);

    let user = discord::get_current_user(&authorization).await?;

    // add to db

    let jwt_data = JwtData {
        user_id: user.id,
        avatar_hash: user.avatar,
        username: user.username,
    };
    let token = utils::jwt::make_token(jwt_data);
    Ok(token)
}