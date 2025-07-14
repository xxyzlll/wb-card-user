<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()

const navigateToFansDetail = () => {
  router.push('/fans-detail')
}

const navigateToMessageHistory = () => {
  router.push('/message-history')
}

const navigateToCommentHistory = () => {
  router.push('/comment-history')
}
</script>

<template>
  <div class="home-container">
    <!-- 页面标题 -->
    <div class="page-header">
      <h1 class="page-title">
        <el-icon class="title-icon"><house /></el-icon>
        欢迎使用微博粉丝管理工具
      </h1>
      <p class="page-subtitle">高效管理您的微博粉丝，提升社交媒体运营效率</p>
    </div>
    
    <!-- 功能卡片网格 -->
    <div class="card-container">
      <el-card class="feature-card primary" shadow="hover" @click="navigateToFansDetail">
        <div class="card-content">
          <div class="icon-container primary">
            <el-icon class="icon"><user /></el-icon>
          </div>
          <h3 class="card-title">粉丝管理</h3>
          <p class="card-description">查看粉丝列表，支持性别筛选和批量操作</p>
          <div class="card-features">
            <el-tag size="small" type="info">批量私信</el-tag>
            <el-tag size="small" type="success">批量评论</el-tag>
            <el-tag size="small" type="warning">性别筛选</el-tag>
          </div>
        </div>
      </el-card>
      
      <el-card class="feature-card warning" shadow="hover" @click="navigateToMessageHistory">
        <div class="card-content">
          <div class="icon-container warning">
            <el-icon class="icon"><chat-line-round /></el-icon>
          </div>
          <h3 class="card-title">私信历史</h3>
          <p class="card-description">查看所有私信记录，管理私信历史</p>
          <div class="card-features">
            <el-tag size="small" type="warning">历史记录</el-tag>
            <el-tag size="small" type="info">统计分析</el-tag>
          </div>
        </div>
      </el-card>
      
      <el-card class="feature-card success" shadow="hover" @click="navigateToCommentHistory">
        <div class="card-content">
          <div class="icon-container success">
            <el-icon class="icon"><chat-dot-round /></el-icon>
          </div>
          <h3 class="card-title">评论历史</h3>
          <p class="card-description">查看所有评论记录，管理评论历史</p>
          <div class="card-features">
            <el-tag size="small" type="success">历史记录</el-tag>
            <el-tag size="small" type="info">统计分析</el-tag>
          </div>
        </div>
      </el-card>
      
      <!-- 预留功能卡片 -->
      <el-card class="feature-card disabled" shadow="hover">
        <div class="card-content">
          <div class="icon-container disabled">
            <el-icon class="icon"><data-analysis /></el-icon>
          </div>
          <h3 class="card-title">数据分析</h3>
          <p class="card-description">粉丝增长趋势和互动数据分析</p>
          <div class="card-features">
            <el-tag size="small" type="info">即将上线</el-tag>
          </div>
        </div>
      </el-card>
    </div>
    
    <!-- 快速操作区域 -->
    <div class="quick-actions">
      <h3 class="section-title">
        <el-icon><lightning /></el-icon>
        快速操作
      </h3>
      <div class="action-buttons">
        <el-button type="primary" size="large" @click="navigateToFansDetail">
          <el-icon><user /></el-icon>
          开始管理粉丝
        </el-button>
        <el-button type="warning" size="large" @click="navigateToMessageHistory">
          <el-icon><chat-line-round /></el-icon>
          查看私信历史
        </el-button>
        <el-button type="success" size="large" @click="navigateToCommentHistory">
          <el-icon><chat-dot-round /></el-icon>
          查看评论历史
        </el-button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.home-container {
  padding: 30px;
  max-width: 1400px;
  margin: 0 auto;
}

/* 页面标题区域 */
.page-header {
  text-align: center;
  margin-bottom: 40px;
  padding: 40px 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 16px;
  color: white;
}

.page-title {
  font-size: 2.5rem;
  margin: 0 0 10px 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
}

.title-icon {
  font-size: 2.5rem;
}

.page-subtitle {
  font-size: 1.1rem;
  margin: 0;
  opacity: 0.9;
}

/* 功能卡片网格 */
.card-container {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 24px;
  margin-bottom: 40px;
}

.feature-card {
  height: 100%;
  transition: all 0.3s ease;
  cursor: pointer;
  border-radius: 12px;
  overflow: hidden;
}

.feature-card:hover:not(.disabled) {
  transform: translateY(-8px);
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
}

.feature-card.disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.card-content {
  padding: 24px;
  text-align: center;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.icon-container {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 20px;
  transition: all 0.3s ease;
}

.icon-container.primary {
  background: linear-gradient(135deg, #667eea, #764ba2);
}

.icon-container.warning {
  background: linear-gradient(135deg, #f093fb, #f5576c);
}

.icon-container.success {
  background: linear-gradient(135deg, #4facfe, #00f2fe);
}

.icon-container.disabled {
  background: linear-gradient(135deg, #d7d2cc, #304352);
}

.icon {
  font-size: 36px;
  color: white;
}

.card-title {
  margin: 0 0 12px 0;
  color: #333;
  font-size: 1.4rem;
  font-weight: 600;
}

.card-description {
  color: #666;
  margin: 0 0 16px 0;
  line-height: 1.6;
  flex: 1;
}

.card-features {
  display: flex;
  gap: 8px;
  justify-content: center;
  flex-wrap: wrap;
}

/* 快速操作区域 */
.quick-actions {
  background: white;
  padding: 30px;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.05);
}

.section-title {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 0 0 20px 0;
  color: #333;
  font-size: 1.3rem;
}

.action-buttons {
  display: flex;
  gap: 16px;
  justify-content: center;
  flex-wrap: wrap;
}

.action-buttons .el-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  border-radius: 8px;
  font-weight: 500;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .home-container {
    padding: 20px;
  }
  
  .page-title {
    font-size: 2rem;
    flex-direction: column;
    gap: 8px;
  }
  
  .card-container {
    grid-template-columns: 1fr;
  }
  
  .action-buttons {
    flex-direction: column;
    align-items: center;
  }
  
  .action-buttons .el-button {
    width: 100%;
    max-width: 300px;
  }
}

/* 深色模式适配 */
@media (prefers-color-scheme: dark) {
  .card-title {
    color: #f0f0f0;
  }
  
  .card-description {
    color: #ccc;
  }
  
  .section-title {
    color: #f0f0f0;
  }
  
  .quick-actions {
    background: #2a2a2a;
  }
}
</style>