<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()

const navigateToFansDetail = () => {
  router.push('/fans-detail')
}
</script>

<template>
  <main class="container">
    <div class="card-container">
      <el-card class="feature-card" shadow="hover" @click="navigateToFansDetail">
        <div class="icon-container">
          <el-icon class="icon"><user /></el-icon>
        </div>
        <h2>粉丝列表</h2>
        <p>查看并管理您的微博粉丝，支持批量私信功能</p>
      </el-card>
      
      <!-- 预留其他功能卡片位置 -->
      <el-card class="feature-card disabled" shadow="hover">
        <div class="icon-container">
          <el-icon class="icon"><data-analysis /></el-icon>
        </div>
        <h2>数据分析</h2>
        <p>分析粉丝互动数据和增长趋势（即将上线）</p>
      </el-card>
      
      <el-card class="feature-card disabled" shadow="hover">
        <div class="icon-container">
          <el-icon class="icon"><chat-dot-round /></el-icon>
        </div>
        <h2>消息中心</h2>
        <p>管理所有私信和评论互动（即将上线）</p>
      </el-card>
    </div>
    
    <div class="footer">
      <p>微博粉丝管理工具 v0.1.0</p>
    </div>
  </main>
</template>

<style scoped>
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 40px 20px;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.card-container {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 30px;
  margin-bottom: 50px;
}

.feature-card {
  height: 100%;
  transition: all 0.3s ease;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 20px;
}

.feature-card:hover:not(.disabled) {
  transform: translateY(-10px);
  box-shadow: 0 15px 30px rgba(0, 0, 0, 0.1);
}

.disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.icon-container {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background-color: #fff3e0;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 20px;
  transition: all 0.3s ease;
}

.feature-card:hover:not(.disabled) .icon-container {
  background-color: #ffcc80;
}

.icon {
  font-size: 40px;
  color: #ff9800;
}

h2 {
  margin: 0 0 10px 0;
  color: #333;
  font-size: 1.5rem;
}

p {
  color: #666;
  margin: 0;
  line-height: 1.5;
}

.footer {
  margin-top: auto;
  text-align: center;
  color: #888;
  font-size: 0.9rem;
}

@media (max-width: 768px) {
  .card-container {
    grid-template-columns: 1fr;
  }
  
  h1 {
    font-size: 2rem;
  }
}
</style>