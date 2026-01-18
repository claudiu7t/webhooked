use crate::db;
use crate::sse;
use tokio;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: db::DbPool,
    pub webhook_tx: tokio::sync::broadcast::Sender<sse::WebhookEventData>,
}
