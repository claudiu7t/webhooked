<script setup lang="ts">
import LandingPageInfo from '../components/landing/LandingPageInfo.vue'
import LandingPageTitle from '../components/landing/LandingPageTitle.vue'
</script>

<template>
  <main>
    <LandingPageTitle class="title"></LandingPageTitle>
    <LandingPageInfo class="info"></LandingPageInfo>
    <button>Get yours now!</button>
  </main>
</template>

<style scoped>
main{
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
}
.title, .info{
  width: 100vw;
  height: 100vh;
}
button{
  position: absolute;
  border: 2px solid white;
  background: radial-gradient(var(--base), black);
  color: white;
  border: 0px;
  padding: 50px;
  z-index: 2;
  font-size: 30px;
  border-radius: 50px;
  border: 4px solid white;
  cursor: pointer;
  box-shadow: 0px 0px 0px 0px var(--base);
  
}
button:hover{
  transform: scale(1.5);
  border-radius: 110px;
  box-shadow: 0px 0px 1000px 10px var(--base);
}
</style>
