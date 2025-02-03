<script setup lang="ts">
import QRCode from 'qrcode';
import { ref } from 'vue';

const host = ref(`http://${window.location.host}`);
let qr = ref('');

QRCode.toDataURL(host.value)
  .then(url => qr.value = url)
  .catch(err => console.error(err))
</script>

<template>
  <div id="qrcode">
    <img :src="qr" alt="QR code to controls">
  </div>
</template>

<style scoped>
#qrcode {
  height: 100%;
  display: grid;
  justify-items: center;
  justify-content: center;
  align-items: center;
  align-content: center;
}
</style>