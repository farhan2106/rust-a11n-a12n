table! {
    password_updates (id) {
        id -> Integer,
        user_id -> Integer,
        token -> Varchar,
        date_created -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        salt -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        enabled -> Bool,
        date_created -> Timestamp,
        date_update -> Timestamp,
    }
}

joinable!(password_updates -> users (user_id));

allow_tables_to_appear_in_same_query!(
    password_updates,
    users,
);
