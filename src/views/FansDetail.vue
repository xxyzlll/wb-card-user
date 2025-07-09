<script setup lang="ts">
import { ref, reactive, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { apiCookie } from "../contants";
import { useRouter } from "vue-router";
import { ElLoading, ElMessage, ElMessageBox } from "element-plus";

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

// 定义常用私信内容接口
interface CommonMessage {
  id: number;
  content: string;
}

const router = useRouter();

// 粉丝数据状态管理
const fansData = ref<FansData>({ users: [] });
const loading = ref<boolean>(false);
const currentPage = ref<number>(1);
const uid = ref<string>("6005682439");
const cookie = ref<string>(apiCookie); // 添加 cookie 变量，默认使用 apiCookie

// 选择状态管理
const selectedFans = reactive<Set<string | number>>(new Set());
const selectAll = ref<boolean>(false);

// 私信相关
const messageContent = ref<string>("");
const showMessageModal = ref<boolean>(false);
const messageTargets = ref<WeiboUser[]>([]);

// 常用私信内容管理
const commonMessages = ref<CommonMessage[]>([
  { id: 1, content: "感谢关注，我会持续更新优质内容！" },
  { id: 2, content: "新内容已更新，欢迎查看！" },
  { id: 3, content: "有任何问题都可以随时私信我哦~" },
]);
const newMessageContent = ref<string>("");
const editingMessageId = ref<number | null>(null);
const editingContent = ref<string>("");

// 顶部查询区域折叠状态
const paramsCardCollapsed = ref<boolean>(false);

// 加载粉丝数据
async function loadFans(): Promise<void> {
  try {
    loading.value = true;
    const data = await invoke<FansData>("fetch_fans", {
      cookie: cookie.value, // 使用输入框中的 cookie
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

// 发送私信
function sendMessage(): void {
  if (!messageContent.value.trim()) {
    ElMessage.warning("请输入私信内容");
    return;
  }

  // 显示加载状态
  const loading = ElLoading.service({
    lock: true,
    text: '正在发送私信...',
    background: 'rgba(0, 0, 0, 0.7)'
  });

  // 调用后端API发送私信
  invoke("send_direct_message", {
    cookie: cookie.value, // 使用实际的cookie值
    text: messageContent.value, // 使用实际的私信内容
    uid: messageTargets.value[0].id.toString(), // 使用选中的第一个粉丝ID
    source: "209678993", // 应用ID
  })
    .then((response) => {
      console.log("私信发送成功:", response);
      ElMessage.success("私信发送成功");
      // 重置状态
      messageContent.value = "";
      showMessageModal.value = false;
      loading.close();
    })
    .catch((error) => {
      console.error("私信发送失败:", error);
      ElMessage.error(`私信发送失败: ${error}`);
      loading.close();
    });
}

// 使用常用私信内容
function useCommonMessage(content: string): void {
  messageContent.value = content;
}

// 添加常用私信内容
function addCommonMessage(): void {
  if (!newMessageContent.value.trim()) {
    ElMessage.warning("请输入常用私信内容");
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

// 删除常用私信内容
function deleteCommonMessage(id: number): void {
  ElMessageBox.confirm("确定要删除这条常用私信内容吗？", "提示", {
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

// 开始编辑常用私信内容
function startEditMessage(message: CommonMessage): void {
  editingMessageId.value = message.id;
  editingContent.value = message.content;
}

// 保存编辑后的常用私信内容
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
          <h1>微博粉丝管理</h1>
        </div>

        <!-- 参数设置区域 - 使用Element Plus重构 -->
        <el-card class="params-card" :body-style="{ padding: '0' }">
          <template #header>
            <div class="params-header" @click="toggleParamsCard">
              <el-icon><setting /></el-icon>
              <span>查询设置</span>
              <el-icon
                class="collapse-icon"
                :class="{ 'is-collapsed': paramsCardCollapsed }"
              >
                <arrow-down />
              </el-icon>
            </div>
          </template>

          <div class="params-body" v-show="!paramsCardCollapsed">
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="博主UID">
                  <el-input
                    v-model="uid"
                    placeholder="请输入博主UID"
                    prefix-icon="User"
                  />
                </el-form-item>
              </el-col>

              <el-col :span="12">
                <el-form-item label="Cookie">
                  <el-input
                    v-model="cookie"
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
              @click="loadFans"
              :loading="loading"
              class="refresh-btn"
            >
              <el-icon><refresh /></el-icon>
              刷新数据
            </el-button>
          </div>
        </el-card>

        <!-- 操作栏 -->
        <div class="action-bar">
          <div class="selection-info">
            <el-checkbox v-model="selectAll" @change="toggleSelectAll">
              全选
            </el-checkbox>
            <el-tag v-if="selectedCount > 0" type="warning" effect="dark">
              已选择 {{ selectedCount }} 位粉丝
            </el-tag>
          </div>

          <el-button
            type="primary"
            @click="openMessageModal"
            :disabled="selectedCount === 0"
            color="#ff9800"
          >
            <el-icon><message /></el-icon> 发送私信
          </el-button>
        </div>
      </el-header>

      <!-- 粉丝列表 -->
      <el-main class="scrollable-content">
        <el-skeleton :rows="10" animated v-if="loading" />

        <template v-else-if="fansData.users && fansData.users.length > 0">
          <el-card
            v-for="user in fansData.users"
            :key="user.id"
            class="fan-card"
            shadow="hover"
          >
            <div class="fan-card-content">
              <div class="select-area">
                <el-checkbox
                  :model-value="selectedFans.has(user.id)"
                  @change="toggleSelect(user.id)"
                />
              </div>

              <el-avatar
                :size="60"
                :src="user.profile_image_url"
                class="avatar"
              />

              <div class="user-info">
                <h3 class="username">{{ user.screen_name }}</h3>
                <p class="bio">
                  {{ user.description || "这个人很懒，什么都没写" }}
                </p>
                <div class="stats">
                  <el-tag size="small" effect="plain" type="danger"
                    >粉丝: {{ user.followers_count }}</el-tag
                  >
                  <el-tag size="small" effect="plain" type="success"
                    >关注: {{ user.friends_count }}</el-tag
                  >
                  <el-tag size="small" effect="plain" type="info"
                    >微博: {{ user.statuses_count }}</el-tag
                  >
                </div>
              </div>

              <div class="actions">
                <el-button
                  type="primary"
                  plain
                  size="small"
                  @click="
                    toggleSelect(user.id);
                    openMessageModal();
                  "
                  color="#ff9800"
                >
                  <el-icon><message /></el-icon> 私信
                </el-button>
              </div>
            </div>
          </el-card>
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

    <!-- 私信模态框 -->
    <el-dialog
      v-model="showMessageModal"
      title="发送私信"
      width="600px"
      destroy-on-close
    >
      <el-form>
        <el-form-item label="收信人:">
          <div class="recipient-list">
            <el-tag
              v-for="user in messageTargets"
              :key="user.id"
              closable
              @close="
                selectedFans.delete(user.id);
                messageTargets = messageTargets.filter((u) => u.id !== user.id);
              "
              type="warning"
              effect="dark"
            >
              {{ user.screen_name }}
            </el-tag>
          </div>
        </el-form-item>

        <el-form-item label="常用私信:">
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
                  @click="useCommonMessage(message.content)"
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

            <el-divider content-position="center">添加常用私信</el-divider>

            <div class="add-common-message">
              <el-input
                v-model="newMessageContent"
                type="textarea"
                :rows="2"
                placeholder="输入新的常用私信内容..."
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
          <el-button @click="showMessageModal = false">取消</el-button>
          <el-button type="primary" @click="sendMessage" color="#ff9800"
            >发送</el-button
          >
        </span>
      </template>
    </el-dialog>
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

h1 {
  text-align: center;
  margin: 0 auto;
  color: #333;
}

/* 参数设置卡片样式 - Element Plus 风格 */
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

/* 粉丝卡片样式 */
.fan-card {
  margin-bottom: 15px;
  border-radius: 8px;
  overflow: hidden;
}

.fan-card-content {
  display: flex;
  align-items: center;
}

.select-area {
  margin-right: 15px;
}

.avatar {
  margin-right: 20px;
  border: 3px solid #f0f2f5;
}

.user-info {
  flex: 1;
}

.username {
  margin: 0 0 5px 0;
  font-size: 18px;
  color: #333;
}

.bio {
  margin: 0 0 10px 0;
  color: #666;
  font-size: 14px;
  line-height: 1.4;
}

.stats {
  display: flex;
  gap: 10px;
}

/* 私信模态框样式 */
.recipient-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 10px;
}

/* 常用私信内容样式 */
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
