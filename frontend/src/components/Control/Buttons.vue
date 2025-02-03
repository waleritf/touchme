<script setup lang="ts">
import DirectionalButton from './DirectionalButton.vue';
const emits = defineEmits<{
  (e: 'ws-send', message: object): void
}>();

const buildMessage = (button: string) => {
  return { input: 'Key', action: { type: 'Click', button } }
}
</script>

<template>
  <div class="buttons-panel">
    <DirectionalButton direction="left" @click="emits('ws-send', buildMessage('Left'))"/>
    <div class="horizontal-buttons">
      <DirectionalButton direction="up" @click="emits('ws-send', buildMessage('Up'))"/>
      <DirectionalButton direction="down" @click="emits('ws-send', buildMessage('Down'))"/>
    </div>
    <DirectionalButton direction="right" @click="emits('ws-send', buildMessage('Right'))"/>
  </div>
</template>

<style scoped>
  .buttons-panel {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    align-items: stretch;
  }

  .horizontal-buttons {
    display: grid;
    grid-template-rows: 1fr 1fr;
  }
</style>