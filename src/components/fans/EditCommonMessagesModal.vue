<script setup lang="ts">
import { ref } from "vue";
import CommonMessageList from "./CommonMessageList.vue";

interface CommonMessage {
  id: number;
  content: string;
}

const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  },
  messages: {
    type: Array as () => CommonMessage[],
    default: () => []
  },
  title: {
    type: String,
    default: "私信"
  }
});

const emit = defineEmits(['update:visible', 'update:messages']);

// 本地编辑的消息列表
const editedMessages = ref<CommonMessage[]>([...props.messages]);

// 保存编辑后的常用消息
function saveMessages(): void {
  emit('update:messages', editedMessages.value);
  emit('update:visible', false);
}

// 取消编辑
function cancelEdit(): void {
  emit('update:visible', false);
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    :title="`编辑常用${title}`"
    width="600px"
    destroy-on-close
    @update:model-value="emit('update:visible', $event)"
  >
    <CommonMessageList 
      :title="title" 
      :initialMessages="editedMessages"
      :readonly="false"
    />
    
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="cancelEdit">取消</el-button>
        <el-button type="primary" @click="saveMessages" color="#ff9800"
          >保存</el-button
        >
      </span>
    </template>
  </el-dialog>
</template>