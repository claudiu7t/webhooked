import { createRouter, createWebHistory } from "vue-router";
import Landing from "./views/landing.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [{ path: "/", component: Landing }],
});
