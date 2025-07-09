<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ElLoading, ElMessage, ElMessageBox } from "element-plus";
import { WeiboUser } from "../../views/FansDetail.vue";
import CommonMessageList from "./CommonMessageList.vue";

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
  }
});

const emit = defineEmits(['update:visible', 'remove-target']);

// 评论内容
const commentContent = ref<string>("感谢关注，我会持续更新优质内容！");

// 批量评论状态
const commentLoading = ref<boolean>(false);
const commentProgress = ref<number>(0);
const commentTotal = ref<number>(0);
const commentSuccess = ref<number>(0);
const commentFailed = ref<number>(0);
const showCommentProgress = ref<boolean>(false);
// 添加失败记录数组
const failedRecords = ref<{user: string, reason: string}[]>([]);

// 常用评论内容
const commonComments = [
  { id: 1, content: "感谢关注，我会持续更新优质内容！" },
  { id: 2, content: "新内容已更新，欢迎查看！" },
  { id: 3, content: "谢谢支持，我会继续努力！" },
];

// 使用常用评论内容
function useCommonComment(content: string): void {
  commentContent.value = content;
}

// 发送评论
async function sendComment(): Promise<void> {
  if (!commentContent.value.trim()) {
    ElMessage.warning("请输入评论内容");
    return;
  }

  // 初始化进度
  commentLoading.value = true;
  commentProgress.value = 0;
  commentTotal.value = props.targets.length;
  commentSuccess.value = 0;
  commentFailed.value = 0;
  showCommentProgress.value = true;

  // 创建加载状态
  const loading = ElLoading.service({
    lock: true,
    text: `正在批量评论 (0/${props.targets.length})`,
    background: "rgba(0, 0, 0, 0.7)",
  });

  // 依次处理每个用户
  for (let i = 0; i < props.targets.length; i++) {
    const user = props.targets[i];
    loading.setText(`正在批量评论 (${i + 1}/${props.targets.length})`);

    try {
      await invoke("fetch_user_timeline", {
        cookie: props.cookie,
        uid: user.id.toString(),
        commentText: commentContent.value,
      });

      commentSuccess.value++;
      console.log(`成功评论用户 ${user.screen_name} 的微博`);
    } catch (error) {
      commentFailed.value++;
      
      // 获取错误信息
      let errorMessage = "未知错误";
      if (typeof error === "object" && error !== null && "error" in error) {
        errorMessage = error.error;
      }
      
      console.error(`评论用户 ${user.screen_name} 的微博失败: ${errorMessage}`);
      
      // 记录失败信息
      failedRecords.value.push({
        user: user.screen_name,
        reason: errorMessage
      });
    }

    commentProgress.value = Math.floor(
      ((i + 1) / props.targets.length) * 100
    );

    // 添加延迟，避免频繁请求
    if (i < props.targets.length - 1) {
      await new Promise((resolve) => setTimeout(resolve, 200));
    }
  }

  // 完成后关闭加载状态
  loading.close();
  commentLoading.value = false;
  emit('update:visible', false);

  // 显示结果
  let resultMessage = `批量评论完成\n成功: ${commentSuccess.value}\n失败: ${commentFailed.value}`;
  
  // 如果有失败记录，添加详细信息
  if (failedRecords.value.length > 0) {
    resultMessage += "\n\n失败详情:";
    failedRecords.value.forEach(record => {
      resultMessage += `\n用户 ${record.user}: ${record.reason}`;
    });
  }
  
  ElMessageBox.alert(
    resultMessage,
    "操作结果",
    {
      confirmButtonText: "确定",
      type: commentFailed.value > 0 ? "warning" : "success",
    }
  );
  
  // 重置状态
  commentContent.value = "感谢关注，我会持续更新优质内容！";
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
    title="批量评论"
    width="600px"
    destroy-on-close
    @update:model-value="emit('update:visible', $event)"
  >
    <el-form>
      <el-form-item label="评论对象:">
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

      <el-form-item label="常用评论:">
        <CommonMessageList 
          title="评论" 
          :initialMessages="commonComments"
          @use-message="useCommonComment"
        />
      </el-form-item>

      <el-form-item label="评论内容:">
        <el-input
          v-model="commentContent"
          type="textarea"
          :rows="5"
          placeholder="请输入评论内容..."
        />
      </el-form-item>
    </el-form>

    <template #footer>
      <span class="dialog-footer">
        <el-button @click="emit('update:visible', false)">取消</el-button>
        <el-button type="primary" @click="sendComment" color="#1DA1F2"
          >发送</el-button
        >
      </span>
    </template>
  </el-dialog>
</template>

<style scoped>
/* 评论模态框样式 */
.recipient-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 10px;
}
</style>