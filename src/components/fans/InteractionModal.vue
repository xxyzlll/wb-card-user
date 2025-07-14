<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ElLoading, ElMessage, ElMessageBox } from "element-plus";
import { WeiboUser } from "../../views/FansDetail.vue";
import { LocalStorage } from "../../utils/storage";

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
  },
  // 新增类型属性，用于区分私信和评论
  type: {
    type: String,
    default: "message", // "message" 或 "comment"
    validator: (value: string) => ["message", "comment"].includes(value)
  }
});

const emit = defineEmits(['update:visible', 'remove-target', 'interaction-success']);

// 内容输入框
const content = ref<string>("感谢关注，我会持续更新优质内容！");

// 批量操作状态
const batchLoading = ref<boolean>(false);
const progress = ref<number>(0);
const total = ref<number>(0);
const success = ref<number>(0);
const failed = ref<number>(0);
const showProgress = ref<boolean>(false);
// 失败记录数组
const failedRecords = ref<{user: string, reason: string}[]>([]);

// 计算属性：标题
const modalTitle = computed(() => {
  return props.type === "message" ? "批量私信" : "批量评论";
});

// 计算属性：标签
const contentLabel = computed(() => {
  return props.type === "message" ? "私信内容:" : "评论内容:";
});

// 计算属性：常用语标签
const commonLabel = computed(() => {
  return props.type === "message" ? "常用私信:" : "常用评论:";
});

// 计算属性：按钮颜色
const buttonColor = computed(() => {
  return props.type === "message" ? "#ff9800" : "#1DA1F2";
});

// 使用常用内容
function useCommonContent(selectedContent: string): void {
  if (selectedContent) {
    content.value = selectedContent;
  }
}

// 发送操作（私信或评论）
async function send(): Promise<void> {
  if (!content.value.trim()) {
    ElMessage.warning(props.type === "message" ? "请输入私信内容" : "请输入评论内容");
    return;
  }

  // 初始化进度
  batchLoading.value = true;
  progress.value = 0;
  total.value = props.targets.length;
  success.value = 0;
  failed.value = 0;
  showProgress.value = true;
  failedRecords.value = [];
  
  const successfulUsers: WeiboUser[] = [];
  const failedUsers: WeiboUser[] = [];

  // 创建加载状态
  const loading = ElLoading.service({
    lock: true,
    text: `正在批量${props.type === "message" ? "私信" : "评论"} (0/${props.targets.length})`,
    background: "rgba(0, 0, 0, 0.7)",
  });

  // 依次处理每个用户
  for (let i = 0; i < props.targets.length; i++) {
    const user = props.targets[i];
    loading.setText(`正在批量${props.type === "message" ? "私信" : "评论"} (${i + 1}/${props.targets.length})`);

    try {
      if (props.type === "message") {
        // 发送私信
        await invoke("send_direct_message", {
          cookie: props.cookie,
          uid: user.id.toString(),
          text: content.value,
          source: "209678993",
        });
        console.log(`成功发送私信给用户 ${user.screen_name}`);
      } else {
        // 发送评论
        await invoke("fetch_user_timeline", {
          cookie: props.cookie,
          uid: user.id.toString(),
          commentText: content.value,
        });
        console.log(`成功评论用户 ${user.screen_name} 的微博`);
      }

      success.value++;
      successfulUsers.push(user);
    } catch (error: any) {
      failed.value++;
      failedUsers.push(user);
      
      let errorMessage = error;
      console.error(`${props.type === "message" ? "私信" : "评论"}用户 ${user.screen_name} 失败: ${errorMessage}`);
      
      // 如果是评论失败，记录失败状态
      if (props.type === "comment") {
        LocalStorage.addCommentFailedUser(user.id);
      }
      
      failedRecords.value.push({
        user: user.screen_name,
        reason: errorMessage
      });
    }

    progress.value = Math.floor(
      ((i + 1) / props.targets.length) * 100
    );

    if (i < props.targets.length - 1) {
      await new Promise((resolve) => setTimeout(resolve, 200));
    }
  }

  // 完成后关闭加载状态
  loading.close();
  batchLoading.value = false;
  emit('update:visible', false);
  
  // 触发成功回调，传递成功的用户列表和内容
  if (successfulUsers.length > 0) {
    emit('interaction-success', successfulUsers, content.value);
  }
  
  // 触发失败回调，传递失败的用户列表
  if (failedUsers.length > 0 && props.type === "comment") {
    emit('interaction-failed', failedUsers);
  }

  // 显示结果
  let resultMessage = `批量${props.type === "message" ? "私信" : "评论"}完成<br>成功: ${success.value}<br>失败: ${failed.value}`;
  
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
      type: failed.value > 0 ? "warning" : "success",
      dangerouslyUseHTMLString: true, // 允许渲染HTML内容
    }
  );
  
  // 重置状态
  content.value = "感谢关注，我会持续更新优质内容！";
  failedRecords.value = [];
}

// 移除目标用户
function removeTarget(userId: string | number): void {
  emit('remove-target', userId);
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    :title="modalTitle"
    width="600px"
    destroy-on-close
    @update:model-value="emit('update:visible', $event)"
  >
    <el-form>
      <el-form-item label="操作对象:">
        <div class="recipient-list">
          <el-tag
            v-for="user in targets"
            :key="user.id"
            closable
            @close="removeTarget(user.id)"
            type="info"
            effect="dark"
          >
            {{ user.screen_name }}
          </el-tag>
        </div>
      </el-form-item>

      <el-form-item :label="commonLabel">
        <el-select 
          :placeholder="`选择${props.type === 'message' ? '常用私信' : '常用评论'}`" 
          style="width: 100%"
          @change="useCommonContent"
        >
          <el-option 
            v-for="message in commonMessages" 
            :key="message.id" 
            :label="message.content" 
            :value="message.content"
          />
        </el-select>
      </el-form-item>

      <el-form-item :label="contentLabel">
        <el-input
          v-model="content"
          type="textarea"
          :rows="5"
          :placeholder="`请输入${props.type === 'message' ? '私信' : '评论'}内容...`"
        />
      </el-form-item>
    </el-form>

    <template #footer>
      <span class="dialog-footer">
        <el-button @click="emit('update:visible', false)">取消</el-button>
        <el-button type="primary" @click="send" :color="buttonColor"
          >发送</el-button
        >
      </span>
    </template>
  </el-dialog>
</template>

<style scoped>
/* 模态框样式 */
.recipient-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 10px;
}
</style>