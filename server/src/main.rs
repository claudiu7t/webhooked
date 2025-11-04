mod db;
mod schema;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let db_connection = db::establish_connection();

    let app = Router::new().route("/", get(index)).with_state(db_connection);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}