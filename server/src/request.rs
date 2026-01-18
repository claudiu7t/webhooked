use crate::app_state;
use crate::db;
use crate::schema::webhook_requests;
use crate::sse::WebhookEventData;
use base64::prelude::*;

use axum::{
    Json,
    body::Bytes,
    extract::ConnectInfo,
    extract::Path,
    extract::State,
    http,
    http::{Method, StatusCode, Uri},
    response::IntoResponse,
};
use diesel::prelude::*;
use std::net::SocketAddr;

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::webhook_requests)]
struct NewWebhookRequest {
    tunnel_id: i32,
    method: String,
    path: String,
    headers: serde_json::Value,
    remote_port: i32,
    remote_ip: String,
    body: Option<Vec<u8>>,
    body_length: Option<i32>,
}

pub async fn request_handler(
    State(app_state): State<app_state::AppState>,
    method: Method,
    uri: Uri,
    headers: http::HeaderMap,
    ConnectInfo(remote_addr): ConnectInfo<SocketAddr>,
    body: Bytes,
) -> impl IntoResponse {
    let mut conn = app_state.db_pool.get().expect("Couldn't get db connection");

    use crate::schema::tunnels::dsl::*;

    let method = method.as_str().to_owned();
    let path_and_query = uri.to_string();
    let remote_ip_value = headers
        .get("x-real-ip")
        .expect("x-real-ip header is missing, should have been added by nginx");
    let remote_ip_str = remote_ip_value.to_str().expect("x-real-ip to_str() failed");
    let remote_ip = remote_ip_str.to_owned();
    let remote_port = remote_addr.port() as i32;

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

    let tunnel_id = tunnels
        .filter(tunnel_name.eq(&subdomain))
        .select(crate::schema::tunnels::id)
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

    // convert the HeaderMap to a vec of tuples to prevent unintelligible recursion limit error ¯\_(ツ)_/¯
    let header_vec: Vec<(String, String)> = headers
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                v.to_str().unwrap_or("[BINARY DATA]").to_string(),
            )
        })
        .collect();

    let header_json = serde_json::to_value(header_vec).unwrap();
    let body_length = Some(body.len() as i32);

    let webhook_request = NewWebhookRequest {
        tunnel_id: tunnel_id,
        method: method.clone(),
        path: path_and_query.clone(),
        headers: header_json.clone(),
        remote_port: remote_port,
        remote_ip: remote_ip.clone(),
        body: Some(body.to_vec()),
        body_length: body_length,
    };

    diesel::insert_into(webhook_requests::table)
        .values(&webhook_request)
        .execute(&mut conn)
        .expect("Error saving webhook request");

    let body_string = if body.is_empty() {
        None
    } else {
        Some(BASE64_STANDARD.encode(&body))
    };

    let webhook_event = WebhookEventData {
        tunnel_name: subdomain,
        method: method,
        path: path_and_query,
        headers: header_json,
        remote_port: remote_port,
        remote_ip: remote_ip,
        body: body_string,
        body_length: body_length,
        arrived_at: chrono::Utc::now(),
    };

    // ignore that nobody might be watching
    let _ = app_state.webhook_tx.send(webhook_event);

    (StatusCode::OK, "Success.").into_response()
}

#[derive(serde::Serialize, diesel::Queryable, diesel::Selectable)]
#[diesel(table_name = crate::schema::webhook_requests)]
struct WebhookRequest {
    id: i32,
    tunnel_id: i32,
    method: String,
    path: String,
    headers: serde_json::Value,
    remote_port: i32,
    remote_ip: String,
    body: Option<Vec<u8>>,
    body_length: Option<i32>,
    arrived_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_all_requests(
    Path(request_tunnel_id): Path<i32>,
    State(db_conn): State<db::DbPool>,
) -> impl IntoResponse {
    let mut conn = db_conn.get().expect("Couldn't get db connection");

    use crate::schema::webhook_requests::dsl::*;

    let results = webhook_requests
        .filter(tunnel_id.eq(request_tunnel_id))
        .order(crate::schema::webhook_requests::id.desc())
        .load::<WebhookRequest>(&mut conn)
        .expect("Failed to load requests");

    Json(results)
}
