<script setup lang="ts">
import { ref } from "vue";
import { ElMessage, ElMessageBox } from "element-plus";

// 定义常用消息内容接口
interface CommonMessage {
  id: number;
  content: string;
}

const props = defineProps({
  title: {
    type: String,
    default: "常用内容"
  },
  initialMessages: {
    type: Array as () => CommonMessage[],
    default: () => []
  }
});

const emit = defineEmits(['use-message']);

// 常用消息内容管理
const commonMessages = ref<CommonMessage[]>(props.initialMessages);
const newMessageContent = ref<string>("");
const editingMessageId = ref<number | null>(null);
const editingContent = ref<string>("");

// 使用常用消息内容
function useMessage(content: string): void {
  emit('use-message', content);
}

// 添加常用消息内容
function addCommonMessage(): void {
  if (!newMessageContent.value.trim()) {
    ElMessage.warning("请输入常用内容");
    return;
  }

  const newId =
    commonMessages.value.length > 0
      ? Math.max(...commonMessages.value.map((msg) => msg.id)) + 1
      : 1;

  commonMessages.value.push({
    id: newId,
    content: newMessageContent.value,
  });

  newMessageContent.value = "";
  ElMessage.success("添加成功");
}

// 删除常用消息内容
function deleteCommonMessage(id: number): void {
  ElMessageBox.confirm("确定要删除这条常用内容吗？", "提示", {
    confirmButtonText: "确定",
    cancelButtonText: "取消",
    type: "warning",
  })
    .then(() => {
      commonMessages.value = commonMessages.value.filter(
        (msg) => msg.id !== id
      );
      ElMessage.success("删除成功");
    })
    .catch(() => {
      // 取消删除
    });
}

// 开始编辑常用消息内容
function startEditMessage(message: CommonMessage): void {
  editingMessageId.value = message.id;
  editingContent.value = message.content;
}

// 保存编辑后的常用消息内容
function saveEditMessage(): void {
  if (!editingContent.value.trim()) {
    ElMessage.warning("内容不能为空");
    return;
  }

  const index = commonMessages.value.findIndex(
    (msg) => msg.id === editingMessageId.value
  );
  if (index !== -1) {
    commonMessages.value[index].content = editingContent.value;
    ElMessage.success("修改成功");
  }

  editingMessageId.value = null;
  editingContent.value = "";
}

// 取消编辑
function cancelEditMessage(): void {
  editingMessageId.value = null;
  editingContent.value = "";
}
</script>

<template>
  <div class="common-messages">
    <div
      v-for="message in commonMessages"
      :key="message.id"
      class="common-message-item"
    >
      <template v-if="editingMessageId === message.id">
        <el-input
          v-model="editingContent"
          type="textarea"
          :rows="2"
          class="edit-message-input"
        />
        <div class="edit-actions">
          <el-button
            type="primary"
            size="small"
            @click="saveEditMessage"
            color="#ff9800"
            >保存</el-button
          >
          <el-button size="small" @click="cancelEditMessage"
            >取消</el-button
          >
        </div>
      </template>

      <template v-else>
        <div
          class="message-content"
          @click="useMessage(message.content)"
        >
          {{ message.content }}
        </div>
        <div class="message-actions">
          <el-button
            type="primary"
            size="small"
            circle
            @click="startEditMessage(message)"
            color="#ff9800"
          >
            <el-icon><edit /></el-icon>
          </el-button>
          <el-button
            type="danger"
            size="small"
            circle
            @click="deleteCommonMessage(message.id)"
          >
            <el-icon><delete /></el-icon>
          </el-button>
        </div>
      </template>
    </div>

    <el-divider content-position="center">添加常用{{ title }}</el-divider>

    <div class="add-common-message">
      <el-input
        v-model="newMessageContent"
        type="textarea"
        :rows="2"
        :placeholder="`输入新的常用${title}内容...`"
      />
      <el-button
        type="primary"
        @click="addCommonMessage"
        color="#ff9800"
      >
        <el-icon><plus /></el-icon> 添加
      </el-button>
    </div>
  </div>
</template>

<style scoped>
/* 常用消息内容样式 */
.common-messages {
  margin-bottom: 20px;
}

.common-message-item {
  display: flex;
  margin-bottom: 10px;
  padding: 10px;
  background-color: #f9f9f9;
  border-radius: 6px;
  border-left: 3px solid #ff9800;
}

.message-content {
  flex: 1;
  padding: 5px 10px;
  cursor: pointer;
  transition: background-color 0.2s;
  border-radius: 4px;
}

.message-content:hover {
  background-color: #fff3e0;
}

.message-actions {
  display: flex;
  gap: 5px;
}

.edit-message-input {
  margin-bottom: 10px;
  width: 100%;
}

.edit-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.add-common-message {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.add-common-message .el-button {
  align-self: flex-end;
}
</style>