use chrono::{FixedOffset, Offset, Utc};
use chrono_tz::Tz;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::sqlx::{Executor, query, query_as, Postgres};

use crate::utils::jwt::JwtData;
use crate::utils::snowflake::Snowflake;

#[derive(sqlx::FromRow, Debug)]
pub struct DbUser {
    pub id: i64,
    pub timezone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdateData {
    pub timezone: Option<String>,
}

pub async fn add_user<'c, E>(user: &JwtData, db: E) -> bool
    where
        E: Executor<'c, Database=Postgres>
{
    query("INSERT INTO tz_users (id) VALUES ($1) ON CONFLICT DO NOTHING;")
        .bind(*user.user_id)
        .execute(db)
        .await
        .map_err(|err| println!("error adding user: {:?}", err))
        .is_ok()
}

pub async fn fetch_user<'c, E>(id: Snowflake, db: E) -> Option<DbUser>
    where
        E: Executor<'c, Database=Postgres>
{
    query_as::<_, DbUser>("SELECT * FROM tz_users WHERE id = $1;")
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(|err| println!("error fetching user: {:?}", err))
        .unwrap_or(None)
}

pub async fn exists_user<'c, E>(id: Snowflake, db: E) -> bool
    where
        E: Executor<'c, Database=Postgres>
{
    query("SELECT id FROM tz_users WHERE id = $1;")
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(|err| println!("error fetching exists user: {:?}", err))
        .unwrap_or(None)
        .is_some()
}

pub async fn delete_user<'c, E>(id: Snowflake, db: E) -> bool
    where
        E: Executor<'c, Database=Postgres>,
{
    query("DELETE FROM tz_users WHERE id = $1;")
        .bind(id)
        .execute(db)
        .await
        .map_err(|err| println!("error deleting user: {:?}", err))
        .is_ok()
}

pub async fn update_user<'c, E>(user: &JwtData, data: UserUpdateData, db: E) -> bool
    where
        E: Executor<'c, Database=Postgres>
{
    query("INSERT INTO tz_users (id, timezone) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET timezone = $2;")
        .bind(*user.user_id)
        .bind(data.timezone)
        .execute(db)
        .await
        .map_err(|err| println!("error updating user: {:?}", err))
        .is_ok()
}

pub fn calculate_tz_offset(timezone: &Option<String>) -> String {
    let tz: Tz = timezone.as_ref()
        .map(|tz| tz.parse().ok())
        .unwrap_or(None)
        .unwrap_or(Tz::UTC);
    let offset: FixedOffset = Utc::now().with_timezone(&tz).offset().fix();
    let minutes = offset.local_minus_utc() / 60;
    let hours = minutes / 60;
    let minutes_remainder = minutes % 60;

    let pos_sign = if minutes > 0 { "+" } else { "" };
    if minutes_remainder > 0 {
        format!("{pos_sign}{hours}.{minutes_remainder}")
    } else {
        format!("{pos_sign}{hours}")
    }
}
