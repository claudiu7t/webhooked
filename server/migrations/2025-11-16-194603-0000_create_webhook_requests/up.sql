CREATE TABLE webhook_requests (
  id SERIAL PRIMARY KEY,
  tunnel_id INTEGER NOT NULL REFERENCES tunnels(id),
  method TEXT NOT NULL,
  path TEXT NOT NULL,
  headers JSONB NOT NULL,
  remote_port SMALLINT NOT NULL,
  remote_ip INET NOT NULL,
  body BYTEA,
  body_length INTEGER,
  arrived_at TIMESTAMPTZ DEFAULT NOW()
)