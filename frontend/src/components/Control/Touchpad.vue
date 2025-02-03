<script setup lang="ts">
import { reactive } from 'vue';

type CoordinatePair = [number, number];
type Touch = {
  startPosition: CoordinatePair,
  finishPosition: CoordinatePair,
  visible: boolean,
  pressed: boolean,
  released: boolean,
  moved: boolean,
  startedAt?: Date,
}

const TAP_THRESHOLD = 1;
const touch: Touch = reactive({
  startPosition: [0, 0],
  finishPosition: [0, 0],
  visible: false,
  pressed: false,
  released: true,
  moved: false,
});

const emits = defineEmits<{
  ( e: 'ws-send', message: object ): void,
}>();

const onTouchStart = (event: TouchEvent): void => {
  const touch_point = event.touches[0];
  const now = new Date;

  if (
    touch.startedAt &&
    (now.getTime() - touch.startedAt.getTime()) < 300
  ) {
    console.log('Pressed');
    touch.pressed = true;
    emits('ws-send', { input: 'Mouse', action: { type: 'Press' } });
  }

  touch.startedAt = now;
  touch.startPosition = [touch_point.clientX, touch_point.clientY];
  touch.moved = false;
  touch.released = false
  touch.visible = true;
}

const onTouchMove = (event: TouchEvent): void => {
  const touch_point = event.touches[0];

  if (
    Math.abs(touch_point.clientX - touch.startPosition[0]) > TAP_THRESHOLD ||
    Math.abs(touch_point.clientY - touch.startPosition[1]) > TAP_THRESHOLD
  ) {
    emits('ws-send', {
      input: 'Mouse',
      action: {
        type: 'Move',
        x: touch_point.clientX - touch.finishPosition[0],
        y: touch_point.clientY - touch.finishPosition[1],
      }
    });

    touch.moved = true;
  }

  touch.finishPosition = [touch_point.clientX, touch_point.clientY];
}

const onTouchEnd = (_event: TouchEvent): void => {
  if (!touch.moved) {
    emits('ws-send', { input: 'Mouse', action: { type: 'Click' } });
  } else if (touch.pressed) {
    touch.pressed = false;
    emits('ws-send', { input: 'Mouse', action: { type: 'Release' } });
  }

  touch.released = true;
  touch.visible = false;
}
</script>

<template>
  <div
    id="touchpad"
    @touchstart.prevent="onTouchStart"
    @touchmove.prevent="onTouchMove"
    @touchend.prevent="onTouchEnd"
  >
    <div
      v-if="touch.visible"
      class="feedback-circle"
      :style="{
        left: touch.finishPosition[0] + 'px',
        top: touch.finishPosition[1] + 'px',
      }"
    />
  </div>
</template>

<style scoped>
#touchpad {
  background: #141414;
  overflow: hidden;
  background-image: 
    radial-gradient(circle, rgb(232, 232, 232) 1px, transparent 1px),
    radial-gradient(circle, rgba(237, 237, 237, 0.879) 1px, transparent 1px);
  background-size: 3px 3px;
  background-position: 0 0, 1.5px 1.5px;
  background-blend-mode: soft-light;
}

.feedback-circle {
  position: absolute;
  width: 90px;
  height: 90px;
  background: rgba(0, 0, 0, 0.5);
  border-radius: 50%;
  transform: translate(-50%, -50%);
  pointer-events: none;
}
</style>