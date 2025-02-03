import Control from './components/Control.vue'
import Qr from './components/Qr.vue'
import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  { path: '/', component: Control },
  { path: '/qr', component: Qr },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})