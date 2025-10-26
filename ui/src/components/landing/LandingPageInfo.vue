<script setup>

</script>

<template>
<header>
<contents>
<div class="supports"><span>HTTP</span><span>Websockets</span><span>TCP</span><span>UDP</span></div>
<div class="features"><span>Receive</span><span class="sep"></span><span>Monitor</span><span class="sep"></span><span>Forward</span></div>
</contents>
</header>
</template>

<style scoped>
contents{
background: #030303;
width: 50vw;
display: flex;
justify-content: center;
flex-direction: column;
align-items: center;
padding: 10px;
}
header{
  background: black;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}
.dynamic-url{
  font-family: monospace;
}
.static-url{
  font-style: italic;
}
.supports{
  display: flex;
  justify-content: center;
  align-items: center;
}
.supports>span{
  border-left:4px solid var(--base);
  border-radius: 0px 0px 10px 0px;
  margin: 5px;
  padding-left:15px;
}
.supports>span:nth-child(1){
  border-left: 0px;
}
.sep{
  clip-path: polygon(100% 50%, 0 0, 0 100%);
  width: 10px;
  height: 10px;
  background: var(--base);
  display: inline-block;
}
.features{
  display: flex;
  justify-content: center;
  align-items: center;
}
.features > span{
  padding: 10px;
}
</style>