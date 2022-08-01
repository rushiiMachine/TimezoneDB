use std::env;
use std::str::FromStr;

use hmac::digest::KeyInit;
use hmac::Hmac;
use lazy_static::lazy_static;
use sha2::Sha384;

lazy_static! {
    pub static ref PORT: u16 = match env::var("PORT") {
        Ok(var) => match u16::from_str(&var) {
            Ok(val) => match cfg!(debug_assertions) {
                true if val != 8000 => panic!("cannot set custom port in debug due to hardcoded redirects"),
                _ => val
            },
            Err(_) => panic!("invalid PORT env var"),
        },
        Err(_) => 8000,
    };

    pub static ref HOST: String = match env::var("HOST") {
        Ok(var) => var,
        Err(_) => match !cfg!(debug_assertions) {
            true => panic!("cannot run in production without HOST env var"),
            false => format!("http://localhost:{0}", *PORT),
        }
    };

    pub static ref JWT_KEY: Hmac<Sha384> = {
        let secret = match env::var("JWT_SECRET") {
            Ok(var) => var,
            Err(_) => match !cfg!(debug_assertions)  {
                true => panic!("cannot run in production without JWT_SECRET env var"),
                false => "timezone_db".to_string(),
            }
        };
        Hmac::new_from_slice(secret.as_bytes()).unwrap()
    };

    pub static ref POSTGRES_URL: String = env::var("POSTGRES_URL")
        .expect("missing env POSTGRES_URL");
    pub static ref DISCORD_ID: String = env::var("DISCORD_ID")
        .expect("missing env DISCORD_ID");
    pub static ref DISCORD_SECRET: String = env::var("DISCORD_SECRET")
        .expect("missing env DISCORD_SECRET");

    pub static ref DISCORD_REDIRECT_URI: String =
        format!("{0}/api/auth", *HOST);
    pub static ref DISCORD_OAUTH_URL: String = format!(
        "https://discord.com/oauth2/authorize?client_id={0}&redirect_uri={1}&response_type=code&scope=identify",
        *DISCORD_ID,
        urlencoding::encode(&*DISCORD_REDIRECT_URI)
    );
}
