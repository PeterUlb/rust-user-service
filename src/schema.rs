table! {
    users (id) {
        id -> Int8,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        password_version -> Int4,
        date_of_birth -> Date,
        status -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
