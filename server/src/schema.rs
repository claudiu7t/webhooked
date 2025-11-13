// @generated automatically by Diesel CLI.

diesel::table! {
    tunnels (id) {
        id -> Int4,
        tunnel_name -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}
