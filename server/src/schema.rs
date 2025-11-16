// @generated automatically by Diesel CLI.

diesel::table! {
    tunnels (id) {
        id -> Int4,
        tunnel_name -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    webhook_requests (id) {
        id -> Int4,
        tunnel_id -> Int4,
        method -> Text,
        path -> Text,
        headers -> Jsonb,
        remote_port -> Int2,
        remote_ip -> Inet,
        body -> Nullable<Bytea>,
        body_length -> Nullable<Int4>,
        arrived_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(webhook_requests -> tunnels (tunnel_id));

diesel::allow_tables_to_appear_in_same_query!(tunnels, webhook_requests,);
