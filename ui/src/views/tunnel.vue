<template>
  <div class="tunnel-view">
    <h1>Tunnel {{ tunnelId }}</h1>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const tunnelId = computed(() => route.params.id);
</script>

<style scoped></style>
