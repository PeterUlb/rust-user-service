table! {
    sessions (id) {
        id -> Uuid,
        user_id -> Int8,
        platform -> Varchar,
        sub_platform -> Varchar,
        refreshed_at -> Timestamptz,
        expires_at -> Timestamptz,
        status -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

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

allow_tables_to_appear_in_same_query!(sessions, users,);
