<script setup lang="ts">
import Touchpad from './Control/Touchpad.vue';
import Buttons from './Control/Buttons.vue';
import { Ws } from '@/utils/ws';

const SERVER_URL = `http://${window.location.hostname}:7127/ws`;
const ws: Ws = new Ws(SERVER_URL);

</script>

<template>
  <div class="control">
    <Touchpad @ws-send="ws.send"/>
    <Buttons @ws-send="ws.send"/>
  </div>
</template>

<style scoped>
  .control {
    min-width: 100%;
    min-height: 100%;
    display: grid;
  }

  @media(orientation: portrait) {
    .control {
      grid-template-rows: 4fr 1fr;
      grid-template-columns: 1fr;
    }
  }

  @media(orientation: landscape) {
    .control {
      grid-template-rows: 1fr;
      grid-template-columns: 4fr 1fr;
    }
  }
</style>
