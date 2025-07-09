<script setup lang="ts">
import { defineProps, defineEmits } from "vue";

const props = defineProps({
  selectAll: {
    type: Boolean,
    default: false
  },
  selectedCount: {
    type: Number,
    default: 0
  }
});

const emit = defineEmits([
  'update:select-all',
  'toggle-select-all',
  'open-message-modal',
  'open-comment-modal'
]);

function updateSelectAll(value: boolean) {
  emit('update:select-all', value);
  emit('toggle-select-all');
}
</script>

<template>
  <div class="action-bar">
    <div class="selection-info">
      <el-checkbox 
        :model-value="selectAll" 
        @update:modelValue="updateSelectAll"
      >
        全选
      </el-checkbox>
      <el-tag v-if="selectedCount > 0" type="warning" effect="dark">
        已选择 {{ selectedCount }} 位粉丝
      </el-tag>
    </div>

    <div class="action-buttons">
      <el-button
        type="primary"
        @click="emit('open-message-modal')"
        :disabled="selectedCount === 0"
        color="#ff9800"
      >
        <el-icon><message /></el-icon> 发送私信
      </el-button>

      <el-button
        type="primary"
        @click="emit('open-comment-modal')"
        :disabled="selectedCount === 0"
        color="#1DA1F2"
      >
        <el-icon><document /></el-icon> 批量评论
      </el-button>
    </div>
  </div>
</template>

<style scoped>
/* 操作栏样式 */
.action-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding: 15px;
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
}

.selection-info {
  display: flex;
  align-items: center;
  gap: 15px;
}

.action-buttons {
  display: flex;
  gap: 10px;
}
</style>