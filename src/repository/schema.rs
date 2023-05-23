// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Varchar,
        title -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        age -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
