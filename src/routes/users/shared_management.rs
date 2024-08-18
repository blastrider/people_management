use crate::db::DbConn;
use crate::models::{NewSharedAccount, SharedAccount, User};
use diesel::prelude::*;
use diesel::result::Error;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;

#[post("/shared_accounts", data = "<shared_account>")]
pub async fn add_shared_account(
    conn: DbConn,
    shared_account: Json<NewSharedAccount>,
) -> Result<Json<SharedAccount>, BadRequest<String>> {
    use crate::schema::shared_accounts::dsl::*;

    let new_shared_account = shared_account.into_inner();

    let result = conn
        .run(move |c| {
            diesel::insert_into(shared_accounts)
                .values(&new_shared_account)
                .get_result::<SharedAccount>(c)
        })
        .await;

    match result {
        Ok(account) => Ok(Json(account)),
        Err(Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => Err(
            BadRequest("This user is already associated with this account".to_string()),
        ),
        Err(e) => Err(BadRequest(format!("Error adding shared account: {}", e))),
    }
}

#[delete("/shared_accounts/<user_id>/<account_id>")]
pub async fn remove_shared_account(
    conn: DbConn,
    user_id: i32,
    account_id: i32,
) -> Result<String, BadRequest<String>> {
    use crate::schema::shared_accounts::dsl::*;

    let result = conn
        .run(move |c| {
            diesel::delete(
                shared_accounts
                    .filter(user_id.eq(user_id))
                    .filter(account_id.eq(account_id)),
            )
            .execute(c)
        })
        .await;

    match result {
        Ok(_) => Ok("Shared account removed successfully".to_string()),
        Err(Error::NotFound) => Err(BadRequest("Shared account not found".to_string())),
        Err(e) => Err(BadRequest(format!("Error removing shared account: {}", e))),
    }
}

#[get("/shared_accounts/<account_id>/users")]
pub async fn get_shared_users(
    conn: DbConn,
    account_id: i32,
) -> Result<Json<Vec<User>>, BadRequest<String>> {
    use crate::schema::{shared_accounts, users};

    let shared_users_result = conn
        .run(move |c| {
            shared_accounts::table
                .inner_join(users::table)
                .filter(shared_accounts::account_id.eq(account_id)) // Utilisation correcte de `account_id`
                .select(users::all_columns)
                .load::<User>(c)
        })
        .await;

    match shared_users_result {
        Ok(users) => Ok(Json(users)),
        Err(Error::NotFound) => Err(BadRequest("No users found for this account".to_string())),
        Err(e) => Err(BadRequest(format!("Error retrieving shared users: {}", e))),
    }
}
