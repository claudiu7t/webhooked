use serde;
#[derive(Debug, serde::Serialize, Clone)]
pub struct WebhookEventData {
    pub tunnel_id: i32,
    pub method: String,
    pub path: String,
    pub headers: serde_json::Value,
    pub remote_port: i32,
    pub remote_ip: String,
    pub body: Option<String>,
    pub body_length: Option<i32>,
    pub arrived_at: chrono::DateTime<chrono::Utc>,
}
