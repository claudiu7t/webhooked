use axum::{
    body::Bytes,
    extract::ConnectInfo,
    http,
    http::{Method, StatusCode, Uri},
    response::IntoResponse,
};
use diesel::prelude::*;
use serde_json::Value;
use std::net::SocketAddr;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::webhook_requests)]
struct NewWebhookRequest {
    tunnel_id: i32,
    method: String,
    path: String,
    headers: diesel_json::Json<http::HeaderMap>,
    remote_port: i16,
    remote_ip: String,
    body: Option<Vec<u8>>,
    body_length: Option<i32>,
}

pub async fn request_handler(
    method: Method,
    uri: Uri,
    headers: http::HeaderMap,
    ConnectInfo(remote_addr): ConnectInfo<SocketAddr>,
    body: Bytes,
) -> impl IntoResponse {
    let method = method.as_str();
    let path_and_query = uri.to_string();
    println!("Method: {}", method);
    println!("URI: {}", path_and_query);
    println!("Headers: {:?}", headers);

    let body_string = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(_) => "[Binary or Invalid UTF-8 Body] test",
    };

    println!("Body length: {}", body.len());
    println!("Body Content: {}", body_string);

    let remote_port = remote_addr.port();
    println!("Source port: {}", remote_port);

    (
        StatusCode::BAD_REQUEST,
        "400 Bad Request: Unhandled endpoint or method.",
    )
        .into_response()
}
