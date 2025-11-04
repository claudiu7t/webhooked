// @generated automatically by Diesel CLI.

diesel::table! {
    tunnels (id) {
        id -> Int4,
        url -> Varchar,
        create_at -> Nullable<Timestamp>,
    }
}
