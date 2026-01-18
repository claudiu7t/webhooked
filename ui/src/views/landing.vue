<script setup lang="ts">
import { useRouter } from "vue-router";
import LandingPageInfo from "../components/landing/LandingPageInfo.vue";
import LandingPageTitle from "../components/landing/LandingPageTitle.vue";

const router = useRouter();

const getTunnel = () => {
  fetch("http://manage.localhost/api/tunnels/", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
  })
    .then((response) => response.json())
    .then((data) => {
      let tunnel_name = data.tunnel_name;
      router.push(`/tunnel/${tunnel_name}`);
    })
    .catch((error) => {
      console.error(error);
      alert("Error creating tunnel, please try again.");
    });
};
</script>

<template>
  <main>
    <LandingPageTitle class="title"></LandingPageTitle>
    <LandingPageInfo class="info"></LandingPageInfo>
    <button @click="getTunnel">Get yours now!</button>
  </main>
</template>

<style scoped>
main {
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
}
.title,
.info {
  width: 100vw;
  height: 100vh;
}
button {
  position: absolute;
  border: 2px solid white;
  background: var(--black);
  color: white;
  border: 0px;
  padding: 50px;
  z-index: 2;
  font-size: 30px;
  border-radius: 50px;
  border: 4px solid white;
  cursor: pointer;
}
button:hover {
  background: white;
  color: var(--base);
}
</style>
