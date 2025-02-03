<script setup lang="ts">
type Direction = 'up' | 'down' | 'left' | 'right';

defineProps<{
  direction: Direction
}>();

const emits = defineEmits<{
  (e: 'click'): void;
}>();;
</script>

<template>
  <button
    :class="['direction-btn', direction]"
    @click="emits('click')"
    :aria-label="`Move ${direction}`"
  >
    <span class="arrow"></span>
  </button>
</template>

<style scoped>
.direction-btn {
  cursor: pointer;
  background-color: #272727;
  display: grid;
  align-items: center;
  justify-items: center;
  border: 0;
  outline: solid 1px #575757;
}

.arrow {
  width: 0;
  height: 0;
  border-style: solid;
}

.up .arrow {
  border-width: 0 20px 40px 20px;
  border-color: transparent transparent #979797 transparent;
}

.down .arrow {
  border-width: 40px 20px 0 20px;
  border-color: #979797 transparent transparent transparent;
}

.left .arrow {
  border-width: 20px 40px 20px 0;
  border-color: transparent #979797 transparent transparent;
}

.right .arrow {
  border-width: 20px 0 20px 40px;
  border-color: transparent transparent transparent #979797;
}
</style>