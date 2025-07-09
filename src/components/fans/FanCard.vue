<script setup lang="ts">
import { defineProps, defineEmits } from "vue";
import { WeiboUser } from "../../views/FansDetail.vue";

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

      <div class="user-info">
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
</style>