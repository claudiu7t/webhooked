mod db;
mod schema;

use axum::{
    routing::get,
    routing::post,
    Router,
    Json,
    extract::State,
};

use nanoid::nanoid;
use serde::{Serialize};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[derive(Serialize)]
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
async fn make_tunnel(State(db_conn): State<db::DbPool>) -> Json<Tunnel> {
    let uuid = nanoid!(8);
    let mut conn = db_conn.get().expect("Couldn't get db connection");

    let new_tunnel = NewTunnel {tunnel_name: uuid};
    let result = diesel::insert_into(crate::schema::tunnels::table).values(&new_tunnel).returning(Tunnel::as_returning()).get_result(&mut conn).expect("Error creating new tunnel");
    return Json(result);

}

#[tokio::main]
async fn main() {
    let db_connection = db::establish_connection();

    let app = Router::new().route("/api/tunnels/", post(make_tunnel)).with_state(db_connection);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}