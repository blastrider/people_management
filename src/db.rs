use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket_sync_db_pools::{database, diesel};

#[database("user_db")]
pub struct DbConn(diesel::PgConnection);

pub const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("./migrations/2024-08-17-204852_create_users");

pub async fn run_migrations(
    rocket: rocket::Rocket<rocket::Build>,
) -> rocket::Rocket<rocket::Build> {
    let conn = DbConn::get_one(&rocket).await.expect("database connection");
    conn.run(|c| {
        c.run_pending_migrations(MIGRATIONS)
            .expect("migrations to run");
    })
    .await;
    rocket
}
