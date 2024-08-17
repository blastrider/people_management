use crate::db::DbConn;
use crate::models::{NewUser, User};
use diesel::prelude::*;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;

#[post("/users", data = "<new_user>")]
pub async fn create_user(
    conn: DbConn,
    new_user: Json<NewUser>,
) -> Result<Json<User>, BadRequest<String>> {
    use crate::schema::users::dsl::*;

    let new_user = new_user.into_inner();
    conn.run(move |c| {
        diesel::insert_into(users)
            .values(&new_user)
            .get_result(c)
            .map(Json)
            .map_err(|e| BadRequest(Some(format!("Error: {}", e)).unwrap_or_default()))
    })
    .await
}
