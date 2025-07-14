<template>
  <main class="container">
    <el-container>
      <el-header height="auto" class="fixed-header">
        <div class="header">
          <el-button class="back-btn" @click="goBack" text>
            <el-icon><arrow-left /></el-icon> 返回粉丝列表
          </el-button>
          <h2 class="page-title">
            <el-icon><chat-dot-round /></el-icon>
            评论历史
          </h2>
          <el-button 
            type="danger" 
            size="small" 
            @click="clearHistory"
            :disabled="historyList.length === 0"
          >
            <el-icon><delete /></el-icon>
            清空历史
          </el-button>
        </div>
        
        <!-- 统计信息 -->
        <el-card class="stats-card">
          <div class="stats-info">
            <el-statistic title="总评论数" :value="historyList.length" />
            <el-statistic title="今日评论" :value="todayCount" />
            <el-statistic title="本周评论" :value="weekCount" />
          </div>
        </el-card>
      </el-header>

      <el-main class="scrollable-content">
        <template v-if="historyList.length > 0">
          <el-card 
            v-for="record in paginatedHistory" 
            :key="`${record.userId}-${record.timestamp}`"
            class="history-card" 
            shadow="hover"
          >
            <div class="history-content">
              <el-avatar
                :size="50"
                :src="record.userAvatar"
                class="avatar"
              />
              
              <div class="record-info">
                <div class="record-header">
                  <h3 class="username">{{ record.userName }}</h3>
                  <span class="timestamp">{{ formatTime(record.timestamp) }}</span>
                </div>
                
                <div class="comment-content">
                  <el-tag type="success" size="small">评论内容</el-tag>
                  <p>{{ record.content }}</p>
                </div>
              </div>
              
              <div class="actions">
                <el-button 
                  type="primary" 
                  size="small" 
                  @click="openUserPage(record.userId)"
                >
                  <el-icon><user /></el-icon>
                  查看主页
                </el-button>
                <el-button 
                  type="danger" 
                  size="small" 
                  @click="removeRecord(record.userId)"
                >
                  <el-icon><delete /></el-icon>
                  删除
                </el-button>
              </div>
            </div>
          </el-card>
        </template>
        
        <el-empty v-else description="暂无评论历史记录">
          <el-button type="primary" @click="goBack">去发送评论</el-button>
        </el-empty>
      </el-main>

      <el-footer class="fixed-footer" v-if="historyList.length > 0">
        <el-pagination
          background
          layout="prev, pager, next, total"
          :current-page="currentPage"
          :page-size="pageSize"
          :total="historyList.length"
          @current-change="handlePageChange"
        />
      </el-footer>
    </el-container>
  </main>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { ElMessage, ElMessageBox } from 'element-plus';
import { LocalStorage, UserHistoryRecord } from '../utils/storage';

const router = useRouter();
const historyList = ref<UserHistoryRecord[]>([]);
const currentPage = ref(1);
const pageSize = ref(20);

// 计算今日评论数
const todayCount = computed(() => {
  const today = new Date();
  today.setHours(0, 0, 0, 0);
  return historyList.value.filter(record => record.timestamp >= today.getTime()).length;
});

// 计算本周评论数
const weekCount = computed(() => {
  const weekAgo = new Date();
  weekAgo.setDate(weekAgo.getDate() - 7);
  weekAgo.setHours(0, 0, 0, 0);
  return historyList.value.filter(record => record.timestamp >= weekAgo.getTime()).length;
});

// 分页数据
const paginatedHistory = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  const end = start + pageSize.value;
  return historyList.value.slice(start, end);
});

// 加载历史记录
function loadHistory(): void {
  historyList.value = LocalStorage.getCommentHistory();
}

// 格式化时间
function formatTime(timestamp: number): string {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - timestamp;
  
  if (diff < 60000) { // 1分钟内
    return '刚刚';
  } else if (diff < 3600000) { // 1小时内
    return `${Math.floor(diff / 60000)}分钟前`;
  } else if (diff < 86400000) { // 24小时内
    return `${Math.floor(diff / 3600000)}小时前`;
  } else {
    return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
  }
}

// 打开用户主页
function openUserPage(userId: string | number): void {
  const userLink = `https://weibo.com/u/${userId}`;
  window.open(userLink, '_blank');
}

// 删除单条记录
function removeRecord(userId: string | number): void {
  ElMessageBox.confirm(
    '确定要删除这条评论记录吗？',
    '确认删除',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    }
  ).then(() => {
    LocalStorage.removeCommentRecord(userId);
    loadHistory();
    ElMessage.success('删除成功');
  }).catch(() => {
    // 用户取消
  });
}

// 清空历史
function clearHistory(): void {
  ElMessageBox.confirm(
    '确定要清空所有评论历史记录吗？此操作不可恢复！',
    '确认清空',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    }
  ).then(() => {
    LocalStorage.clearCommentHistory();
    loadHistory();
    ElMessage.success('清空成功');
  }).catch(() => {
    // 用户取消
  });
}

// 分页处理
function handlePageChange(page: number): void {
  currentPage.value = page;
}

// 返回粉丝列表
function goBack(): void {
  router.push('/fans-detail');
}

// 组件挂载时加载数据
onMounted(() => {
  loadHistory();
});
</script>

<style scoped>
.container {
  max-width: 1200px;
  margin: 0 auto;
  height: 100vh;
  overflow: hidden;
}

.el-container {
  height: 100%;
}

.fixed-header {
  padding: 20px;
  background-color: #fff;
  border-bottom: 1px solid #ebeef5;
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
}

.back-btn {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 16px;
  color: #666;
}

.page-title {
  margin: 0;
  display: flex;
  align-items: center;
  gap: 8px;
  color: #333;
}

.stats-card {
  margin-bottom: 0;
}

.stats-info {
  display: flex;
  justify-content: space-around;
}

.scrollable-content {
  overflow-y: auto;
  padding: 20px;
  background-color: #f5f7fa;
}

.history-card {
  margin-bottom: 15px;
  border-radius: 8px;
}

.history-content {
  display: flex;
  align-items: flex-start;
  gap: 15px;
}

.avatar {
  flex-shrink: 0;
  border: 2px solid #f0f2f5;
}

.record-info {
  flex: 1;
}

.record-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 10px;
}

.username {
  margin: 0;
  font-size: 16px;
  color: #333;
}

.timestamp {
  color: #999;
  font-size: 12px;
}

.comment-content {
  background-color: #f8f9fa;
  padding: 10px;
  border-radius: 6px;
  border-left: 3px solid #1DA1F2;
}

.comment-content p {
  margin: 5px 0 0 0;
  color: #666;
  line-height: 1.5;
}

.actions {
  display: flex;
  flex-direction: column;
  gap: 8px;
  flex-shrink: 0;
}

.fixed-footer {
  display: flex;
  justify-content: center;
  padding: 20px;
  background-color: #fff;
  border-top: 1px solid #ebeef5;
}
</style>