table! {
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

