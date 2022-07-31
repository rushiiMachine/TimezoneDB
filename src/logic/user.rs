use chrono::{FixedOffset, Offset, Utc};
use chrono_tz::Tz;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::sqlx::{Executor, query, query_as, Sqlite};

use crate::utils::jwt::JwtData;
use crate::utils::snowflake::Snowflake;

#[derive(sqlx::FromRow, Debug)]
pub struct DbUser {
    pub id: i64,
    pub username: String,
    pub avatar_hash: String,
    pub timezone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdateData {
    pub timezone: Option<String>,
}

pub async fn add_user<'c, E>(user: &JwtData, db: E) -> bool
    where
        E: Executor<'c, Database=Sqlite>
{
    query("INSERT OR REPLACE INTO users (id, username, avatar_hash) VALUES (?, ?, ?);")
        .bind(*user.user_id)
        .bind(&user.username)
        .bind(&user.avatar_hash)
        .execute(db)
        .await
        .map_err(|err| println!("error adding user: {:?}", err))
        .is_ok()
}

pub async fn fetch_user<'c, E>(id: Snowflake, db: E) -> Option<DbUser>
    where
        E: Executor<'c, Database=Sqlite>
{
    query_as::<_, DbUser>("SELECT * FROM users WHERE id = ?;")
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(|err| println!("error fetching user: {:?}", err))
        .unwrap_or(None)
}

pub async fn exists_user<'c, E>(id: Snowflake, db: E) -> bool
    where
        E: Executor<'c, Database=Sqlite>
{
    query("SELECT id FROM users WHERE id = ?;")
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(|err| println!("error fetching exists user: {:?}", err))
        .unwrap_or(None)
        .is_some()
}

pub async fn delete_user<'c, E>(id: Snowflake, db: E) -> bool
    where
        E: Executor<'c, Database=Sqlite>,
{
    query("DELETE FROM users WHERE id = ?;")
        .bind(id)
        .execute(db)
        .await
        .map_err(|err| println!("error deleting user: {:?}", err))
        .is_ok()
}

pub async fn update_user<'c, E>(user: &JwtData, data: UserUpdateData, db: E) -> bool
    where
        E: Executor<'c, Database=Sqlite>
{
    query("UPDATE users SET timezone = ? WHERE id = ?;")
        .bind(data.timezone)
        .bind(*user.user_id)
        .execute(db)
        .await
        .map_err(|err| println!("error updating user: {:?}", err))
        .is_ok()
}

pub fn calculate_tz_offset(timezone: &Option<String>) -> String {
    let tz: Tz = timezone.as_ref().unwrap_or(&"UTC".to_string())
        .parse().unwrap_or(Tz::UTC);
    let offset: FixedOffset = Utc::now().with_timezone(&tz).offset().fix();
    let offset = offset.local_minus_utc() / (60 * 60);
    offset.to_string()
}
