import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import FansDetail from '../views/FansDetail.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/fans-detail',
      name: 'fans-detail',
      component: FansDetail
    }
  ]
})

export default router