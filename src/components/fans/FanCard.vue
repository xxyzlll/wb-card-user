<script setup lang="ts">
import { defineProps, defineEmits } from "vue";
import { WeiboUser } from "../../views/FansDetail.vue";
import { openUrl } from "@tauri-apps/plugin-opener";
import { ElMessage } from "element-plus";

const props = defineProps({
  user: {
    type: Object as () => WeiboUser,
    required: true
  },
  selected: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['toggle-select']);

// 处理点击用户卡片事件
async function handleUserClick() {
  try {
    // 构建微博用户链接
    const userLink = `https://weibo.com/u/${props.user.id}`;
    
    // 打开网页
    await openUrl(userLink);
    
    // 显示成功消息
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
        <h3 class="username">{{ user.screen_name }}</h3>
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

.username {
  margin: 0 0 5px 0;
  font-size: 18px;
  color: #333;
}

.bio {
  margin: 0 0 10px 0;
  color: #666;
  font-size: 14px;
  line-height: 1.4;
}

.stats {
  display: flex;
  gap: 10px;
}

/* 添加可点击区域样式 */
.clickable-area {
  cursor: pointer;
  transition: background-color 0.2s;
  padding: 8px;
  border-radius: 4px;
}

.clickable-area:hover {
  background-color: rgba(0, 0, 0, 0.05);
}
</style>