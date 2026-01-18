mod app_state;
mod db;
mod request;
mod schema;
mod sse;

use axum::{Json, Router, extract::State, routing::post};

use diesel::prelude::*;
use nanoid::nanoid;
use serde::Serialize;

use std::net::SocketAddr;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::tunnels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Tunnel {
    id: i32,
    tunnel_name: String,
    created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tunnels)]
struct NewTunnel {
    tunnel_name: String,
}

#[axum::debug_handler]
async fn make_tunnel(State(app_state): State<app_state::AppState>) -> Json<Tunnel> {
    let uuid = nanoid!(8).to_lowercase();
    let mut conn = app_state.db_pool.get().expect("Couldn't get db connection");

    let new_tunnel = NewTunnel { tunnel_name: uuid };
    let result = diesel::insert_into(crate::schema::tunnels::table)
        .values(&new_tunnel)
        .returning(Tunnel::as_returning())
        .get_result(&mut conn)
        .expect("Error creating new tunnel");
    return Json(result);
}

#[tokio::main]
async fn main() {
    let db_connection = db::establish_connection();
    let (webhook_tx, _webhook_rx) = tokio::sync::broadcast::channel::<sse::WebhookEventData>(100);

    let app_state = app_state::AppState {
        db_pool: db_connection,
        webhook_tx: webhook_tx,
    };

    let app = Router::new()
        .route("/api/tunnels/", post(make_tunnel))
        .fallback(request::request_handler)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // make_service_with_connect_info allows us to extract the source port later
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
