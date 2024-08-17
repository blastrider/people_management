use rocket::response::status::{BadRequest, NoContent};
use crate::db::DbConn;
use diesel::prelude::*;

#[delete("/users/<user_id>")]
pub async fn delete_user(conn: DbConn, user_id: i32) -> Result<NoContent, BadRequest<String>> {
    use crate::schema::users::dsl::*;

    conn.run(move |c| {
        diesel::delete(users.find(user_id))
            .execute(c)
            .map(|_| NoContent)
            .map_err(|e| BadRequest(Some(format!("Error: {}", e)).unwrap_or_default()))
    }).await
}

