use rocket::serde::json::Json;
use rocket::response::status::BadRequest;
use crate::db::DbConn;
use crate::models::{User, NewUser};
use diesel::prelude::*;

#[put("/users/<user_id>", data = "<updated_user>")]
pub async fn update_user(conn: DbConn, user_id: i32, updated_user: Json<NewUser>) -> Result<Json<User>, BadRequest<String>> {
    use crate::schema::users::dsl::*;

    let updated_user = updated_user.into_inner();
    conn.run(move |c| {
        diesel::update(users.find(user_id))
            .set(&updated_user)
            .get_result(c)
            .map(Json)
            .map_err(|e| BadRequest(Some(format!("Error: {}", e)).unwrap_or_default()))
    }).await
}
