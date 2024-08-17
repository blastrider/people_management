use rocket::serde::json::Json;
use rocket::response::status;
use crate::db::DbConn;
use crate::models::{NewUser, User};
use diesel::prelude::*;

#[post("/users", data = "<new_user>")]
pub async fn create_user(conn: DbConn, new_user: Json<NewUser>) -> Result<Json<User>, status::BadRequest<String>> {
    use crate::schema::users::dsl::*;

    let new_user = new_user.into_inner();
    conn.run(move |c| {
        diesel::insert_into(users)
            .values(&new_user)
            .get_result(c)
            .map(Json)
            .map_err(|e| status::BadRequest(Some(format!("Error: {}", e))))
    }).await
}

