import { ref, type Ref, onUnmounted } from "vue";

export interface WebhookEvent {
  tunnel_name: string;
  method: string;
  path: string;
  headers: [string, string][];
  remote_port: number;
  remote_ip: string;
  body: string | null;
  body_length: number | null;
  arrived_at: string;
}

export function useSSE(tunnelName: string) {
  const events: Ref<WebhookEvent[]> = ref([]);
  const isConnected: Ref<boolean> = ref(false);
  const error: Ref<string | null> = ref(null);
  let eventSource: EventSource | null = null;

  const connect = () => {
    const url = "http://manage.localhost/api/tunnels/${tunnelName}/sse";
    eventSource = new EventSource(url);

    eventSource.onopen = () => {
      console.log("Connected to SSE");
      isConnected.value = true;
      error.value = null;
    };

    eventSource.onerror = () => {
      console.error("Error connecting to SSE");
      isConnected.value = false;
      error.value = "Failed to connect to SSE";
      eventSource?.close();
    };

    eventSource.onmessage = (event) => {
      const webhookEvent: WebhookEvent = JSON.parse(event.data);
      events.value.push(webhookEvent);
    };
  };

  onUnmounted(() => {
    eventSource?.close();
  });

  return {
    events,
    isConnected,
    error,
  };
}
