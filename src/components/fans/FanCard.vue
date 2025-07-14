<script setup lang="ts">
import { defineProps, defineEmits } from "vue";
import { WeiboUser } from "../../views/FansDetail.vue";
import { ElMessage } from "element-plus";

const props = defineProps({
  user: {
    type: Object as () => WeiboUser,
    required: true
  },
  selected: {
    type: Boolean,
    default: false
  },
  isMessaged: {
    type: Boolean,
    default: false
  },
  isCommented: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['toggle-select']);

// 处理点击用户卡片事件
async function handleUserClick() {
  try {
    const userLink = `https://weibo.com/u/${props.user.id}`;
    
    // 在 Electron 环境中使用 window.open
    if (window.electronAPI) {
      window.open(userLink, '_blank');
    } else {
      // 浏览器环境备用方案
      window.open(userLink, '_blank');
    }
    
    ElMessage.success({
      message: `正在打开用户主页: ${props.user.screen_name}`,
      duration: 2000
    });
  } catch (error) {
    console.error("打开网页失败:", error);
    ElMessage.error({
      message: "打开网页失败，请重试",
      duration: 2000
    });
  }
}
</script>

<template>
  <el-card class="fan-card" shadow="hover">
    <div class="fan-card-content">
      <div class="select-area">
        <el-checkbox
          :model-value="selected"
          @update:modelValue="emit('toggle-select', user.id)"
        />
      </div>

      <el-avatar
        :size="60"
        :src="user.profile_image_url"
        class="avatar"
      />

      <div class="user-info clickable-area" @click="handleUserClick">
        <div class="user-header">
          <h3 class="username">{{ user.screen_name }}</h3>
          <!-- 添加状态标签 -->
          <div class="status-tags">
            <el-tag 
              v-if="isMessaged" 
              type="warning" 
              size="small" 
              effect="dark"
              class="status-tag"
            >
              <el-icon><chat-line-round /></el-icon>
              已私信
            </el-tag>
            <el-tag 
              v-if="isCommented" 
              type="success" 
              size="small" 
              effect="dark"
              class="status-tag"
            >
              <el-icon><chat-dot-round /></el-icon>
              已评论
            </el-tag>
          </div>
        </div>
        
        <p class="bio">
          {{ user.description || "这个人很懒，什么都没写" }}
        </p>
        <div class="stats">
          <el-tag size="small" effect="plain" type="danger"
            >粉丝: {{ user.followers_count }}</el-tag
          >
          <el-tag size="small" effect="plain" type="success"
            >关注: {{ user.friends_count }}</el-tag
          >
          <el-tag size="small" effect="plain" type="info"
            >微博: {{ user.statuses_count }}</el-tag
          >
        </div>
      </div>
    </div>
  </el-card>
</template>

<style scoped>
/* 粉丝卡片样式 */
.fan-card {
  margin-bottom: 15px;
  border-radius: 8px;
  overflow: hidden;
}

.fan-card-content {
  display: flex;
  align-items: center;
}

.select-area {
  margin-right: 15px;
}

.avatar {
  margin-right: 20px;
  border: 3px solid #f0f2f5;
}

.user-info {
  flex: 1;
}

.user-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 5px;
}

.username {
  margin: 0;
  font-size: 18px;
  color: #333;
}

.status-tags {
  display: flex;
  gap: 5px;
}

.status-tag {
  display: flex;
  align-items: center;
  gap: 2px;
}
</style>