<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRouter, useRoute } from 'vue-router';

const router = useRouter();
const route = useRoute();

// 侧边栏折叠状态 - 默认折叠
const isCollapsed = ref(true);

// 当前激活的菜单项
const activeIndex = computed(() => {
  return route.path;
});

// 菜单项配置
const menuItems = [
  {
    index: '/',
    title: '首页',
    icon: 'house',
    path: '/'
  },
  {
    index: '/fans-detail',
    title: '粉丝管理',
    icon: 'user',
    path: '/fans-detail'
  },
  {
    index: '/message-history',
    title: '私信历史',
    icon: 'chat-line-round',
    path: '/message-history'
  },
  {
    index: '/comment-history',
    title: '评论历史',
    icon: 'chat-dot-round',
    path: '/comment-history'
  }
];

// 处理菜单点击
function handleMenuSelect(index: string) {
  router.push(index);
}

// 切换侧边栏折叠状态
function toggleCollapse() {
  isCollapsed.value = !isCollapsed.value;
}
</script>

<template>
  <el-container class="app-container">
    <!-- 侧边栏 -->
    <el-aside :width="isCollapsed ? '64px' : '200px'" class="sidebar">
      <div class="sidebar-header">
        <div class="logo" v-if="!isCollapsed">
          <el-icon class="logo-icon"><user /></el-icon>
          <span class="logo-text">微博管理</span>
        </div>
        <el-icon class="logo-icon" v-else><user /></el-icon>
        
        <el-button 
          class="collapse-btn" 
          @click="toggleCollapse"
          :icon="isCollapsed ? 'expand' : 'fold'"
          circle
          size="small"
        />
      </div>
      
      <el-menu
        :default-active="activeIndex"
        :collapse="isCollapsed"
        :collapse-transition="false"
        class="sidebar-menu"
        @select="handleMenuSelect"
      >
        <el-menu-item 
          v-for="item in menuItems" 
          :key="item.index"
          :index="item.index"
          class="menu-item"
        >
          <el-icon><component :is="item.icon" /></el-icon>
          <template #title>{{ item.title }}</template>
        </el-menu-item>
      </el-menu>
      
      <!-- 底部信息 -->
      <div class="sidebar-footer" v-if="!isCollapsed">
        <div class="version-info">
          <el-text size="small" type="info">v0.1.0</el-text>
        </div>
      </div>
    </el-aside>
    
    <!-- 主内容区域 -->
    <el-main class="main-content">
      <router-view />
    </el-main>
  </el-container>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  color: #0f0f0f;
  background-color: #f6f6f6;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body {
  margin: 0;
  padding: 0;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

button:focus,
button:focus-visible {
  outline: 4px auto -webkit-focus-ring-color;
}

/* 应用容器样式 */
.app-container {
  height: 100vh;
  overflow: hidden;
}

/* 侧边栏样式 */
.sidebar {
  background: #ffffff;
  transition: width 0.3s ease;
  display: flex;
  flex-direction: column;
  box-shadow: 2px 0 12px rgba(0, 0, 0, 0.15);
  border-right: 1px solid #e5e7eb;
}

.sidebar-header {
  padding: 20px 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid #e5e7eb;
  min-height: 60px;
}

.logo {
  display: flex;
  align-items: center;
  gap: 12px;
  color: #374151;
}

.logo-icon {
  font-size: 24px;
  color: #6366f1;
}

.logo-text {
  font-size: 18px;
  font-weight: 600;
  color: #374151;
}

.collapse-btn {
  background: #f3f4f6;
  border: 1px solid #d1d5db;
  color: #6b7280;
}

.collapse-btn:hover {
  background: #e5e7eb;
  border-color: #9ca3af;
}

/* 菜单样式 */
.sidebar-menu {
  flex: 1;
  border: none;
  background: transparent;
  padding: 10px 0;
}

.sidebar-menu .el-menu-item {
  color: #6b7280;
  margin: 4px 12px;
  border-radius: 8px;
  transition: all 0.3s ease;
  border: 1px solid transparent;
}

.sidebar-menu .el-menu-item:hover {
  background: #f3f4f6;
  color: #374151;
  border-color: #e5e7eb;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.sidebar-menu .el-menu-item.is-active {
  background: #eff6ff;
  color: #1d4ed8;
  font-weight: 600;
  border-color: #bfdbfe;
  box-shadow: 0 1px 3px rgba(59, 130, 246, 0.15);
}

.sidebar-menu .el-menu-item .el-icon {
  color: inherit;
}

/* 侧边栏底部 */
.sidebar-footer {
  padding: 20px 16px;
  border-top: 1px solid #e5e7eb;
}

.version-info {
  text-align: center;
  color: #9ca3af;
}

/* 主内容区域 */
.main-content {
  background-color: #f5f7fa;
  padding: 0;
  overflow-y: auto;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .sidebar {
    position: fixed;
    z-index: 1000;
    height: 100vh;
  }
  
  .main-content {
    margin-left: 0;
  }
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  
  button:active {
    background-color: #0f0f0f69;
  }
  
  .main-content {
    background-color: #1a1a1a;
  }
  
  /* 深色模式下的侧边栏样式 */
  .sidebar {
    background: #1f2937;
    border-right: 1px solid #374151;
    box-shadow: 2px 0 12px rgba(0, 0, 0, 0.3);
  }
  
  .sidebar-header {
    border-bottom: 1px solid #374151;
  }
  
  .logo {
    color: #f9fafb;
  }
  
  .logo-text {
    color: #f9fafb;
  }
  
  .collapse-btn {
    background: #374151;
    border: 1px solid #4b5563;
    color: #d1d5db;
  }
  
  .collapse-btn:hover {
    background: #4b5563;
    border-color: #6b7280;
  }
  
  .sidebar-menu .el-menu-item {
    color: #d1d5db;
  }
  
  .sidebar-menu .el-menu-item:hover {
    background: #374151;
    color: #f9fafb;
    border-color: #4b5563;
  }
  
  .sidebar-menu .el-menu-item.is-active {
    background: #1e3a8a;
    color: #93c5fd;
    border-color: #3b82f6;
  }
  
  .sidebar-footer {
    border-top: 1px solid #374151;
  }
  
  .version-info {
    color: #6b7280;
  }
}
</style>