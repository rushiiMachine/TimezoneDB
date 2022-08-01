use rocket::{Build, fairing, Rocket};
use rocket::fairing::AdHoc;
use rocket_db_pools::{Database, sqlx};

#[derive(Database)]
#[database("main")]
pub struct Db(sqlx::PgPool);

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("src/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        }
        None => Err(rocket),
    }
}

pub fn setup() -> AdHoc {
    AdHoc::on_ignite("SQLx", |rocket| async {
        rocket
            .attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
    })
}
