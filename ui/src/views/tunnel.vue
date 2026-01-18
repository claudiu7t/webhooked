<template>
  <div class="tunnel-view">
    <h1>Tunnel {{ tunnelId }}</h1>
    <h2>Events</h2>
    <div v-for="(event, index) in events" :key="index">
      {{ event }}
      <br />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import { useSSE } from "../sse/sse";

const route = useRoute();
const tunnelId = computed(() => route.params.id);

const { events, isConnected, error, connect } = useSSE(
  tunnelId.value as string,
);

onMounted(() => {
  connect();
});
</script>

<style scoped></style>
