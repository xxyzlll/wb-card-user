<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ElLoading, ElMessage, ElMessageBox } from "element-plus";
import { WeiboUser } from "../../views/FansDetail.vue";
// 移除 CommonMessageList 导入
// import CommonMessageList from "./CommonMessageList.vue";

interface CommonMessage {
  id: number;
  content: string;
}

const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  },
  targets: {
    type: Array as () => WeiboUser[],
    required: true
  },
  cookie: {
    type: String,
    required: true
  },
  commonMessages: {
    type: Array as () => CommonMessage[],
    default: () => []
  }
});

const emit = defineEmits(['update:visible', 'remove-target']);

// 私信内容
const messageContent = ref<string>("");

// 批量私信状态
const messageLoading = ref<boolean>(false);
const messageProgress = ref<number>(0);
const messageTotal = ref<number>(0);
const messageSuccess = ref<number>(0);
const messageFailed = ref<number>(0);
const showMessageProgress = ref<boolean>(false);
// 添加失败记录数组
const failedRecords = ref<{user: string, reason: string}[]>([]);

// 使用常用私信内容
function useCommonMessage(content: string): void {
  messageContent.value = content;
}

// 选择常用私信
function handleSelectMessage(content: string): void {
  if (content) {
    messageContent.value = content;
  }
}

// 发送私信
async function sendMessage(): Promise<void> {
  if (!messageContent.value.trim()) {
    ElMessage.warning("请输入私信内容");
    return;
  }

  // 批量发送私信
  await batchSendMessage();
}

// 批量发送私信函数
async function batchSendMessage(): Promise<void> {
  if (!messageContent.value.trim()) {
    ElMessage.warning("请输入私信内容");
    return;
  }

  // 初始化进度
  messageLoading.value = true;
  messageProgress.value = 0;
  messageTotal.value = props.targets.length;
  messageSuccess.value = 0;
  messageFailed.value = 0;
  showMessageProgress.value = true;

  // 创建加载状态
  const loading = ElLoading.service({
    lock: true,
    text: `正在批量发送私信 (0/${props.targets.length})`,
    background: "rgba(0, 0, 0, 0.7)",
  });

  // 依次处理每个用户
  for (let i = 0; i < props.targets.length; i++) {
    const user = props.targets[i];
    loading.setText(
      `正在批量发送私信 (${i + 1}/${props.targets.length})`
    );

    try {
      await invoke("send_direct_message", {
        cookie: props.cookie,
        text: messageContent.value,
        uid: user.id.toString(),
        source: "209678993", // 应用ID
      });
  
      messageSuccess.value++;
      console.log(`成功发送私信给用户 ${user.screen_name}`);
    } catch (error) {
      messageFailed.value++;
      // 记录失败信息
      failedRecords.value.push({
        user: user.screen_name,
        reason: error as string
      });
    }

    messageProgress.value = Math.floor(
      ((i + 1) / props.targets.length) * 100
    );

    // 添加延迟，避免频繁请求
    if (i < props.targets.length - 1) {
      await new Promise((resolve) => setTimeout(resolve, 200));
    }
  }

  // 完成后关闭加载状态
  loading.close();
  messageLoading.value = false;
  emit('update:visible', false);

  // 显示结果
  let resultMessage = `批量发送私信完成<br>成功: ${messageSuccess.value}<br>失败: ${messageFailed.value}`;
  
  // 如果有失败记录，添加详细信息
  if (failedRecords.value.length > 0) {
    resultMessage += "<br><br><strong>失败详情:</strong>";
    failedRecords.value.forEach(record => {
      resultMessage += `<br>用户 （${record.user}）: ${record.reason}`;
    });
  }
  
  ElMessageBox.alert(
    resultMessage,
    "操作结果",
    {
      confirmButtonText: "确定",
      type: messageFailed.value > 0 ? "warning" : "success",
      dangerouslyUseHTMLString: true, // 允许渲染HTML内容
    }
  );
  
  // 重置状态
  messageContent.value = "";
  failedRecords.value = []; // 清空失败记录
}

// 移除目标用户
function removeTarget(userId: string | number): void {
  emit('remove-target', userId);
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    title="发送私信"
    width="600px"
    destroy-on-close
    @update:model-value="emit('update:visible', $event)"
  >
    <el-form>
      <el-form-item label="收信人:">
        <div class="recipient-list">
          <el-tag
            v-for="user in targets"
            :key="user.id"
            closable
            @close="removeTarget(user.id)"
            type="warning"
            effect="dark"
          >
            {{ user.screen_name }}
          </el-tag>
        </div>
      </el-form-item>

      <el-form-item label="常用私信:">
        <!-- 替换为下拉选择框 -->
        <el-select 
          placeholder="选择常用私信" 
          style="width: 100%"
          @change="handleSelectMessage"
        >
          <el-option 
            v-for="message in commonMessages" 
            :key="message.id" 
            :label="message.content" 
            :value="message.content"
          />
        </el-select>
      </el-form-item>

      <el-form-item label="私信内容:">
        <el-input
          v-model="messageContent"
          type="textarea"
          :rows="5"
          placeholder="请输入私信内容..."
        />
      </el-form-item>
    </el-form>

    <template #footer>
      <span class="dialog-footer">
        <el-button @click="emit('update:visible', false)">取消</el-button>
        <el-button type="primary" @click="sendMessage" color="#ff9800"
          >发送</el-button
        >
      </span>
    </template>
  </el-dialog>
</template>

<style scoped>
/* 私信模态框样式 */
.recipient-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 10px;
}
</style>