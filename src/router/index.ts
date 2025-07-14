import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import FansDetail from '../views/FansDetail.vue'
import MessageHistory from '../views/MessageHistory.vue'
import CommentHistory from '../views/CommentHistory.vue'

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
    },
    {
      path: '/message-history',
      name: 'message-history',
      component: MessageHistory
    },
    {
      path: '/comment-history',
      name: 'comment-history',
      component: CommentHistory
    }
  ]
})

export default router