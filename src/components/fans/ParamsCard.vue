<script setup lang="ts">
import { defineProps, defineEmits } from "vue";

const props = defineProps({
  uid: {
    type: String,
    required: true
  },
  cookie: {
    type: String,
    required: true
  },
  collapsed: {
    type: Boolean,
    default: true
  },
  loading: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits([
  'update:uid',
  'update:cookie',
  'update:collapsed',
  'refresh',
  'toggle'
]);

function updateUid(value: string) {
  emit('update:uid', value);
}

function updateCookie(value: string) {
  emit('update:cookie', value);
}

function toggleCollapse() {
  emit('toggle');
  emit('update:collapsed', !props.collapsed);
}

function refresh() {
  emit('refresh');
}
</script>

<template>
  <el-card class="params-card" :body-style="{ padding: '0' }">
    <template #header>
      <div class="params-header" @click="toggleCollapse">
        <el-icon><setting /></el-icon>
        <span>查询设置</span>
        <el-icon
          class="collapse-icon"
          :class="{ 'is-collapsed': collapsed }"
        >
          <arrow-down />
        </el-icon>
      </div>
    </template>

    <div class="params-body" v-show="!collapsed">
      <el-row :gutter="20">
        <el-col :span="12">
          <el-form-item label="博主UID">
            <el-input
              :model-value="uid"
              @update:modelValue="updateUid"
              placeholder="请输入博主UID"
              prefix-icon="User"
            />
          </el-form-item>
        </el-col>

        <el-col :span="12">
          <el-form-item label="Cookie">
            <el-input
              :model-value="cookie"
              @update:modelValue="updateCookie"
              placeholder="请输入Cookie"
              prefix-icon="Key"
              type="password"
              show-password
            />
          </el-form-item>
        </el-col>
      </el-row>

      <el-button
        type="primary"
        @click="refresh"
        :loading="loading"
        class="refresh-btn"
      >
        <el-icon><refresh /></el-icon>
        刷新数据
      </el-button>
    </div>
  </el-card>
</template>

<style scoped>
/* 参数设置卡片样式 */
.params-card {
  margin-bottom: 20px;
  border-radius: 8px;
  overflow: hidden;
  transition: all 0.3s ease;
  border: 1px solid #ebeef5;
}

.params-card .el-card__header {
  padding: 0;
}

.params-header {
  background: linear-gradient(90deg, #ff9800, #ff5722);
  color: white;
  padding: 14px 20px;
  font-weight: 600;
  font-size: 16px;
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.collapse-icon {
  margin-left: auto;
  transition: transform 0.3s;
}

.collapse-icon.is-collapsed {
  transform: rotate(-180deg);
}

.params-body {
  padding: 20px;
}

.refresh-btn {
  margin-top: 10px;
  width: 120px;
  float: right;
  background-color: #ff9800;
  border-color: #ff9800;
}
</style>