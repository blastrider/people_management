#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod db;
mod models;
mod schema;
mod routes;

use rocket::fairing::AdHoc;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::DbConn::fairing())
        .attach(AdHoc::on_ignite("Database Migrations", db::run_migrations))
        .mount("/api", routes![
            routes::users::create::create_user,
            routes::users::update::update_user,
            routes::users::delete::delete_user
        ])
}

