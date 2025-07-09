<script setup lang="ts">
import { ref, reactive, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { apiCookie } from "../contants";
import { useRouter } from "vue-router";
import { ElLoading, ElMessage, ElMessageBox } from "element-plus";

// 导入拆分出的组件
import ParamsCard from "../components/fans/ParamsCard.vue";
import ActionBar from "../components/fans/ActionBar.vue";
import FanCard from "../components/fans/FanCard.vue";
import MessageModal from "../components/fans/MessageModal.vue";
import CommentModal from "../components/fans/CommentModal.vue";
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
  [key: string]: any; // 允许其他可能的属性
}

// 定义粉丝数据接口
interface FansData {
  users: WeiboUser[];
  [key: string]: any; // 允许其他可能的属性
}

// 导出接口供其他组件使用
export type { WeiboUser, FansData };

const router = useRouter();

// 粉丝数据状态管理
const fansData = ref<FansData>({ users: [] });
const loading = ref<boolean>(false);
const currentPage = ref<number>(1);
const uid = ref<string>("6005682439");
const cookie = ref<string>(apiCookie); // 添加 cookie 变量，默认使用 apiCookie

// 统一的常用语列表
const commonMessages = ref<{id: number, content: string}[]>([
  { id: 1, content: "感谢关注，我会持续更新优质内容！" },
  { id: 2, content: "新内容已更新，欢迎查看！" },
  { id: 3, content: "有任何问题都可以随时联系我哦~" },
]);

// 选择状态管理
const selectedFans = reactive<Set<string | number>>(new Set());
const selectAll = ref<boolean>(false);

// 私信相关
const showMessageModal = ref<boolean>(false);
const messageTargets = ref<WeiboUser[]>([]);

// 评论相关
const showCommentModal = ref<boolean>(false);
const commentTargets = ref<WeiboUser[]>([]);

// 常用语编辑弹窗 - 只保留一个
const showEditMessagesModal = ref<boolean>(false);

// 顶部查询区域折叠状态
const paramsCardCollapsed = ref<boolean>(false);

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

// 选择/取消选择所有粉丝
function toggleSelectAll(): void {
  if (selectAll.value) {
    // 全选
    fansData.value.users.forEach((user) => {
      selectedFans.add(user.id);
    });
  } else {
    // 取消全选
    selectedFans.clear();
  }
}

// 切换单个粉丝的选择状态
function toggleSelect(userId: string | number): void {
  if (selectedFans.has(userId)) {
    selectedFans.delete(userId);
    selectAll.value = false;
  } else {
    selectedFans.add(userId);
    // 检查是否已全选
    if (selectedFans.size === fansData.value.users.length) {
      selectAll.value = true;
    }
  }
}

// 打开私信模态框
function openMessageModal(): void {
  if (selectedFans.size === 0) {
    ElMessage.warning("请至少选择一位粉丝");
    return;
  }

  messageTargets.value = fansData.value.users.filter((user) =>
    selectedFans.has(user.id)
  );
  showMessageModal.value = true;
}

// 打开评论模态框
function openCommentModal(): void {
  if (selectedFans.size === 0) {
    ElMessage.warning("请至少选择一位粉丝");
    return;
  }

  commentTargets.value = fansData.value.users.filter((user) =>
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
</script>

<template>
  <main class="container">
    <el-container>
      <el-header height="auto" class="fixed-header">
        <div class="header">
          <el-button class="back-btn" @click="goBack" text>
            <el-icon><arrow-left /></el-icon> 返回首页
          </el-button>
        </div>

        <!-- 参数设置卡片组件 -->
        <ParamsCard
          v-model:uid="uid"
          v-model:cookie="cookie"
          v-model:collapsed="paramsCardCollapsed"
          :loading="loading"
          @refresh="loadFans"
          @toggle="toggleParamsCard"
        />

        <!-- 操作栏组件 -->
        <ActionBar
          v-model:select-all="selectAll"
          :selected-count="selectedCount"
          @toggle-select-all="toggleSelectAll"
          @open-message-modal="openMessageModal"
          @open-comment-modal="openCommentModal"
          @edit-common-messages="openEditMessagesModal"
        />
      </el-header>

      <!-- 粉丝列表 -->
      <el-main class="scrollable-content">
        <el-skeleton :rows="10" animated v-if="loading" />

        <template v-else-if="fansData.users && fansData.users.length > 0">
          <FanCard
            v-for="user in fansData.users"
            :key="user.id"
            :user="user"
            :selected="selectedFans.has(user.id)"
            @toggle-select="toggleSelect"
          />
        </template>

        <el-empty v-else description="暂无粉丝数据">
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
          :total="100"
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
    <MessageModal
      v-model:visible="showMessageModal"
      :targets="messageTargets"
      :cookie="cookie"
      :commonMessages="commonMessages"
      @remove-target="(userId) => selectedFans.delete(userId)"
    />

    <!-- 评论模态框组件 -->
    <CommentModal
      v-model:visible="showCommentModal"
      :targets="commentTargets"
      :cookie="cookie"
      :commonMessages="commonMessages"
      @remove-target="(userId) => selectedFans.delete(userId)"
    />
      
    <!-- 只保留一个编辑常用语弹窗 -->
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
  position: relative;
}

.back-btn {
  position: absolute;
  left: 0;
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 16px;
  color: #666;
}

/* 可滚动内容区域 */
.scrollable-content {
  overflow-y: auto;
  padding: 0 20px;
  background-color: #f5f7fa;
}

/* 固定底部 */
.fixed-footer {
  display: flex;
  justify-content: center;
  padding: 20px;
  background-color: #fff;
  box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.05);
}
</style>