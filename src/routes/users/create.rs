use crate::db::DbConn;
use crate::models::{NewUser, User};
use diesel::prelude::*;
use diesel::result::Error;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;

#[post("/users", data = "<new_user>")]
pub async fn create_user(
    conn: DbConn,
    new_user: Json<NewUser>,
) -> Result<Json<User>, BadRequest<String>> {
    use crate::schema::users::dsl::*;

    let mut new_user = new_user.into_inner();
    let ssn_clone = new_user.ssn.clone(); // Clonez la valeur de ssn

    // Vérifier si un utilisateur avec le même SSN existe déjà
    let existing_user = conn
        .run(move |c| users.filter(ssn.eq(ssn_clone)).first::<User>(c))
        .await;

    match existing_user {
        Ok(_) => Err(BadRequest(
            "User with the same SSN already exists".to_string(),
        )),
        Err(Error::NotFound) => {
            // Créer l'utilisateur car il n'existe pas déjà
            let created_user = conn
                .run(move |c| diesel::insert_into(users).values(&new_user).get_result(c))
                .await;

            match created_user {
                Ok(user) => Ok(Json(user)),
                Err(e) => Err(BadRequest(format!("Error creating user: {}", e))),
            }
        }
        Err(e) => Err(BadRequest(format!("Database error: {}", e))),
    }
}
