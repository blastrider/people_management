use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use rocket_sync_db_pools::{database, diesel};
use diesel_migrations::{MigrationHarness, EmbeddedMigrations};

#[database("user_db")]
pub struct DbConn(diesel::PgConnection);

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub async fn run_migrations(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    let conn = DbConn::get_one(&rocket).await
        .expect("database connection");
    conn.run(|c| {
        c.run_pending_migrations(MIGRATIONS).expect("migrations to run");
    }).await;
    rocket
}

