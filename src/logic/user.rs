use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::{query_as, query};
use rocket::serde::{Serialize, Deserialize};
use crate::database::Db;
use crate::JwtData;
use crate::utils::snowflake::Snowflake;

#[derive(sqlx::FromRow, Debug)]
pub struct DbUser {
    pub id: i64,
    pub username: String,
    pub avatar_hash: String,
    pub timezone: Option<String>,
    pub offset: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdateData {
    pub timezone: Option<String>,
    pub offset: Option<String>
}

pub async fn add_user(user: &JwtData, mut db: Connection<Db>) -> bool {
    query("INSERT OR REPLACE INTO users (id, username, avatar_hash) VALUES (?, ?, ?);")
        .bind(*user.user_id)
        .bind(&user.username)
        .bind(&user.avatar_hash)
        .execute(&mut *db)
        .await
        .map_err(|err| println!("error adding user: {:?}", err))
        .is_ok()
}

pub async fn fetch_user(id: Snowflake, mut db: Connection<Db>) -> Option<DbUser> {
    query_as::<_, DbUser>("SELECT * FROM users WHERE id = ?;")
        .bind(id)
        .fetch_optional(&mut *db)
        .await
        .map_err(|err| println!("error fetching user: {:?}", err))
        .unwrap_or(None)
}

pub async fn exists_user(id: Snowflake, mut db: Connection<Db>) -> bool {
    query("SELECT id FROM users WHERE id = ?;")
        .bind(id)
        .fetch_optional(&mut *db)
        .await
        .map_err(|err| println!("error fetching exists user: {:?}", err))
        .unwrap_or(None)
        .is_some()
}

pub async fn delete_user(id: Snowflake, mut db: Connection<Db>) -> bool {
    query("DELETE FROM users WHERE id = ?;")
        .bind(id)
        .execute(&mut *db)
        .await
        .map_err(|err| println!("error deleting user: {:?}", err))
        .is_ok()
}

pub async fn update_user(user: JwtData, data: UserUpdateData, mut db: Connection<Db>) -> bool {
    // if !add_user(&user, &mut *db).await {
    //     return false;
    // }

    query("UPDATE users SET timezone = ?, offset = ? WHERE id = ?;")
        .bind(data.timezone)
        .bind(data.offset)
        .bind(*user.user_id)
        .execute(&mut *db)
        .await
        .map_err(|err| println!("error updating user: {:?}", err))
        .is_ok()
}
