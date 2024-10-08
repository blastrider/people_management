#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod db;
mod models;
mod routes;
mod schema;

use rocket::fairing::AdHoc;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::str::FromStr;

#[launch]
fn rocket() -> _ {
    // Spécifie les origines autorisées
    let allowed_origins = AllowedOrigins::some_exact(&["http://127.0.0.1:8080"]);

    // Configure les options CORS
    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Put, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .to_cors()
        .expect("CORS configuration failed");

    rocket::build()
        .attach(cors) // Attache le middleware CORS
        .attach(db::DbConn::fairing())
        .attach(AdHoc::on_ignite("Database Migrations", db::run_migrations))
        .mount(
            "/api",
            routes![
                routes::users::create::create_user,
                routes::users::update::update_user,
                routes::users::delete::delete_user,
                routes::users::shared_management::add_shared_account,
                routes::users::shared_management::remove_shared_account,
                routes::users::shared_management::get_shared_users,
                routes::users::inactive_management::deactivate_user,
                routes::users::inactive_management::activate_user,
                routes::users::inactive_management::check_user_status
            ],
        )
}
