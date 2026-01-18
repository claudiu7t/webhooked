use crate::app_state;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use futures::StreamExt;
use serde;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Debug, serde::Serialize, Clone)]
pub struct WebhookEventData {
    pub tunnel_name: String,
    pub method: String,
    pub path: String,
    pub headers: serde_json::Value,
    pub remote_port: i32,
    pub remote_ip: String,
    pub body: Option<String>,
    pub body_length: Option<i32>,
    pub arrived_at: chrono::DateTime<chrono::Utc>,
}

pub async fn sse_handler(
    State(app_state): State<app_state::AppState>,
    Path(tunnel_name): Path<String>,
) -> impl IntoResponse {
    let rx = app_state.webhook_tx.subscribe();
    let stream = BroadcastStream::new(rx);

    let filtered_stream = stream.filter_map(move |result| {
        let event = match result {
            Ok(event) => event,
            // ignore errors
            Err(_) => return futures::future::ready(None),
        };

        // only send events for the given tunnel
        if event.tunnel_name == tunnel_name {
            return futures::future::ready(Some(event));
        } else {
            return futures::future::ready(None);
        }
    });

    let sse_stream = filtered_stream.map(|event| {
        let json = serde_json::to_string(&event).unwrap_or_default();
        Ok::<_, std::convert::Infallible>(axum::response::sse::Event::default().data(json))
    });

    axum::response::Sse::new(sse_stream)
}
