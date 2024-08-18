// @generated automatically by Diesel CLI.

diesel::table! {
    shared_accounts (id) {
        id -> Int4,
        user_id -> Int4,
        account_id -> Int4,
        role -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        password_hash -> Varchar,
        address -> Nullable<Varchar>,
        phone_number -> Nullable<Varchar>,
        ssn -> Nullable<Varchar>,
    }
}

diesel::joinable!(shared_accounts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(shared_accounts, users,);
