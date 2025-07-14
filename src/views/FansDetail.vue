<script setup lang="ts">
import { ref, reactive, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { apiCookie } from "../contants";
import { useRouter } from "vue-router";
import { ElLoading, ElMessage, ElMessageBox } from "element-plus";
import { LocalStorage } from "../utils/storage";

// 导入拆分出的组件
import ParamsCard from "../components/fans/ParamsCard.vue";
import ActionBar from "../components/fans/ActionBar.vue";
import FanCard from "../components/fans/FanCard.vue";
import InteractionModal from "../components/fans/InteractionModal.vue";
import EditCommonMessagesModal from "../components/fans/EditCommonMessagesModal.vue";

// 定义用户类型接口
interface WeiboUser {
  id: string | number;
  screen_name: string;
  profile_image_url: string;
  description?: string;
  followers_count: number;
  friends_count: number;
  statuses_count: number;
  gender?: string;
  [key: string]: any;
}

// 定义粉丝数据接口
interface FansData {
  users: WeiboUser[];
  [key: string]: any;
}

// 导出接口供其他组件使用
export type { WeiboUser, FansData };

const router = useRouter();
function extractWeiboId(input: string) {
  // 匹配纯数字ID（如 7961825456）
  if (/^\d+$/.test(input)) return input;

  // 匹配PC端URL：https://weibo.com/u/7961825456
  const pcMatch = input.match(/https?:\/\/weibo\.com\/u\/(\d+)/i);
  if (pcMatch && pcMatch[1]) return pcMatch[1];

  // 匹配移动端URL：https://m.weibo.com/u/7961825456 或 https://m.weibo.com/p/1005057961825456
  const mwebMatch = input.match(
    /(?:https?:\/\/m\.weibo\.(?:com|cn)\/(?:u|p\/100505)\/(\d+))/i
  );
  if (mwebMatch && mwebMatch[1]) return mwebMatch[1];

  return null; // 无效输入
}

// 粉丝数据状态管理
const fansData = ref<FansData>({ users: [] });
const loading = ref<boolean>(false);
const currentPage = ref<number>(1);
const inputUId = ref<string>("https://weibo.com/u/7526709666");
const cookie = ref<string>(apiCookie);

// 添加性别筛选状态
const selectedGender = ref<string>('f');

// 添加历史状态管理
const messagedUsers = ref<Set<string | number>>(new Set());
const commentedUsers = ref<Set<string | number>>(new Set());

// 加载历史状态
function loadHistoryStatus(): void {
  messagedUsers.value = LocalStorage.getMessagedUsers();
  commentedUsers.value = LocalStorage.getCommentedUsers();
}

// 检查用户是否已私信
function isUserMessaged(userId: string | number): boolean {
  return messagedUsers.value.has(userId);
}

// 检查用户是否已评论
function isUserCommented(userId: string | number): boolean {
  return commentedUsers.value.has(userId);
}

const uid = computed(() => {
  return extractWeiboId(inputUId.value);
});

// 添加筛选后的粉丝数据计算属性
const filteredFans = computed(() => {
  if (!fansData.value.users) return [];
  
  if (selectedGender.value === 'all') {
    return fansData.value.users;
  }
  
  return fansData.value.users.filter(user => {
    // 根据微博API，性别字段可能是 'f'(女), 'm'(男), 'n'(未知)
    return user.gender === selectedGender.value;
  });
});

// 统一的常用语列表
const commonMessages = ref<{ id: number; content: string }[]>([
  { id: 1, content: "感谢关注，我会持续更新优质内容！" },
  { id: 2, content: "新内容已更新，欢迎查看！" },
  { id: 3, content: "有任何问题都可以随时联系我哦~" },
]);

// 选择状态管理
const selectedFans = reactive<Set<string | number>>(new Set());
const selectAll = ref<boolean>(false);

// 新增：全选未私信用户
function selectAllUnmessagedUsers(): void {
  const unmessagedUsers = filteredFans.value.filter(user => !isUserMessaged(user.id));
  unmessagedUsers.forEach(user => {
    selectedFans.add(user.id);
  });
  // 检查是否已全选
  if (selectedFans.size === filteredFans.value.length) {
    selectAll.value = true;
  }
}

// 新增：全选未评论用户
function selectAllUncommentedUsers(): void {
  const uncommentedUsers = filteredFans.value.filter(user => !isUserCommented(user.id));
  uncommentedUsers.forEach(user => {
    selectedFans.add(user.id);
  });
  // 检查是否已全选
  if (selectedFans.size === filteredFans.value.length) {
    selectAll.value = true;
  }
}

// 私信相关
const showMessageModal = ref<boolean>(false);
const messageTargets = ref<WeiboUser[]>([]);

// 评论相关
const showCommentModal = ref<boolean>(false);
const commentTargets = ref<WeiboUser[]>([]);

// 常用语编辑弹窗 - 只保留一个
const showEditMessagesModal = ref<boolean>(false);

// 顶部查询区域折叠状态
const paramsCardCollapsed = ref<boolean>(true);

// 打开编辑常用语弹窗 - 只保留一个函数
function openEditMessagesModal(): void {
  showEditMessagesModal.value = true;
}

// 加载粉丝数据
async function loadFans(): Promise<void> {
  try {
    loading.value = true;
    const data = await invoke<FansData>("fetch_fans", {
      cookie: cookie.value,
      page: currentPage.value,
      uid: uid.value,
    });
    fansData.value = data;
    loading.value = false;
  } catch (err) {
    console.error("请求出错:", err);
    ElMessage.error("加载粉丝数据失败，请检查UID和Cookie是否正确");
    loading.value = false;
  }
}

// 选择/取消选择所有粉丝 - 修改为使用筛选后的数据
function toggleSelectAll(): void {
  if (selectAll.value) {
    // 全选
    filteredFans.value.forEach((user) => {
      selectedFans.add(user.id);
    });
  } else {
    // 取消全选
    selectedFans.clear();
  }
}

// 切换单个粉丝的选择状态 - 修改为使用筛选后的数据
function toggleSelect(userId: string | number): void {
  if (selectedFans.has(userId)) {
    selectedFans.delete(userId);
    selectAll.value = false;
  } else {
    selectedFans.add(userId);
    // 检查是否已全选
    if (selectedFans.size === filteredFans.value.length) {
      selectAll.value = true;
    }
  }
}

// 打开私信模态框 - 修改为使用筛选后的数据
function openMessageModal(): void {
  if (selectedFans.size === 0) {
    ElMessage.warning("请至少选择一位粉丝");
    return;
  }

  messageTargets.value = filteredFans.value.filter((user) =>
    selectedFans.has(user.id)
  );
  showMessageModal.value = true;
}

// 打开评论模态框 - 修改为使用筛选后的数据
function openCommentModal(): void {
  if (selectedFans.size === 0) {
    ElMessage.warning("请至少选择一位粉丝");
    return;
  }

  commentTargets.value = filteredFans.value.filter((user) =>
    selectedFans.has(user.id)
  );
  showCommentModal.value = true;
}

// 切换顶部查询区域的折叠状态
function toggleParamsCard(): void {
  paramsCardCollapsed.value = !paramsCardCollapsed.value;
}

// 返回首页
function goBack(): void {
  router.push("/");
}

// 计算选中的粉丝数量
const selectedCount = computed(() => selectedFans.size);

// 初始加载
loadFans();

// 组件挂载时加载历史状态
onMounted(() => {
  loadHistoryStatus();
});

// 监听私信和评论成功事件，更新本地状态
function onInteractionSuccess(type: 'message' | 'comment', users: WeiboUser[], content: string): void {
  users.forEach(user => {
    if (type === 'message') {
      LocalStorage.addMessageRecord(user.id, user.screen_name, user.profile_image_url, content);
      messagedUsers.value.add(user.id);
    } else {
      LocalStorage.addCommentRecord(user.id, user.screen_name, user.profile_image_url, content);
      commentedUsers.value.add(user.id);
    }
  });
}
</script>

<template>
  <main class="container">
    <el-container>
      <el-header height="auto" class="fixed-header">
        <!-- 移除原来的返回按钮和历史页面导航按钮 -->
        <div class="header">
          <h2 class="page-title">
            <el-icon><user /></el-icon>
            粉丝管理
          </h2>
        </div>

        <!-- 参数设置卡片组件 -->
        <ParamsCard
          v-model:uid="inputUId"
          v-model:cookie="cookie"
          v-model:collapsed="paramsCardCollapsed"
          v-model:gender="selectedGender"
          :loading="loading"
          @refresh="loadFans"
          @toggle="toggleParamsCard"
        />

        <!-- 操作栏组件 -->
        <ActionBar
          v-model:select-all="selectAll"
          :selected-count="selectedCount"
          @toggle-select-all="toggleSelectAll"
          @select-all-unmessaged="selectAllUnmessagedUsers"
          @select-all-uncommented="selectAllUncommentedUsers"
          @open-message-modal="openMessageModal"
          @open-comment-modal="openCommentModal"
          @edit-common-messages="openEditMessagesModal"
        />
      </el-header>

      <!-- 粉丝列表 -->
      <el-main class="scrollable-content">
        <el-skeleton :rows="10" animated v-if="loading" />

        <template v-else-if="filteredFans && filteredFans.length > 0">
          <FanCard
            v-for="user in filteredFans"
            :key="user.id"
            :user="user"
            :selected="selectedFans.has(user.id)"
            :is-messaged="isUserMessaged(user.id)"
            :is-commented="isUserCommented(user.id)"
            @toggle-select="toggleSelect"
          />
        </template>

        <el-empty v-else description="暂无符合条件的粉丝数据">
          <el-button type="primary" @click="loadFans">重新加载</el-button>
        </el-empty>
      </el-main>

      <!-- 固定底部 -->
      <el-footer class="fixed-footer">
        <!-- 分页控件 -->
        <el-pagination
          background
          layout="prev, pager, next"
          :current-page="currentPage"
          :total="1000000"
          @current-change="
            (page) => {
              currentPage = page;
              loadFans();
            }
          "
        />
      </el-footer>
    </el-container>

    <!-- 私信模态框组件 -->
    <InteractionModal
      v-model:visible="showMessageModal"
      :targets="messageTargets"
      :cookie="cookie"
      :commonMessages="commonMessages"
      type="message"
      @remove-target="(userId) => selectedFans.delete(userId)"
      @interaction-success="(users, content) => onInteractionSuccess('message', users, content)"
    />

    <InteractionModal
      v-model:visible="showCommentModal"
      :targets="commentTargets"
      :cookie="cookie"
      :commonMessages="commonMessages"
      type="comment"
      @remove-target="(userId) => selectedFans.delete(userId)"
      @interaction-success="(users, content) => onInteractionSuccess('comment', users, content)"
    />

    <!-- 常用语编辑弹窗保持不变 -->
    <EditCommonMessagesModal
      v-model:visible="showEditMessagesModal"
      v-model:messages="commonMessages"
      title="常用语"
    />
  </main>
</template>

<style scoped>
/* 基础样式 */
.container {
  max-width: 1200px;
  margin: 0 auto;
  height: 100vh;
  overflow: hidden;
}

/* Element Plus 容器样式 */
.el-container {
  height: 100%;
}

/* 固定顶部 */
.fixed-header {
  padding: 20px 20px 0;
  background-color: #fff;
  z-index: 10;
}

.header {
  display: flex;
  align-items: center;
  margin-bottom: 20px;
}

.page-title {
  margin: 0;
  display: flex;
  align-items: center;
  gap: 8px;
  color: #333;
  font-size: 1.5rem;
}

/* 移除 back-btn 和 history-nav 相关样式 */
</style>
