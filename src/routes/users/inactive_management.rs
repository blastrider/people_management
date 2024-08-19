use crate::db::DbConn;
use crate::models::User;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::response::status::BadRequest;

#[put("/users/<user_id>/deactivate")]
pub async fn deactivate_user(conn: DbConn, user_id: i32) -> Result<String, BadRequest<String>> {
    use crate::schema::users::dsl::{active, users};

    let result = conn
        .run(move |c| {
            diesel::update(users.find(user_id))
                .set(active.eq(false))
                .execute(c)
        })
        .await;

    match result {
        Ok(_) => Ok("User deactivated successfully".to_string()),
        Err(Error::NotFound) => Err(BadRequest("User not found".to_string())),
        Err(e) => Err(BadRequest(format!("Error deactivating user: {}", e))),
    }
}

#[put("/users/<user_id>/activate")]
pub async fn activate_user(conn: DbConn, user_id: i32) -> Result<String, BadRequest<String>> {
    use crate::schema::users::dsl::{active, users};

    let result = conn
        .run(move |c| {
            diesel::update(users.find(user_id))
                .set(active.eq(true))
                .execute(c)
        })
        .await;

    match result {
        Ok(_) => Ok("User activated successfully".to_string()),
        Err(Error::NotFound) => Err(BadRequest("User not found".to_string())),
        Err(e) => Err(BadRequest(format!("Error activating user: {}", e))),
    }
}

#[get("/users/<user_id>/status")]
pub async fn check_user_status(conn: DbConn, user_id: i32) -> Result<String, BadRequest<String>> {
    use crate::schema::users::dsl::{active, users};

    let user_status = conn
        .run(move |c| users.find(user_id).select(active).first::<Option<bool>>(c))
        .await;

    match user_status {
        Ok(status) => {
            // TODO: Make this Errors messages really cathed
            if Some(status).is_some() {
                Ok("User is active".to_string())
            } else {
                Ok("User is inactive".to_string())
            }
        }
        Err(Error::NotFound) => Err(BadRequest("User not found".to_string())),
        Err(e) => Err(BadRequest(format!("Error checking user status: {}", e))),
    }
}
