<script setup lang="ts">
import { defineProps, defineEmits } from "vue";
import { invoke } from "@tauri-apps/api/core";
const props = defineProps({
  selectAll: {
    type: Boolean,
    default: false,
  },
  selectedCount: {
    type: Number,
    default: 0,
  },
});

const emit = defineEmits([
  "update:select-all",
  "toggle-select-all",
  "select-all-unmessaged",
  "select-all-uncommented",
  "open-message-modal",
  "open-comment-modal",
  "edit-common-messages",
]);

// 获取登录二维码
async function getLoginQRCode() {
  try {
    const qrcodeData = await invoke("get_login_qrcode", { size: 180 });
    console.log("二维码数据:", qrcodeData);
    // 处理二维码数据，例如显示二维码图片
    // qrcodeData 可能包含 qrid 和 image 等字段
  } catch (error) {
    console.error("获取二维码失败:", error);
  }
}

function updateSelectAll(value: boolean) {
  emit("update:select-all", value);
  emit("toggle-select-all");
}
</script>

<template>
  <div class="action-bar">
    <div class="selection-info">
      <div class="select-buttons">
        <el-button
          size="small"
          type="primary"
          @click="emit('select-all-unmessaged')"
          plain
        >
          <el-icon><message /></el-icon>
          全选未私信用户
        </el-button>
        
        <el-button
          size="small"
          type="success"
          @click="emit('select-all-uncommented')"
          plain
        >
          <el-icon><document /></el-icon>
          全选未评论用户
        </el-button>
        
        <el-checkbox
          :model-value="selectAll"
          @update:modelValue="updateSelectAll"
        >
          全选
        </el-checkbox>
      </div>
      
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

      <!-- 添加编辑常用语按钮 -->
      <el-button type="info" @click="emit('edit-common-messages')">
        <el-icon><edit /></el-icon> 编辑常用语
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

.select-buttons {
  display: flex;
  align-items: center;
  gap: 10px;
}

.action-buttons {
  display: flex;
  gap: 10px;
  align-items: center;
}
</style>
