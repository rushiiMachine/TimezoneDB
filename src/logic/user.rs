use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::{query_as, query};
use crate::database::Db;
use crate::utils::snowflake::Snowflake;

#[derive(sqlx::FromRow)]
pub struct DbUser {
    pub id: i64,
    pub username: String,
    pub avatar_hash: String,
    pub timezone: Option<String>,
    pub offset: Option<String>,
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
