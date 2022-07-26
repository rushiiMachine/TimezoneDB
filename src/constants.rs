use std::env;
use std::str::FromStr;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref PORT: u16 = match env::var("PORT") {
        Ok(var) => match u16::from_str(&var) {
            Ok(val) => val,
            Err(_) => panic!("invalid PORT env var"),
        },
        Err(_) => 8000,
    };

    pub static ref ENV: String = match env::var("ENV") {
        Ok(var) => var,
        Err(_) => "development".to_string(),
    };

    pub static ref PROD: bool = if *ENV == "production" { true } else { false };

    pub static ref HOST: String = match env::var("HOST") {
        Ok(var) => var,
        Err(_) => match *PROD {
            true => panic!("cannot run in production without HOST env var"),
            false => format!("http://localhost:{0}", *PORT),
        }
    };

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
