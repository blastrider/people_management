#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod db;
mod models;
mod routes;
mod schema;

use rocket::fairing::AdHoc;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::DbConn::fairing())
        .attach(AdHoc::on_ignite("Database Migrations", db::run_migrations))
        .mount("/api", routes![routes::users::create_user])
}
