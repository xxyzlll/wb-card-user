// 本地存储工具类
export interface UserHistoryRecord {
  userId: string | number;
  userName: string;
  userAvatar: string;
  timestamp: number;
  content: string;
}

export class LocalStorage {
  private static readonly MESSAGE_HISTORY_KEY = 'weibo_message_history';
  private static readonly COMMENT_HISTORY_KEY = 'weibo_comment_history';
  private static readonly MESSAGED_USERS_KEY = 'weibo_messaged_users';
  private static readonly COMMENTED_USERS_KEY = 'weibo_commented_users';
  // 新增：评论失败用户ID列表
  private static readonly COMMENT_FAILED_USERS_KEY = 'weibo_comment_failed_users';

  // 获取已私信用户ID列表
  static getMessagedUsers(): Set<string | number> {
    const data = localStorage.getItem(this.MESSAGED_USERS_KEY);
    return new Set(data ? JSON.parse(data) : []);
  }

  // 获取已评论用户ID列表
  static getCommentedUsers(): Set<string | number> {
    const data = localStorage.getItem(this.COMMENTED_USERS_KEY);
    return new Set(data ? JSON.parse(data) : []);
  }

  // 新增：获取评论失败用户ID列表
  static getCommentFailedUsers(): Set<string | number> {
    const data = localStorage.getItem(this.COMMENT_FAILED_USERS_KEY);
    return new Set(data ? JSON.parse(data) : []);
  }

  // 新增：添加评论失败用户
  static addCommentFailedUser(userId: string | number): void {
    const failedUsers = this.getCommentFailedUsers();
    failedUsers.add(userId);
    localStorage.setItem(this.COMMENT_FAILED_USERS_KEY, JSON.stringify([...failedUsers]));
  }

  // 新增：移除评论失败用户（当重新评论成功时）
  static removeCommentFailedUser(userId: string | number): void {
    const failedUsers = this.getCommentFailedUsers();
    failedUsers.delete(userId);
    localStorage.setItem(this.COMMENT_FAILED_USERS_KEY, JSON.stringify([...failedUsers]));
  }

  // 添加私信记录
  static addMessageRecord(userId: string | number, userName: string, userAvatar: string, content: string): void {
    // 添加到已私信用户列表
    const messagedUsers = this.getMessagedUsers();
    messagedUsers.add(userId);
    localStorage.setItem(this.MESSAGED_USERS_KEY, JSON.stringify([...messagedUsers]));

    // 添加到私信历史记录
    const history = this.getMessageHistory();
    const record: UserHistoryRecord = {
      userId,
      userName,
      userAvatar,
      timestamp: Date.now(),
      content
    };
    history.unshift(record); // 最新记录在前
    
    // 限制历史记录数量（最多保存1000条）
    if (history.length > 1000) {
      history.splice(1000);
    }
    
    localStorage.setItem(this.MESSAGE_HISTORY_KEY, JSON.stringify(history));
  }

  // 修改：添加评论记录时移除失败状态
  static addCommentRecord(userId: string | number, userName: string, userAvatar: string, content: string): void {
    // 移除评论失败状态（如果存在）
    this.removeCommentFailedUser(userId);
    
    // 添加到已评论用户列表
    const commentedUsers = this.getCommentedUsers();
    commentedUsers.add(userId);
    localStorage.setItem(this.COMMENTED_USERS_KEY, JSON.stringify([...commentedUsers]));

    // 添加到评论历史记录
    const history = this.getCommentHistory();
    const record: UserHistoryRecord = {
      userId,
      userName,
      userAvatar,
      timestamp: Date.now(),
      content
    };
    history.unshift(record); // 最新记录在前
    
    // 限制历史记录数量（最多保存1000条）
    if (history.length > 1000) {
      history.splice(1000);
    }
    
    localStorage.setItem(this.COMMENT_HISTORY_KEY, JSON.stringify(history));
  }

  // 获取私信历史记录
  static getMessageHistory(): UserHistoryRecord[] {
    const data = localStorage.getItem(this.MESSAGE_HISTORY_KEY);
    return data ? JSON.parse(data) : [];
  }

  // 获取评论历史记录
  static getCommentHistory(): UserHistoryRecord[] {
    const data = localStorage.getItem(this.COMMENT_HISTORY_KEY);
    return data ? JSON.parse(data) : [];
  }

  // 清空私信历史
  static clearMessageHistory(): void {
    localStorage.removeItem(this.MESSAGE_HISTORY_KEY);
    localStorage.removeItem(this.MESSAGED_USERS_KEY);
  }

  // 修改：清空评论历史时也清空失败状态
  static clearCommentHistory(): void {
    localStorage.removeItem(this.COMMENT_HISTORY_KEY);
    localStorage.removeItem(this.COMMENTED_USERS_KEY);
    localStorage.removeItem(this.COMMENT_FAILED_USERS_KEY);
  }

  // 修改：删除单条评论记录时也移除失败状态
  static removeCommentRecord(userId: string | number): void {
    const commentedUsers = this.getCommentedUsers();
    commentedUsers.delete(userId);
    localStorage.setItem(this.COMMENTED_USERS_KEY, JSON.stringify([...commentedUsers]));

    // 移除评论失败状态
    this.removeCommentFailedUser(userId);

    const history = this.getCommentHistory().filter(record => record.userId !== userId);
    localStorage.setItem(this.COMMENT_HISTORY_KEY, JSON.stringify(history));
  }
}