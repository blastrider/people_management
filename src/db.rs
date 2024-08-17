use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use rocket_sync_db_pools::{database, diesel};

#[database("user_db")]
pub struct DbConn(diesel::PgConnection);

embed_migrations!();

pub async fn run_migrations(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    let conn = DbConn::get_one(&rocket).await
        .expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await
        .expect("can run migrations");
    rocket
}

