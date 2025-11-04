<script setup lang="ts">
import { useIntervalFn } from '@vueuse/core'
import { nanoid } from 'nanoid'
import { ref } from 'vue'

const webhooked_url = ref(nanoid(8));
const { pause, resume, isActive } = useIntervalFn(() => {
  webhooked_url.value = nanoid(8);
}, 1000);

</script>

<template>
<header>
  <div class="name">
  <span class="dynamic-url">{{ webhooked_url }}.</span>
  <span class="name-part1">Webhook</span><span class="name-part2">ed</span>
  <span class="com">.com</span>
  </div>
</header>
</template>

<style scoped>
.name{
  font-size: 40px;
}
.dynamic-url{
  color: var(--base);
}
header{
  background: #B0B0B0;
  display: flex;
  justify-content: end;
  align-items: center;
  box-shadow: 0px 0px 1000px 0px #2C365E;
  z-index: 1;
}
.name{
  background: #9f9f9f;
  width: 50vw;
  display: flex;
  justify-content: right;
  align-items: center;
  right: 150px;
  padding-right: 20px;
}
.name-part1{
  font-size: 60px;
}
.name-part2, .com{
  top: 20px;
}
.name>span{
  color: var(--base);
  font-weight: bold;
}
.dynamic-url{
  min-width: 200px;
  display: inline-flex;
  justify-content: right;
  top: -20px;
  font-family: monospace;
}
.static-url{
  font-style: italic;
}
</style>
