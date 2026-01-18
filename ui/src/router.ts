import { createRouter, createWebHistory } from "vue-router";
import Landing from "./views/landing.vue";
import Tunnel from "./views/tunnel.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", component: Landing },
    { path: "/tunnel/:id", component: Tunnel },
  ],
});
