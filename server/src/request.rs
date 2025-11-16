use crate::db;
use crate::schema::tunnels::dsl::*;
use axum::{
    body::Bytes,
    extract::ConnectInfo,
    extract::State,
    http,
    http::{Method, StatusCode, Uri},
    response::IntoResponse,
};
use diesel::prelude::*;
use serde_json::Value;
use std::net::SocketAddr;

#[derive(Insertable, Debug)]
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
    State(db_conn): State<db::DbPool>,
    method: Method,
    uri: Uri,
    headers: http::HeaderMap,
    ConnectInfo(remote_addr): ConnectInfo<SocketAddr>,
    body: Bytes,
) -> impl IntoResponse {
    let mut conn = db_conn.get().expect("Couldn't get db connection");

    let method = method.as_str().to_owned();
    let path_and_query = uri.to_string();
    let remote_ip_value = headers
        .get("x-real-ip")
        .expect("x-real-ip header is missing, should have been added by nginx");
    let remote_ip_str = remote_ip_value.to_str().expect("x-real-ip to_str() failed");
    let remote_ip = remote_ip_str.to_owned();
    let remote_port = remote_addr.port();

    let host = headers.get("host").expect("host header is missing");
    let host_str = host.to_str().expect("host header to_str() failed");
    let host = host_str.to_owned();

    let subdomain = match host.split(".").next() {
        Some(s) => s.to_owned(),
        None => {
            return (StatusCode::BAD_REQUEST, "400 Bad Request: No such tunnel.").into_response();
        }
    };

    let subdomain = subdomain.to_lowercase();

    println!("!{}!", subdomain);
    let tunnel_id = tunnels
        .filter(tunnel_name.eq(subdomain))
        .select(id)
        .first::<i32>(&mut conn)
        .optional()
        .expect("Db query for looking for tunnel id failed");

    let tunnel_id: i32 = match tunnel_id {
        Some(t_id) => t_id,
        None => {
            return (
                StatusCode::NOT_FOUND,
                "404 Not Found: Tunnel ID could not be found for the given subdomain.",
            )
                .into_response();
        }
    } as i32;

    let webhook_request = NewWebhookRequest {
        tunnel_id: tunnel_id,
        method: method,
        path: path_and_query,
        headers: diesel_json::Json(headers),
        remote_port: remote_port as i16,
        remote_ip: remote_ip,
        body: Some(body.to_vec()),
        body_length: Some(body.len() as i32),
    };

    println!("{:?}", webhook_request);

    (StatusCode::OK, "Success.").into_response()
}
